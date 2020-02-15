mod tidy;

use std::convert::TryFrom;
use std::fs;

use tidy::{Manifest, Monitor};

fn main() {
    let raw_data = fs::read_to_string("./test_cfg.json").unwrap();
    let man: Manifest = serde_json::from_str(&raw_data).unwrap();
    for mon_def in man.monitors.into_iter() {
        let mon = Monitor::try_from(mon_def).unwrap();
        mon.check().unwrap()
    }
}
