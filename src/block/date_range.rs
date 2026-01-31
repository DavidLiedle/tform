//! Date range composite block.

use crossterm::event::KeyEvent;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use serde_json::Value;

use crate::block::Block;
use crate::field::{Field, TextInput};
use crate::style::FormStyle;
use crate::validation::ValidationError;
use crate::validation::rules::Pattern;

/// A composite block for date ranges (start date and end date).
pub struct DateRangeBlock {
    prefix: String,
    title: Option<String>,
    required: bool,
}

impl DateRangeBlock {
    /// Creates a new date range block with the given prefix.
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

impl Block for DateRangeBlock {
    fn prefix(&self) -> &str {
        &self.prefix
    }

    fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    fn fields(&self) -> Vec<Box<dyn Field>> {
        let mut fields: Vec<Box<dyn Field>> = Vec::new();

        // Start Date
        let mut start_date = TextInput::new(self.field_id("start"), "Start Date")
            .placeholder("YYYY-MM-DD")
            .validator(Box::new(Pattern::date()));
        if self.required {
            start_date = start_date.required();
        }
        fields.push(Box::new(start_date));

        // End Date
        let mut end_date = TextInput::new(self.field_id("end"), "End Date")
            .placeholder("YYYY-MM-DD")
            .validator(Box::new(Pattern::date()));
        if self.required {
            end_date = end_date.required();
        }
        fields.push(Box::new(end_date));

        fields
    }
}

/// A date range field that validates end >= start.
/// This is used internally to wrap the date range fields with cross-field validation.
#[allow(dead_code)]
pub struct DateRangeField {
    start_field: TextInput,
    end_field: TextInput,
    prefix: String,
    current_focus: usize, // 0 = start, 1 = end
}

#[allow(dead_code)]
impl DateRangeField {
    /// Creates a new date range field.
    pub fn new(prefix: impl Into<String>, required: bool) -> Self {
        let prefix = prefix.into();

        let mut start_field = TextInput::new(format!("{}_start", prefix), "Start Date")
            .placeholder("YYYY-MM-DD")
            .validator(Box::new(Pattern::date()));
        if required {
            start_field = start_field.required();
        }

        let mut end_field = TextInput::new(format!("{}_end", prefix), "End Date")
            .placeholder("YYYY-MM-DD")
            .validator(Box::new(Pattern::date()));
        if required {
            end_field = end_field.required();
        }

        Self {
            start_field,
            end_field,
            prefix,
            current_focus: 0,
        }
    }

    fn validate_range(&self) -> Result<(), ValidationError> {
        let start_value = match self.start_field.value() {
            Value::String(s) if !s.is_empty() => s,
            _ => return Ok(()),
        };

        let end_value = match self.end_field.value() {
            Value::String(s) if !s.is_empty() => s,
            _ => return Ok(()),
        };

        // Simple string comparison works for YYYY-MM-DD format
        if end_value < start_value {
            Err(ValidationError {
                field_id: format!("{}_end", self.prefix),
                message: "End date must be on or after start date".to_string(),
            })
        } else {
            Ok(())
        }
    }
}

impl Field for DateRangeField {
    fn id(&self) -> &str {
        &self.prefix
    }

    fn label(&self) -> &str {
        "Date Range"
    }

    fn render(&self, area: Rect, buf: &mut Buffer, focused: bool, style: &FormStyle) {
        if area.height < 2 {
            return;
        }

        let start_area = Rect {
            x: area.x,
            y: area.y,
            width: area.width,
            height: 1,
        };

        let end_area = Rect {
            x: area.x,
            y: area.y + 1,
            width: area.width,
            height: 1,
        };

        self.start_field.render(start_area, buf, focused && self.current_focus == 0, style);
        self.end_field.render(end_area, buf, focused && self.current_focus == 1, style);
    }

    fn handle_input(&mut self, event: &KeyEvent) -> bool {
        if self.current_focus == 0 {
            self.start_field.handle_input(event)
        } else {
            self.end_field.handle_input(event)
        }
    }

    fn value(&self) -> Value {
        serde_json::json!({
            "start": self.start_field.value(),
            "end": self.end_field.value()
        })
    }

    fn validate(&self) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();

        if let Err(mut e) = self.start_field.validate() {
            errors.append(&mut e);
        }

        if let Err(mut e) = self.end_field.validate() {
            errors.append(&mut e);
        }

        if let Err(e) = self.validate_range() {
            errors.push(e);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn height(&self) -> u16 {
        2
    }
}
