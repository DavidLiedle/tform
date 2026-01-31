//! Form and FormBuilder implementation.

use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Padding, Widget};
use serde_json::{Map, Value};

use crate::block::Block as FormBlock;
use crate::field::{Checkbox, Field, Select, TextInput};
use crate::navigation::FocusManager;
use crate::style::FormStyle;
use crate::validation::ValidationError;

/// Result of form submission.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormResult {
    /// Form was submitted successfully.
    Submitted,
    /// Form was cancelled.
    Cancelled,
    /// Form is still active.
    Active,
}

/// A form with fields and navigation.
pub struct Form {
    title: Option<String>,
    fields: Vec<Box<dyn Field>>,
    focus_manager: FocusManager,
    style: FormStyle,
    result: FormResult,
    validation_errors: Vec<ValidationError>,
}

impl Form {
    /// Creates a new form builder.
    pub fn builder() -> FormBuilder {
        FormBuilder::new()
    }

    /// Returns the current form result.
    pub fn result(&self) -> &FormResult {
        &self.result
    }

    /// Returns whether the form is still active.
    pub fn is_active(&self) -> bool {
        self.result == FormResult::Active
    }

    /// Handles keyboard input.
    pub fn handle_input(&mut self, event: KeyEvent) {
        // Handle global keys
        match event.code {
            KeyCode::Esc => {
                self.result = FormResult::Cancelled;
                return;
            }
            KeyCode::Tab => {
                if event.modifiers.contains(KeyModifiers::SHIFT) {
                    self.focus_manager.focus_previous();
                } else {
                    self.focus_manager.focus_next();
                }
                return;
            }
            KeyCode::Enter if self.focus_manager.is_submit_focused() => {
                self.try_submit();
                return;
            }
            KeyCode::Down => {
                // Only move focus if the current field doesn't consume the event
                if !self.delegate_to_focused_field(&event) {
                    self.focus_manager.focus_next();
                }
                return;
            }
            KeyCode::Up => {
                if !self.delegate_to_focused_field(&event) {
                    self.focus_manager.focus_previous();
                }
                return;
            }
            _ => {}
        }

        // Delegate to focused field
        self.delegate_to_focused_field(&event);
    }

    fn delegate_to_focused_field(&mut self, event: &KeyEvent) -> bool {
        if self.focus_manager.is_submit_focused() {
            return false;
        }

        let index = self.focus_manager.current_index();
        if let Some(field) = self.fields.get_mut(index) {
            field.handle_input(event)
        } else {
            false
        }
    }

    fn try_submit(&mut self) {
        self.validation_errors.clear();

        for field in &self.fields {
            if let Err(errors) = field.validate() {
                self.validation_errors.extend(errors);
            }
        }

        if self.validation_errors.is_empty() {
            self.result = FormResult::Submitted;
        } else {
            // Focus on the first field with an error
            if let Some(error) = self.validation_errors.first() {
                for (i, field) in self.fields.iter().enumerate() {
                    if field.id() == error.field_id {
                        self.focus_manager.focus_field(i);
                        break;
                    }
                }
            }
        }
    }

    /// Returns the form data as a JSON object.
    pub fn to_json(&self) -> Value {
        let mut map = Map::new();

        for field in &self.fields {
            map.insert(field.id().to_string(), field.value());
        }

        Value::Object(map)
    }

    /// Writes the form data to a JSON file.
    pub fn write_json(&self, path: impl AsRef<Path>) -> io::Result<()> {
        let json = self.to_json();
        let mut file = File::create(path)?;
        let formatted = serde_json::to_string_pretty(&json)?;
        file.write_all(formatted.as_bytes())?;
        Ok(())
    }

    /// Returns validation errors.
    pub fn validation_errors(&self) -> &[ValidationError] {
        &self.validation_errors
    }

    /// Renders the form to a buffer.
    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        // Create the outer block with border
        let border_style = if self.focus_manager.is_submit_focused() {
            self.style.border
        } else {
            self.style.border_focused
        };

        let mut block = Block::default()
            .borders(Borders::ALL)
            .border_style(border_style)
            .padding(Padding::horizontal(1));

        if let Some(ref title) = self.title {
            block = block.title(Span::styled(title, self.style.title));
        }

        let inner_area = block.inner(area);
        block.render(area, buf);

        if inner_area.height < 2 || inner_area.width < 10 {
            return;
        }

        // Layout for fields and submit button
        let field_count = self.fields.len();
        let mut constraints = Vec::with_capacity(field_count + 2);

        for field in &self.fields {
            constraints.push(Constraint::Length(field.height()));
        }
        constraints.push(Constraint::Length(1)); // Spacer
        constraints.push(Constraint::Length(1)); // Submit button
        constraints.push(Constraint::Min(0)); // Remaining space

        let layout = Layout::vertical(constraints).split(inner_area);

        // Render each field
        for (i, field) in self.fields.iter().enumerate() {
            let is_focused = !self.focus_manager.is_submit_focused()
                && i == self.focus_manager.current_index();
            field.render(layout[i], buf, is_focused, &self.style);
        }

        // Render submit button
        let submit_idx = field_count + 1;
        if submit_idx < layout.len() {
            self.render_submit_button(layout[submit_idx], buf);
        }

