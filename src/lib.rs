//! # ratatui_form - Rust TUI Form Builder
//!
//! A Rust crate built on RataTUI that makes building terminal forms easy.
//! Developers import `ratatui_form` and use a fluent builder API to create forms
//! with pre-built field types and composite blocks (like address).
//!
//! ## Example
//!
//! ```no_run
//! use ratatui_form::{Form, AddressBlock};
//!
//! let form = Form::builder()
//!     .title("Shipping Info")
//!     .text("name", "Full Name").required().done()
//!     .text("email", "Email").required().done()
//!     .block(AddressBlock::new("shipping"))
//!     .build();
//! ```

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
