use std::sync::mpsc;
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use anyhow::{anyhow, ensure, Result};

use rood::cli::OutputManager;

use crate::tidy::Monitor;

pub struct MonitorThread {
    thread_handle: Option<JoinHandle<()>>,
    stop_channel: mpsc::Sender<bool>,
}

impl MonitorThread {
    pub fn start(m: Monitor, output: OutputManager) -> MonitorThread {
        let (tx, rx) = mpsc::channel();

        MonitorThread {
            thread_handle: Some(thread::spawn(move || {
                MonitorThread::thread_main(m, rx, output)
            })),
            stop_channel: tx,
        }
    }

    pub fn signal_stop(&self) -> Result<()> {
        match self.stop_channel.send(true) {
            Err(e) => Err(anyhow!("SendError: {}", e.to_string())),
            _ => Ok(()),
        }
    }

    pub fn wait(&mut self) -> Result<()> {
        let res = self
            .thread_handle
            .take()
            .ok_or_else(|| anyhow!("Cannot call stop on a stopped thread"))? // TODO: Here again, refactor to prevent invalid states.
            .join();

        ensure!(res.is_ok(), "Failed to join thread");

        Ok(())
    }

    fn check(mon: &Monitor, output: &OutputManager) {
        output.step(&format!(
            "Checking monitor {}...",
            mon.root_directory.to_str().unwrap()
        ));
        match mon.check() {
            Ok(_) => {
                // TODO: Print execution report in debug info.
                output.success("OK");
            }
            Err(e) => output.error(&format!("{}", e)),
        }
    }

    fn thread_main(monitor: Monitor, signal_receiver: mpsc::Receiver<bool>, output: OutputManager) {
        output.debug(&format!(
            "Monitor started for [{}]",
            &monitor.root_directory.to_str().unwrap()
        ));

        // Perform the first monitor check.
        MonitorThread::check(&monitor, &output);
        let mut last_check_time = Instant::now();

        loop {
            thread::sleep(Duration::from_millis(50)); // TODO: Customize.

            match signal_receiver.try_recv() {
                Ok(_) | Err(mpsc::TryRecvError::Disconnected) => {
                    // Terminating...
                    output.debug(&format!(
                        "Monitor for [{}] received exit signal",
                        &monitor.root_directory.to_str().unwrap()
                    ));
                    break;
                }
                Err(mpsc::TryRecvError::Empty) => {}
            }

            let current_instant = Instant::now();
            if current_instant.duration_since(last_check_time) > monitor.check_frequency {
                MonitorThread::check(&monitor, &output);
                last_check_time = current_instant;
            }
        }
    }
}
