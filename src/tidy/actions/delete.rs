use std::fs;

use serde::{Deserialize, Serialize};

use libcond::{Action, Error, GetField};

use crate::tidy::file::FileField;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DeleteAction {}

impl<T> Action<T, FileField> for DeleteAction
where
    T: GetField<FileField>,
{
    fn execute(&self, file: &T) -> Result<(), Error> {
        let path_str: String = file.get_field(&FileField::Path)?;

        if fs::metadata(&path_str)?.is_dir() {
            fs::remove_dir_all(&path_str)?;
        } else {
            fs::remove_file(&path_str)?;
        }

        Ok(())
    }
}
