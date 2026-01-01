use crate::handlers::price_update_handler::handle_price_update;
use crate::models::enums::Platform;
use crate::models::kafka_event::KEvent;
use futures::StreamExt;
use log::{error, info, warn};
use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;
use tokio_util::sync::CancellationToken;
use yellowstone_grpc_client::GeyserGrpcClient;
use yellowstone_grpc_proto::geyser::{SubscribeRequest, SubscribeRequestFilterAccounts};

const MAX_BACKOFF: u64 = 30;

pub struct AccountConsumer {
    pub geyser_url: String,
    pub geyser_token: Option<String>,
    pub event_tx: Sender<KEvent>,
    pub tracked_accounts: HashMap<String, Platform>,
}

impl AccountConsumer {
    // pub async fn update_subscriptions(&self, new_subscriptions: HashMap<String, Platform>) {
    //     let mut subs = self.subscriptions.lock().await;
    //     *subs = new_subscriptions;
    // }
    //
    // pub async fn get_platform(&self, key: &str) -> Option<Platform> {
    //     let subs = self.subscriptions.lock().await;
    //     subs.get(key).cloned()
    // }

    pub fn spawn(
        self,
        cancel_token: CancellationToken,
    ) -> (tokio::task::JoinHandle<()>, oneshot::Receiver<()>) {
        let (stability_tx, stability_rx) = oneshot::channel();
        let handle = tokio::spawn(async move {
            self.run(cancel_token, Some(stability_tx)).await;
        });
        (handle, stability_rx)
    }

    async fn run(
        self,
        cancel_token: CancellationToken,
        mut stability_tx: Option<oneshot::Sender<()>>,
    ) {
        let mut backoff = 1;
        let mut stability_sent = false;

        loop {
            if cancel_token.is_cancelled() {
                info!("AccountConsumer cancelled");
                break;
            }

            match GeyserGrpcClient::build_from_shared(self.geyser_url.clone()).and_then(|builder| {
                let builder = if let Some(token) = &self.geyser_token {
                    builder.x_token(Some(token.clone()))?
                } else {
                    builder
                };
                Ok(builder.connect())
            }) {
                Ok(fut) => match fut.await {
                    Ok(mut client) => {
                        info!("Connected to Geyser Account stream");
                        backoff = 1;

                        let tracked_accounts = self.tracked_accounts.clone();
                        let items: Vec<String> = tracked_accounts.keys().cloned().collect();

                        let mut account_filters = std::collections::HashMap::new();
                        account_filters.insert(
                            "tracked-accounts".to_string(),
                            SubscribeRequestFilterAccounts {
                                account: items,
                                ..Default::default()
                            },
                        );

                        let request = SubscribeRequest {
                            accounts: account_filters,
                            ..Default::default()
                        };

                        match client.subscribe_once(request).await {
                            Ok(mut stream) => {
                                if !stability_sent {
                                    if let Some(tx) = stability_tx.take() {
                                        let _ = tx.send(());
                                        info!("Stability reached - subscription active");
                                    }
                                    stability_sent = true;
                                }

                                loop {
                                    tokio::select! {
                                        _ = cancel_token.cancelled() => {
                                            info!("Stream cancelled, shutting down");
                                            break;
                                        }
                                        msg = stream.next() => {
                                            match msg {
                                                Some(Ok(update)) => {

                                                    if let Some(oneof) = update.update_oneof {

                                                        if let yellowstone_grpc_proto::geyser::subscribe_update::UpdateOneof::Account(acc_update) = oneof {

                                                            if let Some(info) = acc_update.account {

                                                                match handle_price_update(&info, tracked_accounts.clone()).await {
                                                                    Ok(kevent) => {
                                                                        if let Err(e) = self.event_tx.send(kevent).await {
                                                                            error!("Failed to connect Geyser client: {e}");
                                                                        }
                                                                    },
                                                                    Err(e) => {
                                                                        error!("Failed to handle_price_update: {e}");
                                                                    }
                                                                }
                                                            } else {
                                                                warn!("Invalid info");
                                                            }
                                                        } else {
                                                            warn!("Invalid update");
                                                        }
                                                    } else {
                                                        warn!("Invalid oneof");
                                                    }
                                                }
                                                Some(Err(e)) => {
                                                    warn!("Stream error: {e:?}");
                                                },
                                                None => {
                                                    info!("Stream ended");
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                warn!("Failed to subscribe to account updates: {e}");
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to connect Geyser client: {e}");
                    }
                },
                Err(e) => {
                    error!("Invalid Geyser URL: {e}");
                }
            }

            if cancel_token.is_cancelled() {
                error!("is_cancelled");
                break;
            }

            warn!("Reconnecting in {backoff}s...");
            tokio::time::sleep(Duration::from_secs(backoff)).await;
            backoff = (backoff * 2).min(MAX_BACKOFF);
        }
    }
}
