use anyhow::Result;

use libcond::Action;

use serde::{Deserialize, Serialize};

use super::file::{File, FileField};

// Module Exports.
mod delete;
pub use delete::DeleteAction;

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum FileAction {
    Delete(DeleteAction),
}

impl FileAction {
    pub fn action_obj(&self) -> Result<Box<dyn Action<File, FileField> + Send>> {
        // Not super elegant, but works OK.
        Ok(match self {
            FileAction::Delete(a) => Box::from(a.clone()),
        })
    }
}
