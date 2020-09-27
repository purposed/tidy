use std::collections::HashMap;
use std::convert::TryFrom;

use snafu::Snafu;

use crate::field_value::FieldValue;
use crate::get_field::GetField;

#[derive(Debug, Snafu)]
pub enum Error {
    InvalidData,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum VirtualField {
    IntA,
    IntB,

    StringA,
    StringB,

    DurationA,
    DurationB,
}

impl TryFrom<String> for VirtualField {
    type Error = String;
    fn try_from(v: String) -> Result<VirtualField, String> {
        match v.as_ref() {
            "inta" => Ok(VirtualField::IntA),
            "intb" => Ok(VirtualField::IntB),
            "stringa" => Ok(VirtualField::StringA),
            "stringb" => Ok(VirtualField::StringB),
            "dura" => Ok(VirtualField::DurationA),
            "durb" => Ok(VirtualField::DurationB),
            _ => Err(String::from("Invalid field")),
        }
    }
}

#[derive(Debug)]
pub struct VirtualDocument {
    internal: HashMap<VirtualField, FieldValue>,
}

impl VirtualDocument {
    pub fn new(vals: Vec<(VirtualField, FieldValue)>) -> VirtualDocument {
        let mut data = HashMap::new();

        for (v_field, v_val) in vals.iter() {
            data.insert(v_field.clone(), v_val.clone());
        }

        VirtualDocument { internal: data }
    }
}

impl GetField<VirtualField> for VirtualDocument {
    type Error = Error;
    fn get_field_value(&self, field: &VirtualField) -> Result<FieldValue, Error> {
        Ok(self.internal.get(field).ok_or(Error::InvalidData)?.clone())
    }
}
