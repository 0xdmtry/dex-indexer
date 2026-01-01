use crate::handlers::tx_handler::handle_tx;
use crate::models::consts::PUMPFUN_PROGRAM_ID;
use crate::models::kafka_event::KEvent;
use futures::StreamExt;
use log::{error, info, warn};
use std::time::Duration;
use tokio::sync::mpsc::Sender;
use yellowstone_grpc_client::GeyserGrpcClient;
use yellowstone_grpc_proto::geyser::subscribe_update::UpdateOneof::Transaction;
use yellowstone_grpc_proto::geyser::{SubscribeRequest, SubscribeRequestFilterTransactions};

const MAX_BACKOFF: u64 = 30;

pub struct TxConsumer {
    pub geyser_url: String,
    pub geyser_token: Option<String>,
    pub event_tx: Sender<KEvent>,
}

impl TxConsumer {
    pub async fn start(self) {
        let mut backoff = 1;
        loop {
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
                        info!("Connected to Geyser Tx stream");

                        backoff = 1;

                        let mut tx_filters = std::collections::HashMap::new();
                        tx_filters.insert(
                            "all-protocols".to_string(),
                            SubscribeRequestFilterTransactions {
                                account_include: vec![
                                    PUMPFUN_PROGRAM_ID.to_string(),
                                    // PUMPSWAP_PROGRAM_ID.to_string(),
                                    // RAYDIUM_LAUNCHLAB_PROGRAM_ID.to_string(),
                                ],
                                vote: Some(false),
                                failed: Some(false),
                                ..Default::default()
                            },
                        );

                        let request = SubscribeRequest {
                            transactions: tx_filters,
                            ..Default::default()
                        };

                        match client.subscribe_once(request).await {
                            Ok(mut stream) => {
                                while let Some(msg) = stream.next().await {
                                    match msg {
                                        Ok(update) => {
                                            if let Some(Transaction(tx_update)) =
                                                update.update_oneof
                                            {
                                                let slot = tx_update.slot;
                                                if let Some(tx_info) = tx_update.transaction {
                                                    match handle_tx(tx_info, slot) {
                                                        Ok(kevent) => match kevent {
                                                            Some(ke) => {
                                                                self.event_tx
                                                                    .send(ke)
                                                                    .await
                                                                    .unwrap();
                                                            }
                                                            None => {
                                                                warn!("Invalid event")
                                                            }
                                                        },
                                                        Err(e) => {
                                                            warn!("Failed to handle tx: {e}")
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            warn!("Stream error: {e:?}");
                                            break;
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                warn!("Failed to subscribe: {e}");
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

            warn!("Reconnecting in {backoff}s...");
            tokio::time::sleep(Duration::from_secs(backoff)).await;
            backoff = (backoff * 2).min(MAX_BACKOFF);
        }
    }
}
