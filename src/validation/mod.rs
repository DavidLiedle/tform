//! Validation traits and types.

pub mod rules;

/// A validation error for a specific field.
#[derive(Debug, Clone)]
pub struct ValidationError {
    /// The ID of the field that failed validation.
    pub field_id: String,
    /// The error message.
    pub message: String,
}

/// Trait for field validators.
pub trait Validator: Send + Sync {
    /// Validates a value and returns an error message if invalid.
    fn validate(&self, value: &str) -> Result<(), String>;
}
