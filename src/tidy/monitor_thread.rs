use anyhow::{anyhow, Result};

use tokio::sync::mpsc;
use tokio::task::JoinHandle;

use crate::tidy::Monitor;

pub struct MonitorThread {
    handle: JoinHandle<()>,
    stop_channel: mpsc::Sender<bool>,
}

impl MonitorThread {
    pub fn start(m: Monitor) -> MonitorThread {
        let (tx, rx) = mpsc::channel(1);

        MonitorThread {
            handle: tokio::task::spawn(async move {
                if let Err(e) = MonitorThread::thread_main(m, rx).await {
                    log::error!("unhandled error in monitor thread: {}", e);
                }
            }),
            stop_channel: tx,
        }
    }

    pub async fn signal_stop(&self) -> Result<()> {
        match self.stop_channel.send(true).await {
            Err(e) => Err(anyhow!("SendError: {}", e.to_string())),
            _ => Ok(()),
        }
    }

    pub async fn join(self) -> Result<()> {
        self.handle.await?;
        Ok(())
    }

    fn check(mon: &Monitor) {
        log::info!("checking monitor for '{:?}'", mon.root_directory);
        match mon.check() {
            Ok(_) => {
                // TODO: Print execution report in debug info.
                log::info!("monitoring complete for '{:?}'", mon.root_directory);
            }
            Err(e) => {
                log::error!("{}", e);
            }
        }
    }

    async fn thread_main(
        monitor: Monitor,
        mut signal_receiver: mpsc::Receiver<bool>,
    ) -> Result<()> {
        log::debug!("monitor started for '{:?}'", &monitor.root_directory);

        loop {
            MonitorThread::check(&monitor);

            let delay = tokio::time::sleep(monitor.check_frequency);
            tokio::pin!(delay);

            let should_stop = tokio::select! {
                _ = &mut delay => {
                    false
                }
                _ = signal_receiver.recv() => {
                    true
                }
            };

            if should_stop {
                log::info!(
                    "monitor for '{:?}' received exit signal",
                    &monitor.root_directory
                );
                break;
            }
        }

        Ok(())
    }
}
