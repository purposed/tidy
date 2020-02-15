use std::convert::TryFrom;

use rood::{Cause, CausedResult, Error};

use crate::{parse_condition, GetField};

#[derive(Debug)]
pub enum FieldOperator {
    Equal,
    NotEqual,
    Lt,
    Gt,
    Leq,
    Geq,
}

impl TryFrom<String> for FieldOperator {
    type Error = Error;
    fn try_from(v: String) -> CausedResult<FieldOperator> {
        match v.as_ref() {
            "==" => Ok(FieldOperator::Equal),
            "!=" => Ok(FieldOperator::NotEqual),
            "<" => Ok(FieldOperator::Lt),
            ">" => Ok(FieldOperator::Gt),
            "<=" => Ok(FieldOperator::Leq),
            ">=" => Ok(FieldOperator::Geq),
            _ => Err(Error::new(
                Cause::InvalidData,
                &format!("Could not cast '{}' to FieldOperator", v),
            )),
        }
    }
}

#[derive(Debug)]
pub enum BoolOperator {
    And,
    Or,
    Xor,
}

impl TryFrom<String> for BoolOperator {
    type Error = Error;

    fn try_from(v: String) -> CausedResult<BoolOperator> {
        match v.as_ref() {
            "&&" => Ok(BoolOperator::And),
            "||" => Ok(BoolOperator::Or),
            "-|" => Ok(BoolOperator::Xor),
            _ => Err(Error::new(
                Cause::InvalidData,
                &format!("Could not cast '{}' to BoolOperator", v),
            )),
        }
    }
}

#[derive(Debug)]
pub enum Condition<F> {
    FieldCondition(BaseCondition<F>),
    BoolCondition(Box<NestedCondition<F>>),
}

impl<F> Condition<F>
where
    F: TryFrom<String>,
{
    pub fn parse(src: &str) -> CausedResult<Condition<F>> {
        parse_condition(src)
    }

    pub fn eval<T>(&self, target: &T) -> CausedResult<bool>
    where
        T: GetField<F>,
    {
        match self {
            Condition::FieldCondition(f) => f.eval(target),
            Condition::BoolCondition(b) => b.eval(target),
        }
    }
}

#[derive(Debug)]
pub struct BaseCondition<F> {
    value: String,
    operator: FieldOperator,
    field: F,
}

impl<F> BaseCondition<F>
where
    F: TryFrom<String>,
{
    pub fn new(value: String, operator: FieldOperator, field: F) -> BaseCondition<F> {
        BaseCondition {
            value,
            operator,
            field,
        }
    }

    pub fn eval<T>(&self, target: &T) -> CausedResult<bool>
    where
        T: GetField<F>,
    {
        let field_val = target.get_field(&self.field)?;

        Ok(match self.operator {
            FieldOperator::Equal => field_val == self.value,
            FieldOperator::NotEqual => field_val != self.value,
            FieldOperator::Gt => field_val > self.value,
            FieldOperator::Lt => field_val < self.value,
            FieldOperator::Geq => field_val >= self.value,
            FieldOperator::Leq => field_val <= self.value,
        })
    }
}

#[derive(Debug)]
pub struct NestedCondition<F> {
    lhs_cond: Condition<F>,
    operator: BoolOperator,
    rhs_cond: Condition<F>,
}

impl<F> NestedCondition<F>
where
    F: TryFrom<String>,
{
    pub fn new(
        lhs_cond: Condition<F>,
        operator: BoolOperator,
        rhs_cond: Condition<F>,
    ) -> NestedCondition<F> {
        NestedCondition {
            lhs_cond,
            operator,
            rhs_cond,
        }
    }

    pub fn eval<T>(&self, target: &T) -> CausedResult<bool>
    where
        T: GetField<F>,
    {
        let left_val = self.lhs_cond.eval(target)?;
        let right_val = self.rhs_cond.eval(target)?;
        Ok(match self.operator {
            BoolOperator::And => left_val && right_val,
            BoolOperator::Or => left_val || right_val,
            BoolOperator::Xor => (left_val || right_val) && !(left_val && right_val),
        })
    }
}
