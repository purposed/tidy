//! Abstract condition execution library.

mod action;
mod condition;
mod field_value;
mod get_field;
mod parser;
mod rule;

// Library interface
pub use action::Action;
pub use condition::Condition;
pub use field_value::FieldValue;
pub use get_field::GetField;
pub use rule::Rule;

#[cfg(test)]
mod tests;
