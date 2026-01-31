//! Text input field.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Widget;
use serde_json::Value;
use unicode_width::UnicodeWidthStr;

use crate::field::Field;
use crate::style::FormStyle;
use crate::validation::{ValidationError, Validator};

/// A single-line text input field.
pub struct TextInput {
    id: String,
    label: String,
    value: String,
    cursor_position: usize,
    placeholder: Option<String>,
    required: bool,
    validators: Vec<Box<dyn Validator>>,
    validation_errors: Vec<ValidationError>,
}

impl TextInput {
    /// Creates a new text input field.
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            value: String::new(),
            cursor_position: 0,
            placeholder: None,
            required: false,
            validators: Vec::new(),
            validation_errors: Vec::new(),
        }
    }

    /// Sets a placeholder text.
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    /// Marks this field as required.
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    /// Adds a validator to this field.
    pub fn validator(mut self, validator: Box<dyn Validator>) -> Self {
        self.validators.push(validator);
        self
    }

    /// Sets the initial value.
    pub fn initial_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self.cursor_position = self.value.len();
        self
    }

    fn insert_char(&mut self, c: char) {
        self.value.insert(self.cursor_position, c);
        self.cursor_position += c.len_utf8();
    }

    fn delete_char_before_cursor(&mut self) {
        if self.cursor_position > 0 {
            let prev_char_boundary = self.value[..self.cursor_position]
                .char_indices()
                .last()
                .map(|(i, _)| i)
                .unwrap_or(0);
            self.value.remove(prev_char_boundary);
            self.cursor_position = prev_char_boundary;
        }
    }

    fn delete_char_at_cursor(&mut self) {
        if self.cursor_position < self.value.len() {
            self.value.remove(self.cursor_position);
        }
    }

    fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position = self.value[..self.cursor_position]
                .char_indices()
                .last()
                .map(|(i, _)| i)
                .unwrap_or(0);
        }
    }

    fn move_cursor_right(&mut self) {
        if self.cursor_position < self.value.len() {
            self.cursor_position = self.value[self.cursor_position..]
                .char_indices()
                .nth(1)
                .map(|(i, _)| self.cursor_position + i)
                .unwrap_or(self.value.len());
        }
    }

    fn move_cursor_home(&mut self) {
        self.cursor_position = 0;
    }

    fn move_cursor_end(&mut self) {
        self.cursor_position = self.value.len();
    }
}

impl Field for TextInput {
    fn id(&self) -> &str {
        &self.id
    }

    fn label(&self) -> &str {
        &self.label
    }

    fn render(&self, area: Rect, buf: &mut Buffer, focused: bool, style: &FormStyle) {
        if area.height < 1 || area.width < 1 {
            return;
        }

        // Render label
        let label_style = if focused {
            style.label_focused
        } else {
            style.label
        };

        let required_marker = if self.required { "*" } else { "" };
        let label_text = format!("{}{}: ", self.label, required_marker);
        let label_width = label_text.width().min(area.width as usize);

        let label_span = Span::styled(&label_text, label_style);
        let label_line = Line::from(label_span);
        let label_area = Rect {
            x: area.x,
            y: area.y,
            width: label_width as u16,
            height: 1,
        };
        label_line.render(label_area, buf);

        // Calculate input area
        let input_x = area.x + label_width as u16;
        let input_width = area.width.saturating_sub(label_width as u16);

        if input_width == 0 {
            return;
        }

        // Determine what to display
        let (display_text, display_style) = if self.value.is_empty() {
            if let Some(ref placeholder) = self.placeholder {
                (placeholder.as_str(), style.placeholder)
            } else {
                ("", style.input)
            }
        } else {
            (self.value.as_str(), style.input)
        };

        // Render input value with background
        let input_bg_style = if focused {
            style.input_focused
        } else {
            style.input
        };

        // Fill input area with background
        for x in input_x..input_x + input_width {
            buf[(x, area.y)].set_style(input_bg_style);
            buf[(x, area.y)].set_char(' ');
        }

        // Render the text
        let visible_text: String = display_text.chars().take(input_width as usize).collect();
        for (i, c) in visible_text.chars().enumerate() {
            if input_x + i as u16 >= area.x + area.width {
                break;
            }
            buf[(input_x + i as u16, area.y)].set_char(c);
            buf[(input_x + i as u16, area.y)].set_style(display_style);
        }

        // Render cursor if focused
        if focused {
            let cursor_x = input_x + self.value[..self.cursor_position].width() as u16;
            if cursor_x < area.x + area.width {
                buf[(cursor_x, area.y)].set_style(
                    Style::default()
                        .bg(Color::White)
                        .fg(Color::Black)
                        .add_modifier(Modifier::SLOW_BLINK),
                );
            }
        }

        // Render validation errors if any
        if !self.validation_errors.is_empty() && area.height > 1 {
            let error_msg = &self.validation_errors[0].message;
            let error_span = Span::styled(error_msg, style.error);
            let error_line = Line::from(error_span);
            let error_area = Rect {
                x: input_x,
                y: area.y + 1,
                width: input_width,
                height: 1,
            };
            error_line.render(error_area, buf);
        }
    }

    fn handle_input(&mut self, event: &KeyEvent) -> bool {
        match event.code {
            KeyCode::Char(c) => {
                if event.modifiers.contains(KeyModifiers::CONTROL) {
                    match c {
                        'a' => self.move_cursor_home(),
                        'e' => self.move_cursor_end(),
                        'u' => {
                            self.value.clear();
                            self.cursor_position = 0;
                        }
                        _ => return false,
                    }
                } else {
                    self.insert_char(c);
                }
                true
            }
            KeyCode::Backspace => {
                self.delete_char_before_cursor();
                true
            }
            KeyCode::Delete => {
                self.delete_char_at_cursor();
                true
            }
            KeyCode::Left => {
                self.move_cursor_left();
                true
            }
            KeyCode::Right => {
                self.move_cursor_right();
                true
            }
            KeyCode::Home => {
                self.move_cursor_home();
                true
            }
            KeyCode::End => {
                self.move_cursor_end();
                true
            }
            _ => false,
        }
    }

    fn value(&self) -> Value {
        Value::String(self.value.clone())
    }

    fn validate(&self) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();

        // Check required
        if self.required && self.value.trim().is_empty() {
            errors.push(ValidationError {
                field_id: self.id.clone(),
                message: format!("{} is required", self.label),
            });
        }

        // Run validators
        for validator in &self.validators {
            if let Err(msg) = validator.validate(&self.value) {
                errors.push(ValidationError {
                    field_id: self.id.clone(),
                    message: msg,
                });
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn height(&self) -> u16 {
        if self.validation_errors.is_empty() {
            1
        } else {
            2
        }
    }

    fn is_required(&self) -> bool {
        self.required
    }
}
