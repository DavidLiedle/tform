//! Select/dropdown field.

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Widget;
use serde_json::Value;
use unicode_width::UnicodeWidthStr;

use crate::field::Field;
use crate::style::FormStyle;
use crate::validation::ValidationError;

/// A select/dropdown field.
pub struct Select {
    id: String,
    label: String,
    options: Vec<(String, String)>, // (value, display)
    selected_index: Option<usize>,
    is_open: bool,
    highlighted_index: usize,
    required: bool,
}

impl Select {
    /// Creates a new select field.
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            options: Vec::new(),
            selected_index: None,
            is_open: false,
            highlighted_index: 0,
            required: false,
        }
    }

    /// Adds an option to the select.
    pub fn option(mut self, value: impl Into<String>, display: impl Into<String>) -> Self {
        self.options.push((value.into(), display.into()));
        self
    }

    /// Adds multiple options at once.
    pub fn options(mut self, options: Vec<(impl Into<String>, impl Into<String>)>) -> Self {
        for (value, display) in options {
            self.options.push((value.into(), display.into()));
        }
        self
    }

    /// Marks this field as required.
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    /// Sets the initial selected value.
    pub fn initial_value(mut self, value: &str) -> Self {
        for (i, (v, _)) in self.options.iter().enumerate() {
            if v == value {
                self.selected_index = Some(i);
                self.highlighted_index = i;
                break;
            }
        }
        self
    }

    fn toggle_open(&mut self) {
        self.is_open = !self.is_open;
        if self.is_open {
            if let Some(idx) = self.selected_index {
                self.highlighted_index = idx;
            }
        }
    }

    fn select_highlighted(&mut self) {
        if !self.options.is_empty() {
            self.selected_index = Some(self.highlighted_index);
        }
        self.is_open = false;
    }

    fn move_highlight_up(&mut self) {
        if self.highlighted_index > 0 {
            self.highlighted_index -= 1;
        }
    }

    fn move_highlight_down(&mut self) {
        if self.highlighted_index < self.options.len().saturating_sub(1) {
            self.highlighted_index += 1;
        }
    }
}

impl Field for Select {
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

        // Get selected display text
        let display_text = self
            .selected_index
            .and_then(|i| self.options.get(i))
            .map(|(_, display)| display.as_str())
            .unwrap_or("-- Select --");

        // Render the selected value with dropdown indicator
        let input_style = if focused {
            style.input_focused
        } else {
            style.input
        };

        // Fill input area with background
        for x in input_x..input_x + input_width {
            buf[(x, area.y)].set_style(input_style);
            buf[(x, area.y)].set_char(' ');
        }

        // Render selected text
        let arrow = if self.is_open { " ▲" } else { " ▼" };
        let max_text_width = input_width.saturating_sub(2) as usize;
        let truncated_text: String = display_text.chars().take(max_text_width).collect();

        for (i, c) in truncated_text.chars().enumerate() {
            if input_x + i as u16 >= area.x + area.width - 2 {
                break;
            }
            buf[(input_x + i as u16, area.y)].set_char(c);
        }

        // Render arrow
        let arrow_x = input_x + input_width - 2;
        for (i, c) in arrow.chars().enumerate() {
            if arrow_x + (i as u16) < area.x + area.width {
                buf[(arrow_x + i as u16, area.y)].set_char(c);
            }
        }

        // Render dropdown if open
        if self.is_open && area.height > 1 {
            let max_dropdown_height = (area.height - 1).min(self.options.len() as u16);

            for (i, (_, display)) in self.options.iter().enumerate() {
                if i >= max_dropdown_height as usize {
                    break;
                }

                let y = area.y + 1 + i as u16;
                let is_highlighted = i == self.highlighted_index;
                let is_selected = Some(i) == self.selected_index;

                let option_style = if is_highlighted {
                    Style::default().bg(Color::Blue).fg(Color::White)
                } else if is_selected {
                    Style::default().bg(Color::DarkGray).fg(Color::White)
                } else {
                    style.input
                };

                // Fill option row with background
                for x in input_x..input_x + input_width {
                    buf[(x, y)].set_style(option_style);
                    buf[(x, y)].set_char(' ');
                }

                // Render option text
                let prefix = if is_selected { "● " } else { "  " };
                for (j, c) in prefix.chars().enumerate() {
                    buf[(input_x + j as u16, y)].set_char(c);
                }

                let text_start = input_x + 2;
                for (j, c) in display.chars().enumerate() {
                    if text_start + j as u16 >= input_x + input_width {
                        break;
                    }
                    buf[(text_start + j as u16, y)].set_char(c);
                }
            }
        }
    }

    fn handle_input(&mut self, event: &KeyEvent) -> bool {
        match event.code {
            KeyCode::Enter | KeyCode::Char(' ') => {
                if self.is_open {
                    self.select_highlighted();
                } else {
                    self.toggle_open();
                }
                true
            }
            KeyCode::Esc => {
                if self.is_open {
                    self.is_open = false;
                    true
                } else {
                    false
                }
            }
            KeyCode::Up => {
                if self.is_open {
                    self.move_highlight_up();
                    true
                } else {
                    false
                }
            }
            KeyCode::Down => {
                if self.is_open {
                    self.move_highlight_down();
                    true
                } else {
                    self.toggle_open();
                    true
                }
            }
            _ => false,
        }
    }

    fn value(&self) -> Value {
        self.selected_index
            .and_then(|i| self.options.get(i))
            .map(|(v, _)| Value::String(v.clone()))
            .unwrap_or(Value::Null)
    }

    fn validate(&self) -> Result<(), Vec<ValidationError>> {
        if self.required && self.selected_index.is_none() {
            Err(vec![ValidationError {
                field_id: self.id.clone(),
                message: format!("{} is required", self.label),
            }])
        } else {
            Ok(())
        }
    }

    fn height(&self) -> u16 {
        if self.is_open {
            1 + self.options.len().min(10) as u16
        } else {
            1
        }
    }

    fn is_required(&self) -> bool {
        self.required
    }
}
