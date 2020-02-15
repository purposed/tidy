use std::convert::TryFrom;

use rood::CausedResult;

use crate::GetField;

pub trait Action<T, F>
where
    T: GetField<F>,
    F: TryFrom<String>,
{
    fn execute(&self, target: &T) -> CausedResult<()>;
}
