use std::convert::TryFrom;

use crate::{ExecutionError, GetField};

pub trait Action<T, F>
where
    T: GetField<F>,
    F: TryFrom<String>,
{
    fn execute(&self, target: &T) -> Result<(), ExecutionError>;
}
