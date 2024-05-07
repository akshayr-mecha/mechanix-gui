use futures::StreamExt;
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use std::time::Duration;
use tokio::time;

use super::service::BluetoothService;
use crate::{types::BluetoothStatus, AppMessage};
use tracing::error;

pub struct BluetoothServiceHandle {
    app_channel: Sender<AppMessage>,
}

impl BluetoothServiceHandle {
    pub fn new(app_channel: Sender<AppMessage>) -> Self {
        Self { app_channel }
    }

    pub async fn run(&mut self) {
        let task = "run";
        let mut stream_res = BluetoothService::get_notification_stream().await;

        if let Err(e) = stream_res.as_ref() {
            error!(task, "error while getting bluetooth stream {}", e);
            let _ = self.app_channel.send(AppMessage::Bluetooth {
                status: BluetoothStatus::NotFound,
            });
            return;
        }

        while let Some(signal) = stream_res.as_mut().unwrap().next().await {
            if let Ok(args) = signal.args() {
                let notification_event = args.event;
                let _ = self.app_channel.send(AppMessage::Bluetooth {
                    status: BluetoothStatus::NotFound,
                });
            }
        }
    }
}
