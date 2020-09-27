use std::convert::TryFrom;

use crate::{Error, FieldValue};

pub trait GetField<T: TryFrom<String>> {
    type Error: std::error::Error;

    fn get_field_value(&self, field: &T) -> Result<FieldValue, Self::Error>;

    fn get_field<R>(&self, field: &T) -> Result<R, Error>
    where
        R: TryFrom<FieldValue>,
    {
        let field_value = self.get_field_value(field)?;
        let r_val = R::try_from(field_value).map_err(|_e| Error::FieldTypeError)?;
        Ok(r_val)
    }
}
