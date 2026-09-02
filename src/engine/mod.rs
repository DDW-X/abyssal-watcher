use std::sync::Arc;
use tokio::sync::mpsc::{self, Sender};
use tokio::time::{interval, Duration};
use crate::core::CheckStrategy;
use crate::infra::event_bus::EventBus;

pub enum EngineCommand {
    Tick,
}

pub struct Engine<T: CheckStrategy + Send + Sync + 'static> {
    strategy: Arc<T>,
    tx: Sender<EngineCommand>,
}

impl<T: CheckStrategy + Send + Sync + 'static> Engine<T> {
    pub fn new(strategy: Arc<T>) -> Self {
        let (tx, mut rx) = mpsc::channel(32);
        let cloned_strategy = Arc::clone(&strategy);
        tokio::spawn(async move {
            let mut ticker = interval(Duration::from_secs(2));
            loop {
                tokio::select! {
                    _ = ticker.tick() => {
                        if cloned_strategy.check() {
                            log::warn!("Threat detected by engine.");
                        } else {
                            log::info!("System check passed.");
                        }
                    }
                    Some(cmd) = rx.recv() => {
                        match cmd {
                            EngineCommand::Tick => {
                                log::debug!("Manual tick triggered.");
                            }
                        }
                    }
                }
            }
        });
        Self { strategy, tx }
    }

    pub fn trigger(&self) {
        let tx = self.tx.clone();
        tokio::spawn(async move {
            let _ = tx.send(EngineCommand::Tick).await;
        });
    }
}

pub mod threat_detector;
