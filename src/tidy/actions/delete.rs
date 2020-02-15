use std::fs;

use rood::{Cause, CausedResult, Error};

use serde::{Deserialize, Serialize};

use libcond::{Action, FieldValue, GetField};

use crate::tidy::file::FileField;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DeleteAction {}

impl<T> Action<T, FileField> for DeleteAction
where
    T: GetField<FileField>,
{
    fn execute(&self, file: &T) -> CausedResult<()> {
        let field_val = file.get_field(&FileField::Path)?;
        match field_val {
            FieldValue::String(path) => {
                if fs::metadata(&path)?.is_dir() {
                    fs::remove_dir_all(&path)?;
                } else {
                    fs::remove_file(&path)?;
                }
                Ok(())
            }
            _ => Err(Error::new(Cause::InvalidData, "File path must be a string")),
        }
    }
}
