use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use std::time::Duration;
use tokio::{sync::oneshot, time};

use tracing::error;

use crate::gui::Message;

use super::service::SoundService;

#[derive(Debug)]
pub enum ServiceMessage {
    Start { respond_to: oneshot::Sender<u32> },
    Stop { respond_to: oneshot::Sender<u32> },
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ServiceStatus {
    INACTIVE = 0,
    STARTED = 1,
    STOPPED = -1,
}

pub struct SoundServiceHandle {
    status: ServiceStatus,
}

impl SoundServiceHandle {
    pub fn new() -> Self {
        Self {
            status: ServiceStatus::INACTIVE,
        }
    }

    pub async fn run(&mut self, sender: Sender<Message>) {
        let task = "run";
        let mut interval = time::interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
            match SoundService::get_sound_value().await {
                Ok(value) => {
                    let _ = sender.send(Message::Sound { value });
                }
                Err(e) => {
                    error!(task, "error while getting sound status {}", e);
                    let _ = sender.send(Message::Sound { value: 0 });
                }
            };
        }
    }

    pub fn stop(&mut self) {
        self.status = ServiceStatus::STOPPED;
    }

    pub fn start(&mut self) {
        self.status = ServiceStatus::STARTED;
    }
}