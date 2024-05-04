use futures::StreamExt;
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use std::{io, str::FromStr, time::Duration};
use tokio::time;

use crate::{types::BatteryStatus, AppMessage};

use super::service::BatteryService;
use tracing::{error, info};

pub struct BatteryServiceHandle {
    app_channel: Sender<AppMessage>,
}

impl BatteryServiceHandle {
    pub fn new(app_channel: Sender<AppMessage>) -> Self {
        Self { app_channel }
    }

    pub async fn run(&mut self) {
        let mut stream_res = BatteryService::get_notification_stream().await;

        if let Err(e) = stream_res.as_ref() {
            error!("error while getting battery stream {}", e);
            let _ = self.app_channel.send(AppMessage::Battery {
                level: 0,
                status: BatteryStatus::Unknown,
            });
            return;
        }

        while let Some(signal) = stream_res.as_mut().unwrap().next().await {
            if let Ok(args) = signal.args() {
                let notification_event = args.event;
                // let _ = self.app_channel.send(AppMessage::Battery {
                //     level: capacity,
                //     status: BatteryStatus::from_str(status.as_ref()).unwrap(),
                // });
            }
        }
    }
}
