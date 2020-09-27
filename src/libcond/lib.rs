//! Abstract condition execution library.

mod action;
mod condition;
mod error;
mod field_value;
mod get_field;
mod rule;
mod syntax;

// Library interface
pub use action::Action;
pub use condition::Condition;
pub use error::Error;
pub use field_value::FieldValue;
pub use get_field::GetField;
pub use rule::Rule;

#[cfg(test)]
mod tests;
