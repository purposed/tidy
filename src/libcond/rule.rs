use std::convert::TryFrom;
use std::fmt::Display;

use snafu::{ResultExt, Snafu};

use crate::{Action, Condition, GetField};

#[derive(Clone, Debug, Snafu)]
pub enum Error {
    ConditionEvaluationError { message: String },
    ActionExecutionError { message: String },
}

pub struct Rule<T, F>
where
    F: TryFrom<String>,
{
    //name: String,
    condition: Condition<F>,
    action: Box<dyn Action<T, F> + Send>,
}

impl<T, F> Rule<T, F>
where
    T: GetField<F>,
    T::Error: Display,
    F: TryFrom<String>,
{
    pub fn new(
        //name: String,
        condition: Condition<F>,
        action: Box<dyn Action<T, F> + Send>,
    ) -> Rule<T, F> {
        Rule {
            //name,
            condition,
            action,
        }
    }

    pub fn apply(&self, item: &T) -> Result<bool, Error> {
        let cond_val = self
            .condition
            .eval(item)
            .map_err(|e| Error::ConditionEvaluationError {
                message: e.to_string(),
            })?;
        if cond_val {
            self.action
                .execute(item)
                .map_err(|e| Error::ActionExecutionError {
                    message: e.to_string(),
                })?;
        }
        Ok(cond_val)
    }
}
