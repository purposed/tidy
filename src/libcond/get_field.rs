use std::convert::TryFrom;

use crate::field_value::FieldValue;

pub trait GetField<T: TryFrom<String>> {
    type Error;

    fn get_field(&self, field: &T) -> Result<FieldValue, Self::Error>;
}
