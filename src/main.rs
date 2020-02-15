mod tidy;

use std::fs;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time;

use tidy::{Engine, Manifest, Monitor};

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let raw_data =
        fs::read_to_string("/home/wdussault/.config/dalloriam/tidy/config.json").unwrap();
    let mut engine = Engine::new(&raw_data).unwrap();
    engine.start().unwrap();

    while running.load(Ordering::SeqCst) {}
    engine.stop().unwrap();
}
