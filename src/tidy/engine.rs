use std::convert::TryFrom;

use anyhow::{anyhow, Result};

use rood::cli::OutputManager;

use super::{Manifest, Monitor};
use crate::tidy::monitor_thread::MonitorThread;

pub struct Engine {
    monitors: Option<Vec<Monitor>>,

    threads: Option<Vec<MonitorThread>>,

    output: OutputManager,
}

impl Engine {
    pub fn new(config_source: &str, verbose: bool) -> Result<Engine> {
        let man: Manifest = serde_json::from_str(config_source)?;

        let monitors = man
            .monitors
            .into_iter()
            .map(Monitor::try_from)
            .collect::<Result<Vec<Monitor>>>()?;

        Ok(Engine {
            monitors: Some(monitors),
            threads: None,
            output: OutputManager::new(verbose),
        })
    }

    pub fn start(&mut self) -> Result<()> {
        // TODO: Do two-tier start struct to prevent invalid states (e.g. already started engines.)
        self.output.step("Tidy Engine started");
        let mons = self
            .monitors
            .take()
            .ok_or_else(|| anyhow!("Engine already started"))?;

        let mut threads = Vec::new();
        for monitor in mons.into_iter() {
            threads.push(MonitorThread::start(monitor, self.output.push()));
        }
        self.threads = Some(threads);

        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        self.output.step("Stopping Tidy Engine");
        let mut threads = self
            .threads
            .take()
            .ok_or_else(|| anyhow!("Engine not running"))?;

        threads.iter_mut().try_for_each(|t| {
            t.signal_stop()?;
            t.wait()
        })?;

        self.output.step("Goodbye!");

        Ok(())
    }
}
