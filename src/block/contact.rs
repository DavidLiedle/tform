//! Contact information composite block.

use crate::block::Block;
use crate::field::{Field, TextInput};
use crate::validation::rules::{Email, Pattern};

/// A composite block for contact information.
pub struct ContactBlock {
    prefix: String,
    title: Option<String>,
    required: bool,
}

impl ContactBlock {
    /// Creates a new contact block with the given prefix.
    pub fn new(prefix: impl Into<String>) -> Self {
        Self {
            prefix: prefix.into(),
            title: None,
            required: false,
        }
    }

    /// Sets the block title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Marks all fields in this block as required.
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    fn field_id(&self, name: &str) -> String {
        format!("{}_{}", self.prefix, name)
    }
}

impl Block for ContactBlock {
    fn prefix(&self) -> &str {
        &self.prefix
    }

    fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    fn fields(&self) -> Vec<Box<dyn Field>> {
        let mut fields: Vec<Box<dyn Field>> = Vec::new();

        // Full Name
        let mut name = TextInput::new(self.field_id("name"), "Full Name")
            .placeholder("John Doe");
        if self.required {
            name = name.required();
        }
        fields.push(Box::new(name));

        // Email
        let mut email = TextInput::new(self.field_id("email"), "Email")
            .placeholder("john@example.com")
            .validator(Box::new(Email));
        if self.required {
            email = email.required();
        }
        fields.push(Box::new(email));

        // Phone
        let phone = TextInput::new(self.field_id("phone"), "Phone")
            .placeholder("(555) 123-4567")
            .validator(Box::new(Pattern::phone()));
        fields.push(Box::new(phone));

        fields
    }
}
