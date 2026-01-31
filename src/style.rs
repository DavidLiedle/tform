//! Form styling and theming.

use ratatui::style::{Color, Modifier, Style};

/// Style configuration for forms.
#[derive(Debug, Clone)]
pub struct FormStyle {
    /// Style for the form title.
    pub title: Style,
    /// Style for field labels.
    pub label: Style,
    /// Style for focused field labels.
    pub label_focused: Style,
    /// Style for input fields.
    pub input: Style,
    /// Style for focused input fields.
    pub input_focused: Style,
    /// Style for placeholder text.
    pub placeholder: Style,
    /// Style for error messages.
    pub error: Style,
    /// Style for the submit button.
    pub button: Style,
    /// Style for the focused submit button.
    pub button_focused: Style,
    /// Style for the form border.
    pub border: Style,
    /// Style for the focused form border.
    pub border_focused: Style,
}

impl Default for FormStyle {
    fn default() -> Self {
        Self {
            title: Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
            label: Style::default().fg(Color::White),
            label_focused: Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
            input: Style::default().fg(Color::White).bg(Color::DarkGray),
            input_focused: Style::default().fg(Color::White).bg(Color::Blue),
            placeholder: Style::default().fg(Color::Gray),
            error: Style::default().fg(Color::Red),
            button: Style::default().fg(Color::White).bg(Color::DarkGray),
            button_focused: Style::default()
                .fg(Color::Black)
                .bg(Color::Green)
                .add_modifier(Modifier::BOLD),
            border: Style::default().fg(Color::Gray),
            border_focused: Style::default().fg(Color::Cyan),
        }
    }
}

impl FormStyle {
    /// Creates a new form style with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the title style.
    pub fn title(mut self, style: Style) -> Self {
        self.title = style;
        self
    }

    /// Sets the label style.
    pub fn label(mut self, style: Style) -> Self {
        self.label = style;
        self
    }

    /// Sets the focused label style.
    pub fn label_focused(mut self, style: Style) -> Self {
        self.label_focused = style;
        self
    }

    /// Sets the input style.
    pub fn input(mut self, style: Style) -> Self {
        self.input = style;
        self
    }

    /// Sets the focused input style.
    pub fn input_focused(mut self, style: Style) -> Self {
        self.input_focused = style;
        self
    }

    /// Sets the placeholder style.
    pub fn placeholder(mut self, style: Style) -> Self {
        self.placeholder = style;
        self
    }

    /// Sets the error style.
    pub fn error(mut self, style: Style) -> Self {
        self.error = style;
        self
    }

    /// Sets the button style.
    pub fn button(mut self, style: Style) -> Self {
        self.button = style;
        self
    }

    /// Sets the focused button style.
    pub fn button_focused(mut self, style: Style) -> Self {
        self.button_focused = style;
        self
    }

    /// Creates a dark theme.
    pub fn dark() -> Self {
        Self::default()
    }

    /// Creates a light theme.
    pub fn light() -> Self {
        Self {
            title: Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
            label: Style::default().fg(Color::Black),
            label_focused: Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
            input: Style::default().fg(Color::Black).bg(Color::White),
            input_focused: Style::default().fg(Color::Black).bg(Color::LightBlue),
            placeholder: Style::default().fg(Color::DarkGray),
            error: Style::default().fg(Color::Red),
            button: Style::default().fg(Color::Black).bg(Color::White),
            button_focused: Style::default()
                .fg(Color::White)
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
            border: Style::default().fg(Color::DarkGray),
            border_focused: Style::default().fg(Color::Blue),
        }
    }
}
