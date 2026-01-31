//! Focus and keyboard navigation management.

/// Manages focus navigation between form fields.
pub struct FocusManager {
    field_count: usize,
    current_index: usize,
    submit_button_focused: bool,
}

impl FocusManager {
    /// Creates a new focus manager.
    pub fn new(field_count: usize) -> Self {
        Self {
            field_count,
            current_index: 0,
            submit_button_focused: false,
        }
    }

    /// Returns the currently focused field index.
    pub fn current_index(&self) -> usize {
        self.current_index
    }

    /// Returns whether the submit button is focused.
    pub fn is_submit_focused(&self) -> bool {
        self.submit_button_focused
    }

    /// Moves focus to the next field.
    pub fn focus_next(&mut self) {
        if self.submit_button_focused {
            // Wrap around to first field
            self.submit_button_focused = false;
            self.current_index = 0;
        } else if self.current_index + 1 >= self.field_count {
            // Move to submit button
            self.submit_button_focused = true;
        } else {
            self.current_index += 1;
        }
    }

    /// Moves focus to the previous field.
    pub fn focus_previous(&mut self) {
        if self.submit_button_focused {
            // Move back to last field
            self.submit_button_focused = false;
            self.current_index = self.field_count.saturating_sub(1);
        } else if self.current_index > 0 {
            self.current_index -= 1;
        } else {
            // Wrap around to submit button
            self.submit_button_focused = true;
        }
    }

    /// Sets the total number of fields.
    pub fn set_field_count(&mut self, count: usize) {
        self.field_count = count;
        if self.current_index >= count {
            self.current_index = count.saturating_sub(1);
        }
    }

    /// Focuses on a specific field index.
    pub fn focus_field(&mut self, index: usize) {
        if index < self.field_count {
            self.current_index = index;
            self.submit_button_focused = false;
        }
    }

    /// Focuses on the submit button.
    pub fn focus_submit(&mut self) {
        self.submit_button_focused = true;
    }
}
