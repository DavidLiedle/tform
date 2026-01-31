//! Built-in validation rules.

use crate::validation::Validator;

/// Validates that a field is not empty.
pub struct Required;

impl Validator for Required {
    fn validate(&self, value: &str) -> Result<(), String> {
        if value.trim().is_empty() {
            Err("This field is required".to_string())
        } else {
            Ok(())
        }
    }
}

/// Validates that a field contains a valid email address.
pub struct Email;

impl Validator for Email {
    fn validate(&self, value: &str) -> Result<(), String> {
        if value.is_empty() {
            return Ok(()); // Empty is OK, use Required for that
        }

        // Simple email validation
        let parts: Vec<&str> = value.split('@').collect();
        if parts.len() != 2 {
            return Err("Invalid email address".to_string());
        }

        let (local, domain) = (parts[0], parts[1]);

        if local.is_empty() || domain.is_empty() {
            return Err("Invalid email address".to_string());
        }

        if !domain.contains('.') {
            return Err("Invalid email address".to_string());
        }

        let domain_parts: Vec<&str> = domain.split('.').collect();
        if domain_parts.iter().any(|p| p.is_empty()) {
            return Err("Invalid email address".to_string());
        }

        Ok(())
    }
}

/// Validates minimum string length.
pub struct MinLength(pub usize);

impl Validator for MinLength {
    fn validate(&self, value: &str) -> Result<(), String> {
        if value.is_empty() {
            return Ok(()); // Empty is OK, use Required for that
        }

        if value.len() < self.0 {
            Err(format!("Must be at least {} characters", self.0))
        } else {
            Ok(())
        }
    }
}

/// Validates maximum string length.
pub struct MaxLength(pub usize);

impl Validator for MaxLength {
    fn validate(&self, value: &str) -> Result<(), String> {
        if value.len() > self.0 {
            Err(format!("Must be at most {} characters", self.0))
        } else {
            Ok(())
        }
    }
}

/// Validates against a regex pattern.
pub struct Pattern {
    regex: regex::Regex,
    message: String,
}

impl Pattern {
    /// Creates a new pattern validator.
    ///
    /// # Panics
    /// Panics if the pattern is not a valid regex.
    pub fn new(pattern: &str, message: impl Into<String>) -> Self {
        Self {
            regex: regex::Regex::new(pattern).expect("Invalid regex pattern"),
            message: message.into(),
        }
    }

    /// Creates a US ZIP code validator (5 digits or 5+4 format).
    pub fn zip_code() -> Self {
        Self::new(r"^\d{5}(-\d{4})?$", "Invalid ZIP code format")
    }

    /// Creates a US phone number validator.
    pub fn phone() -> Self {
        Self::new(
            r"^(\+1[-.\s]?)?(\(?\d{3}\)?[-.\s]?)?\d{3}[-.\s]?\d{4}$",
            "Invalid phone number format",
        )
    }

    /// Creates a date validator (YYYY-MM-DD format).
    pub fn date() -> Self {
        Self::new(r"^\d{4}-\d{2}-\d{2}$", "Invalid date format (use YYYY-MM-DD)")
    }
}

impl Validator for Pattern {
    fn validate(&self, value: &str) -> Result<(), String> {
        if value.is_empty() {
            return Ok(()); // Empty is OK, use Required for that
        }

        if self.regex.is_match(value) {
            Ok(())
        } else {
            Err(self.message.clone())
        }
    }
}
