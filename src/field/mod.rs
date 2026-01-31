//! Field types for form inputs.

mod text;
mod select;
mod checkbox;

pub use text::TextInput;
pub use select::Select;
pub use checkbox::Checkbox;

use crossterm::event::KeyEvent;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use serde_json::Value;

use crate::validation::ValidationError;
use crate::style::FormStyle;

/// Trait for form fields.
pub trait Field: Send + Sync {
    /// Returns the unique identifier for this field.
    fn id(&self) -> &str;

    /// Returns the display label for this field.
    fn label(&self) -> &str;

    /// Renders the field to the buffer.
    fn render(&self, area: Rect, buf: &mut Buffer, focused: bool, style: &FormStyle);

    /// Handles keyboard input. Returns true if the input was consumed.
    fn handle_input(&mut self, event: &KeyEvent) -> bool;

    /// Returns the current value as a JSON value.
    fn value(&self) -> Value;

    /// Validates the field and returns any errors.
    fn validate(&self) -> Result<(), Vec<ValidationError>>;

    /// Returns the height needed to render this field.
    fn height(&self) -> u16 {
        1
    }

    /// Returns whether this field is required.
    fn is_required(&self) -> bool {
        false
    }
}
