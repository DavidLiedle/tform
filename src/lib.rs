//! # ratatui-form
//!
//! A fluent TUI form builder built on [Ratatui]. Compose forms from pre-built
//! field types and composite blocks, wire in validators, theme with presets or
//! custom styles, and export results as JSON.
//!
//! [Ratatui]: https://github.com/ratatui/ratatui
//!
//! ## Features
//!
//! - **Builder API** — chain `.text()`, `.select()`, `.checkbox()`, `.block()` to assemble a form.
//! - **Fields** — [`TextInput`], [`Select`] (dropdown), [`Checkbox`].
//! - **Blocks** — [`AddressBlock`], [`ContactBlock`], [`DateRangeBlock`] bundle related fields.
//! - **Validation** — [`Required`], [`Email`], [`MinLength`], [`MaxLength`], [`Pattern`], or your own [`Validator`].
//! - **Theming** — [`FormStyle::dark`] / [`FormStyle::light`] presets, or override any component style.
//! - **Output** — [`Form::to_json`] / [`Form::write_json`] for flat JSON export.
//!
//! ## Building a form
//!
//! ```no_run
//! use ratatui_form::{Form, AddressBlock, Email};
//!
//! let form = Form::builder()
//!     .title("Shipping Info")
//!     .text("name", "Full Name").required().done()
//!     .text("email", "Email")
//!         .required()
//!         .validator(Box::new(Email))
//!         .done()
//!     .block(AddressBlock::new("shipping").required())
//!     .checkbox("newsletter", "Subscribe to newsletter").done()
//!     .build();
//! ```
//!
//! See `examples/address_form.rs` for a complete event-loop wiring with
//! `crossterm` + `ratatui::Terminal`.
//!
//! ## Fields
//!
//! ```no_run
//! # use ratatui_form::{Form, MinLength};
//! // TextInput
//! Form::builder()
//!     .text("username", "Username")
//!         .placeholder("your-handle")
//!         .required()
//!         .validator(Box::new(MinLength(3)))
//!         .done()
//!     .build();
//! ```
//!
//! ```no_run
//! # use ratatui_form::Form;
//! // Select (dropdown)
//! Form::builder()
//!     .select("priority", "Priority")
//!         .option("low", "Low")
//!         .option("medium", "Medium")
//!         .option("high", "High")
//!         .initial_value("medium")
//!         .required()
//!         .done()
//!     .build();
//! ```
//!
//! ```no_run
//! # use ratatui_form::Form;
//! // Checkbox
//! Form::builder()
//!     .checkbox("terms", "I agree to the terms")
//!         .required()
//!         .done()
//!     .build();
//! ```
//!
//! ## Blocks
//!
//! Blocks expand into several related fields with sensible validators:
//!
//! ```no_run
//! use ratatui_form::{Form, AddressBlock, ContactBlock, DateRangeBlock};
//!
//! Form::builder()
//!     .block(ContactBlock::new("contact").required())      // _name, _email, _phone
//!     .block(AddressBlock::new("shipping").required())     // _street1.._zip
//!     .block(DateRangeBlock::new("trip").required())       // _start, _end (YYYY-MM-DD)
//!     .build();
//! ```
//!
//! ## Validation
//!
//! Built-in validators live at the crate root. Implement [`Validator`] for custom rules:
//!
//! ```
//! use ratatui_form::Validator;
//!
//! struct Even;
//! impl Validator for Even {
//!     fn validate(&self, value: &str) -> Result<(), String> {
//!         match value.parse::<i32>() {
//!             Ok(n) if n % 2 == 0 => Ok(()),
//!             Ok(_) => Err("must be even".into()),
//!             Err(_) => Err("must be a number".into()),
//!         }
//!     }
//! }
//! ```
//!
//! ## Theming
//!
//! ```no_run
//! use ratatui_form::{Form, FormStyle};
//! use ratatui::style::{Color, Modifier, Style};
//!
//! // Presets
//! Form::builder().style(FormStyle::light()).build();
//!
//! // Custom
//! let style = FormStyle::new()
//!     .title(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))
//!     .input_focused(Style::default().fg(Color::White).bg(Color::Blue))
//!     .button_focused(Style::default().fg(Color::Black).bg(Color::Green));
//! Form::builder().style(style).build();
//! ```
//!
//! ## JSON output
//!
//! [`Form::to_json`] returns a flat `serde_json::Value` keyed by field id;
//! [`Form::write_json`] writes the pretty-printed JSON to disk.

pub mod field;
pub mod block;
pub mod form;
pub mod navigation;
pub mod style;
pub mod validation;

pub use field::{Field, TextInput, Select, Checkbox};
pub use block::{Block, AddressBlock, ContactBlock, DateRangeBlock};
pub use form::{Form, FormBuilder, FormResult};
pub use navigation::FocusManager;
pub use style::FormStyle;
pub use validation::{ValidationError, Validator};
pub use validation::rules::{Required, Email, MinLength, MaxLength, Pattern};
