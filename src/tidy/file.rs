use std::convert::Infallible;
use std::convert::TryFrom;
use std::fs;
use std::path::{Path, PathBuf};
use std::time;

use anyhow::{anyhow, Result};

use libcond::{FieldValue, GetField};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum FileField {
    Name,
    Extension,
    //Created,
    //Modified,
    Size,
    Age,
    Path,
}

impl TryFrom<String> for FileField {
    type Error = anyhow::Error;

    fn try_from(v: String) -> Result<FileField> {
        match v.as_ref() {
            "name" => Ok(FileField::Name),
            "extension" => Ok(FileField::Extension),
            "size" => Ok(FileField::Size),
            "age" => Ok(FileField::Age),
            "path" => Ok(FileField::Path),
            _ => Err(anyhow!("Canot cast {} to file field", &v)),
        }
    }
}

#[derive(Debug)]
pub struct File {
    metadata: fs::Metadata,
    path: PathBuf,
}

impl File {
    pub fn new<P>(path: P) -> Result<File>
    where
        P: AsRef<Path>,
    {
        let meta = fs::metadata(path.as_ref())?;

        Ok(File {
            metadata: meta,
            path: PathBuf::from(path.as_ref()),
        })
    }
}

impl GetField<FileField> for File {
    type Error = Infallible;

    fn get_field_value(&self, field: &FileField) -> Result<FieldValue, Self::Error> {
        // TODO: Handle errors cleanly
        match field {
            FileField::Name => Ok(FieldValue::String(String::from(
                self.path.file_name().unwrap().to_str().unwrap(),
            ))),
            FileField::Extension => Ok(FieldValue::String(String::from(
                self.path.extension().unwrap().to_str().unwrap(),
            ))),
            FileField::Path => Ok(FieldValue::String(String::from(
                self.path.to_str().unwrap(),
            ))),
            FileField::Size => Ok(FieldValue::Number(self.metadata.len())),
            FileField::Age => Ok(FieldValue::Duration(
                time::SystemTime::now()
                    .duration_since(self.metadata.modified().unwrap())
                    .unwrap(),
            )),
            //FileField::Created => Ok(FieldValue::Time(self.metadata.created()?)),
            //FileField::Modified => Ok(FieldValue::Time((self.metadata.modified()?)))
        }
    }
}
