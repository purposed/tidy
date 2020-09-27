use std::convert::TryFrom;

use snafu::{ResultExt, Snafu};

use crate::syntax;
use crate::GetField;

#[derive(Clone, Debug, Snafu)]
pub enum Error {
    BoolOperatorCastError { operator: String },
    FieldOperatorCastError { operator: String },
    GetFieldError { message: String },
    ParseError { source: syntax::Error },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FieldOperator {
    Equal,
    NotEqual,
    Lt,
    Gt,
    Leq,
    Geq,
}

type Result<T> = std::result::Result<T, Error>;

impl TryFrom<&str> for FieldOperator {
    type Error = Error;

    fn try_from(v: &str) -> Result<FieldOperator> {
        match v {
            "==" => Ok(FieldOperator::Equal),
            "!=" => Ok(FieldOperator::NotEqual),
            "<" => Ok(FieldOperator::Lt),
            ">" => Ok(FieldOperator::Gt),
            "<=" => Ok(FieldOperator::Leq),
            ">=" => Ok(FieldOperator::Geq),
            _ => Err(Error::FieldOperatorCastError {
                operator: v.to_string(),
            }),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BoolOperator {
    And,
    Or,
    Xor,
}

impl TryFrom<&str> for BoolOperator {
    type Error = Error;

    fn try_from(v: &str) -> Result<BoolOperator> {
        match v.as_ref() {
            "&&" => Ok(BoolOperator::And),
            "||" => Ok(BoolOperator::Or),
            "-|" => Ok(BoolOperator::Xor),
            _ => Err(Error::BoolOperatorCastError {
                operator: v.to_string(),
            }),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Condition<F> {
    FieldCondition(BaseCondition<F>),
    BoolCondition(Box<NestedCondition<F>>),
}

impl<F> Condition<F>
where
    F: TryFrom<String>,
{
    pub fn parse(src: &str) -> Result<Condition<F>> {
        syntax::parse_condition(src).context(ParseError)
    }

    pub fn eval<T>(&self, target: &T) -> Result<bool>
    where
        T: GetField<F>,
        T::Error: ToString,
    {
        match self {
            Condition::FieldCondition(f) => f.eval(target),
            Condition::BoolCondition(b) => b.eval(target),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
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

    pub fn eval<T>(&self, target: &T) -> Result<bool>
    where
        T: GetField<F>,
        T::Error: ToString,
    {
        let field_val = target
            .get_field(&self.field)
            .map_err(|e| Error::GetFieldError {
                message: e.to_string(),
            })?;

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

#[derive(Clone, Debug, PartialEq)]
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

    pub fn eval<T>(&self, target: &T) -> Result<bool>
    where
        T: GetField<F>,
        T::Error: ToString,
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
