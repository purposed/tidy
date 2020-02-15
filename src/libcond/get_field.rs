use std::convert::TryFrom;

use rood::CausedResult;

use crate::field_value::FieldValue;

pub trait GetField<T: TryFrom<String>> {
    fn get_field(&self, field: &T) -> CausedResult<FieldValue>;
}
