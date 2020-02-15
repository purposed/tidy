use std::convert::TryFrom;

use rood::CausedResult;

use crate::{Action, Condition, GetField};

pub struct Rule<T, F>
where
    F: TryFrom<String>,
{
    name: String,
    condition: Condition<F>,
    action: Box<dyn Action<T, F>>,
}

impl<T, F> Rule<T, F>
where
    T: GetField<F>,
    F: TryFrom<String>,
{
    pub fn new(name: String, condition: Condition<F>, action: Box<dyn Action<T, F>>) -> Rule<T, F> {
        Rule {
            name,
            condition,
            action,
        }
    }

    pub fn apply(&self, item: &T) -> CausedResult<bool> {
        let cond_val = self.condition.eval(item)?;
        if cond_val {
            self.action.execute(item)?;
        }
        Ok(cond_val)
    }
}
