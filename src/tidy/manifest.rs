use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::actions::FileAction;

#[derive(Debug, Deserialize, Serialize)]
pub struct Manifest {
    pub monitors: Vec<MonitorDefinition>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MonitorDefinition {
    pub root_directory: PathBuf,
    pub recursive: bool,
    pub check_frequency: String, // TODO: Parse duration string,
    pub rules: Vec<RuleDefinition>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RuleDefinition {
    pub name: String,
    pub condition: String,
    pub action: FileAction,
}
