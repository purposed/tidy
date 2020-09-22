//! Abstract condition execution library.

mod action;
mod condition;
mod field_value;
mod get_field;
mod lexer;
mod parser;
mod rule;

// Library interface
pub use action::Action;
pub use condition::Condition;
pub use field_value::FieldValue;
pub use get_field::GetField;
pub use rule::Rule;

use parser::parse as parse_condition;

#[cfg(test)]
mod tests;
