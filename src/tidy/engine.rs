use std::convert::TryFrom;

use anyhow::Result;

use super::{Manifest, Monitor};
use crate::tidy::monitor_thread::MonitorThread;

pub struct Engine {
    monitors: Vec<MonitorThread>,
}

impl Engine {
    pub fn start(config_source: &str) -> Result<Engine> {
        let man: Manifest = serde_json::from_str(config_source)?;

        let monitors: Vec<MonitorThread> = man
            .monitors
            .into_iter()
            .map(Monitor::try_from)
            .collect::<Result<Vec<Monitor>>>()?
            .into_iter()
            .map(MonitorThread::start)
            .collect();

        Ok(Engine { monitors })
    }

    pub async fn stop(self) -> Result<()> {
        log::info!("sending stop signal to the tidy engine");

        // Get an iterator of joinable futures for stopping each task.
        let join_futures = self.monitors.into_iter().map(|mon| async move {
            mon.signal_stop().await?;
            mon.join().await?;
            Ok(()) as Result<()>
        });

        // Wait until all tasks are stopped and return the errors.
        futures::future::join_all(join_futures)
            .await
            .into_iter()
            .collect::<Result<Vec<()>>>()?;

        log::info!("goodbye!");

        Ok(())
    }
}
