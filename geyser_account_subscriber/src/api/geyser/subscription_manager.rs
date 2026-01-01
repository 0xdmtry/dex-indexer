use crate::api::geyser::account_consumer::AccountConsumer;
use crate::models::enums::Platform;
use crate::models::kafka_event::KEvent;
use log::info;
use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::{mpsc, oneshot};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

pub enum SubscriptionCommand {
    Update {
        tracked_accounts: HashMap<String, Platform>,
        response: oneshot::Sender<Result<(), String>>,
    },
    Shutdown,
}

pub struct SubscriptionManager {
    geyser_url: String,
    geyser_token: Option<String>,
    event_tx: mpsc::Sender<KEvent>,
    current_task: Option<(JoinHandle<()>, CancellationToken)>,
    command_rx: mpsc::Receiver<SubscriptionCommand>,
}

impl SubscriptionManager {
    pub fn spawn(
        geyser_url: String,
        geyser_token: Option<String>,
        event_tx: mpsc::Sender<KEvent>,
    ) -> (JoinHandle<()>, SubscriptionManagerHandle) {
        let (cmd_tx, cmd_rx) = mpsc::channel(32);

        let mut manager = Self {
            geyser_url,
            geyser_token,
            event_tx,
            current_task: None,
            command_rx: cmd_rx,
        };

        let handle = tokio::spawn(async move {
            manager.run().await;
        });

        let manager_handle = SubscriptionManagerHandle { cmd_tx };

        (handle, manager_handle)
    }

    async fn run(&mut self) {
        while let Some(cmd) = self.command_rx.recv().await {
            match cmd {
                SubscriptionCommand::Update {
                    tracked_accounts,
                    response,
                } => {
                    let result = self.update_subscription(tracked_accounts).await;
                    let _ = response.send(result);
                }
                SubscriptionCommand::Shutdown => {
                    self.shutdown().await;
                    break;
                }
            }
        }
    }

    async fn update_subscription(
        &mut self,
        tracked_accounts: HashMap<String, Platform>,
    ) -> Result<(), String> {
        info!(
            "Starting new subscription with {} accounts",
            tracked_accounts.len()
        );

        let new_cancel_token = CancellationToken::new();
        let consumer = AccountConsumer {
            geyser_url: self.geyser_url.clone(),
            geyser_token: self.geyser_token.clone(),
            event_tx: self.event_tx.clone(),
            tracked_accounts,
        };

        let (new_handle, stability_rx) = consumer.spawn(new_cancel_token.clone());

        match tokio::time::timeout(Duration::from_secs(10), stability_rx).await {
            Ok(_) => {
                info!("New subscription stable, shutting down old subscription");

                if let Some((old_handle, old_token)) = self.current_task.take() {
                    old_token.cancel();
                    let _ = old_handle.await;
                }

                self.current_task = Some((new_handle, new_cancel_token));
                info!("Subscription swap complete");
                Ok(())
            }
            // TODO re-review the error handling. Make it more insigthful
            Err(_) => {
                info!("New subscription failed to stabilize, cancelling");
                new_cancel_token.cancel();
                let _ = new_handle.await;
                Err("Failed to reach stability".to_string())
            }
        }
    }

    async fn shutdown(&mut self) {
        if let Some((handle, token)) = self.current_task.take() {
            info!("Shutting down subscription");
            token.cancel();
            let _ = handle.await;
        }
    }
}

#[derive(Clone)]
pub struct SubscriptionManagerHandle {
    cmd_tx: mpsc::Sender<SubscriptionCommand>,
}

impl SubscriptionManagerHandle {
    pub async fn update_subscription(
        &self,
        tracked_accounts: HashMap<String, Platform>,
    ) -> Result<(), String> {
        let (response_tx, response_rx) = oneshot::channel();

        self.cmd_tx
            .send(SubscriptionCommand::Update {
                tracked_accounts,
                response: response_tx,
            })
            .await
            .map_err(|_| {
                "SubscriptionManagerHandle::update_subscription::ERROR::Manager disconnected"
                    .to_string()
            })?;

        response_rx.await.map_err(|_| {
            "SubscriptionManagerHandle::update_subscription::ERROR::Response channel closed"
                .to_string()
        })?
    }

    pub async fn shutdown(&self) -> Result<(), String> {
        self.cmd_tx
            .send(SubscriptionCommand::Shutdown)
            .await
            .map_err(|_| {
                "PUMP::ACCOUNT::SubscriptionManagerHandle::shutdown::ERROR::Manager disconnected"
                    .to_string()
            })
    }
}
