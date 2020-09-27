use std::convert::TryFrom;

use crate::field_value::FieldValue;

pub trait GetField<T: TryFrom<String>> {
    type Error: std::error::Error;

    fn get_field(&self, field: &T) -> Result<FieldValue, Self::Error>; // TODO: Would be nice if it could cast the field to the requested type instead of returning a fieldvalue.
}
