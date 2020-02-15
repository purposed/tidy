use std::convert::TryFrom;
use std::path::Path;

use rood::{Cause, CausedResult, Error};

use super::{Manifest, Monitor};
use crate::tidy::monitor_thread::MonitorThread;
use rood::cli::OutputManager;

pub struct Engine {
    monitors: Option<Vec<Monitor>>,

    threads: Option<Vec<MonitorThread>>,

    output: OutputManager,
}

impl Engine {
    pub fn new(config_source: &str) -> CausedResult<Engine> {
        let man: Manifest = match serde_json::from_str(config_source) {
            Ok(m) => m,
            Err(e) => return Err(Error::new(Cause::InvalidData, "Invalid config JSON")),
        };

        let monitors = man
            .monitors
            .into_iter()
            .map(|f| Monitor::try_from(f))
            .collect::<CausedResult<Vec<Monitor>>>()?;

        Ok(Engine {
            monitors: Some(monitors),
            threads: None,
            output: OutputManager::new(true), // TODO: Parametrize verbosity
        })
    }

    pub fn start(&mut self) -> CausedResult<()> {
        self.output.step("Tidy Engine started");
        let mons = self.monitors.take().ok_or(Error::new(
            Cause::InvalidState,
            "Cannot start a started engine",
        ))?;

        let mut threads = Vec::new();
        for monitor in mons.into_iter() {
            threads.push(MonitorThread::start(monitor, self.output.push()));
        }
        self.threads = Some(threads);

        Ok(())
    }

    pub fn stop(&mut self) -> CausedResult<()> {
        self.output.step("Stopping Tidy Engine");
        let mut threads = self
            .threads
            .take()
            .ok_or(Error::new(Cause::InvalidState, "No threads running"))?;

        threads.iter_mut().try_for_each(|t| {
            t.signal_stop()?;
            t.wait()
        })?;

        self.output.step("Goodbye!");

        Ok(())
    }
}