        // Render validation errors summary if any
        if !self.validation_errors.is_empty() {
            let error_count = self.validation_errors.len();
            let error_msg = if error_count == 1 {
                "1 validation error".to_string()
            } else {
                format!("{} validation errors", error_count)
            };

            let error_area = Rect {
                x: inner_area.x,
                y: inner_area.y + inner_area.height.saturating_sub(1),
                width: inner_area.width,
                height: 1,
            };

            let error_line = Line::from(Span::styled(error_msg, self.style.error));
            error_line.render(error_area, buf);
        }
    }

    fn render_submit_button(&self, area: Rect, buf: &mut Buffer) {
        let is_focused = self.focus_manager.is_submit_focused();
        let style = if is_focused {
            self.style.button_focused
        } else {
            self.style.button
        };

        let text = if is_focused { "[ Submit ]" } else { "  Submit  " };

        // Center the button
        let button_width = text.len() as u16;
        let x = area.x + (area.width.saturating_sub(button_width)) / 2;

        for (i, c) in text.chars().enumerate() {
            if x + (i as u16) < area.x + area.width {
                buf[(x + i as u16, area.y)].set_char(c);
                buf[(x + i as u16, area.y)].set_style(style);
            }
        }
    }
}

/// Builder for creating forms.
pub struct FormBuilder {
    title: Option<String>,
    fields: Vec<Box<dyn Field>>,
    style: FormStyle,
}

impl FormBuilder {
    /// Creates a new form builder.
    pub fn new() -> Self {
        Self {
            title: None,
            fields: Vec::new(),
            style: FormStyle::default(),
        }
    }

    /// Sets the form title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the form style.
    pub fn style(mut self, style: FormStyle) -> Self {
        self.style = style;
        self
    }

    /// Starts building a text field.
    pub fn text(self, id: impl Into<String>, label: impl Into<String>) -> TextFieldBuilder {
        TextFieldBuilder::new(self, id.into(), label.into())
    }

    /// Starts building a select field.
    pub fn select(self, id: impl Into<String>, label: impl Into<String>) -> SelectFieldBuilder {
        SelectFieldBuilder::new(self, id.into(), label.into())
    }

    /// Starts building a checkbox field.
    pub fn checkbox(self, id: impl Into<String>, label: impl Into<String>) -> CheckboxFieldBuilder {
        CheckboxFieldBuilder::new(self, id.into(), label.into())
    }

    /// Adds a pre-built field.
    pub fn field(mut self, field: Box<dyn Field>) -> Self {
        self.fields.push(field);
        self
    }

    /// Adds all fields from a block.
    pub fn block(mut self, block: impl FormBlock) -> Self {
        for field in block.fields() {
            self.fields.push(field);
        }
        self
    }

    /// Builds the form.
    pub fn build(self) -> Form {
        let field_count = self.fields.len();
        Form {
            title: self.title,
            fields: self.fields,
            focus_manager: FocusManager::new(field_count),
            style: self.style,
            result: FormResult::Active,
            validation_errors: Vec::new(),
        }
    }
}

impl Default for FormBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for text fields.
pub struct TextFieldBuilder {
    form_builder: FormBuilder,
    field: TextInput,
}

impl TextFieldBuilder {
    fn new(form_builder: FormBuilder, id: String, label: String) -> Self {
        Self {
            form_builder,
            field: TextInput::new(id, label),
        }
    }

    /// Sets a placeholder.
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.field = self.field.placeholder(placeholder);
        self
    }

    /// Marks the field as required.
    pub fn required(mut self) -> Self {
        self.field = self.field.required();
        self
    }

    /// Sets the initial value.
    pub fn initial_value(mut self, value: impl Into<String>) -> Self {
        self.field = self.field.initial_value(value);
        self
    }

    /// Adds a validator.
    pub fn validator(mut self, validator: Box<dyn crate::validation::Validator>) -> Self {
        self.field = self.field.validator(validator);
        self
    }

    /// Finishes building this field and returns to the form builder.
    pub fn done(mut self) -> FormBuilder {
        self.form_builder.fields.push(Box::new(self.field));
        self.form_builder
    }
}

/// Builder for select fields.
pub struct SelectFieldBuilder {
    form_builder: FormBuilder,
    field: Select,
}

impl SelectFieldBuilder {
    fn new(form_builder: FormBuilder, id: String, label: String) -> Self {
        Self {
            form_builder,
            field: Select::new(id, label),
        }
    }

    /// Adds an option.
    pub fn option(mut self, value: impl Into<String>, display: impl Into<String>) -> Self {
        self.field = self.field.option(value, display);
        self
    }

    /// Adds multiple options.
    pub fn options(mut self, options: Vec<(impl Into<String>, impl Into<String>)>) -> Self {
        self.field = self.field.options(options);
        self
    }

    /// Marks the field as required.
    pub fn required(mut self) -> Self {
        self.field = self.field.required();
        self
    }

    /// Sets the initial value.
    pub fn initial_value(mut self, value: &str) -> Self {
        self.field = self.field.initial_value(value);
        self
    }

    /// Finishes building this field and returns to the form builder.
    pub fn done(mut self) -> FormBuilder {
        self.form_builder.fields.push(Box::new(self.field));
        self.form_builder
    }
}

/// Builder for checkbox fields.
pub struct CheckboxFieldBuilder {
    form_builder: FormBuilder,
    field: Checkbox,
}

impl CheckboxFieldBuilder {
    fn new(form_builder: FormBuilder, id: String, label: String) -> Self {
        Self {
            form_builder,
            field: Checkbox::new(id, label),
        }
    }

    /// Sets the initial checked state.
    pub fn checked(mut self, checked: bool) -> Self {
        self.field = self.field.checked(checked);
        self
    }

    /// Marks the field as required (must be checked).
    pub fn required(mut self) -> Self {
        self.field = self.field.required();
        self
    }

    /// Finishes building this field and returns to the form builder.
    pub fn done(mut self) -> FormBuilder {
        self.form_builder.fields.push(Box::new(self.field));
        self.form_builder
    }
}
