# tform

A Rust TUI form builder crate built on [Ratatui](https://github.com/ratatui/ratatui). Create terminal forms with a fluent builder API, pre-built field types, and composite blocks for common patterns like addresses and contact info.

## Features

- **Fluent Builder API** - Chain methods to build forms quickly
- **Pre-built Fields** - TextInput, Select (dropdown), Checkbox
- **Composite Blocks** - AddressBlock, ContactBlock, DateRangeBlock
- **Validation** - Required, Email, MinLength, MaxLength, Pattern (regex)
- **Keyboard Navigation** - Tab, Shift+Tab, Arrow keys
- **Theming** - Customizable styles with dark/light presets
- **JSON Export** - Serialize form data to JSON files

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
tform = "0.1.0"
```

## Quick Start

```rust
use std::io;
use crossterm::event::{self, Event};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::execute;
use ratatui::{backend::CrosstermBackend, Terminal};
use tform::{Form, FormResult, AddressBlock, Email};

fn main() -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Build the form
    let mut form = Form::builder()
        .title("Shipping Information")
        .text("name", "Full Name")
            .placeholder("John Doe")
            .required()
            .done()
        .text("email", "Email")
            .placeholder("john@example.com")
            .required()
            .validator(Box::new(Email))
            .done()
        .block(AddressBlock::new("shipping").required())
        .checkbox("newsletter", "Subscribe to newsletter")
            .done()
        .build();

    // Event loop
    loop {
        terminal.draw(|frame| {
            form.render(frame.area(), frame.buffer_mut());
        })?;

        if let Event::Key(key) = event::read()? {
            form.handle_input(key);

            match form.result() {
                FormResult::Submitted => {
                    form.write_json("output.json")?;
                    break;
                }
                FormResult::Cancelled => break,
                FormResult::Active => {}
            }
        }
    }

    // Cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}
```

## Field Types

### TextInput

Single-line text input with cursor support.

```rust
Form::builder()
    .text("username", "Username")
        .placeholder("Enter username")
        .required()
        .initial_value("default")
        .validator(Box::new(MinLength(3)))
        .done()
    .build()
```

### Select

Dropdown selection with keyboard navigation.

```rust
Form::builder()
    .select("priority", "Priority")
        .option("low", "Low")
        .option("medium", "Medium")
        .option("high", "High")
        .required()
        .initial_value("medium")
        .done()
    .build()
```

### Checkbox

Toggle checkbox for boolean values.

```rust
Form::builder()
    .checkbox("terms", "I agree to the terms")
        .required()  // Must be checked
        .checked(false)
        .done()
    .build()
```

## Composite Blocks

Blocks are pre-configured groups of related fields.

### AddressBlock

US address with street, city, state (dropdown), and ZIP code validation.

```rust
use tform::AddressBlock;

Form::builder()
    .block(AddressBlock::new("shipping").required())
    .build()
```

Creates fields: `shipping_street1`, `shipping_street2`, `shipping_city`, `shipping_state`, `shipping_zip`

### ContactBlock

Contact information with email validation.

```rust
use tform::ContactBlock;

Form::builder()
    .block(ContactBlock::new("contact").required())
    .build()
```

Creates fields: `contact_name`, `contact_email`, `contact_phone`

### DateRangeBlock

Start and end date fields with YYYY-MM-DD format validation.

```rust
use tform::DateRangeBlock;

Form::builder()
    .block(DateRangeBlock::new("trip").required())
    .build()
```

Creates fields: `trip_start`, `trip_end`

## Validation

### Built-in Validators

```rust
use tform::{Required, Email, MinLength, MaxLength, Pattern};

// Required - field cannot be empty
.validator(Box::new(Required))

// Email - valid email format
.validator(Box::new(Email))

// MinLength - minimum character count
.validator(Box::new(MinLength(3)))

// MaxLength - maximum character count
.validator(Box::new(MaxLength(100)))

// Pattern - custom regex
.validator(Box::new(Pattern::new(r"^\d{3}-\d{4}$", "Invalid format")))

// Pre-built patterns
.validator(Box::new(Pattern::zip_code()))   // US ZIP code
.validator(Box::new(Pattern::phone()))      // US phone number
.validator(Box::new(Pattern::date()))       // YYYY-MM-DD
```

### Custom Validators

Implement the `Validator` trait:

```rust
use tform::Validator;

struct EvenNumber;

impl Validator for EvenNumber {
    fn validate(&self, value: &str) -> Result<(), String> {
        match value.parse::<i32>() {
            Ok(n) if n % 2 == 0 => Ok(()),
            Ok(_) => Err("Must be an even number".to_string()),
            Err(_) => Err("Must be a number".to_string()),
        }
    }
}
```

## Keyboard Navigation

| Key | Action |
|-----|--------|
| `Tab` | Next field |
| `Shift+Tab` | Previous field |
| `Up` / `Down` | Navigate fields (or dropdown options when open) |
| `Enter` | Submit form (on button) / Select option (in dropdown) |
| `Space` | Toggle checkbox / Open dropdown |
| `Esc` | Cancel form / Close dropdown |
| `Left` / `Right` | Move cursor in text fields |
| `Backspace` | Delete character before cursor |
| `Delete` | Delete character at cursor |
| `Ctrl+A` | Move cursor to start |
| `Ctrl+E` | Move cursor to end |
| `Ctrl+U` | Clear field |

## Theming

### Using Presets

```rust
use tform::FormStyle;

// Dark theme (default)
Form::builder()
    .style(FormStyle::dark())
    .build()

// Light theme
Form::builder()
    .style(FormStyle::light())
    .build()
```

### Custom Styles

```rust
use tform::FormStyle;
use ratatui::style::{Color, Modifier, Style};

let custom_style = FormStyle::new()
    .title(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))
    .label(Style::default().fg(Color::White))
    .label_focused(Style::default().fg(Color::Yellow))
    .input(Style::default().fg(Color::White).bg(Color::DarkGray))
    .input_focused(Style::default().fg(Color::White).bg(Color::Blue))
    .error(Style::default().fg(Color::Red))
    .button(Style::default().fg(Color::White).bg(Color::DarkGray))
    .button_focused(Style::default().fg(Color::Black).bg(Color::Green));

Form::builder()
    .style(custom_style)
    .build()
```

## JSON Output

Forms serialize to flat JSON with field IDs as keys:

```rust
// After form.write_json("output.json")
```

```json
{
  "name": "John Doe",
  "email": "john@example.com",
  "shipping_street1": "123 Main St",
  "shipping_street2": "",
  "shipping_city": "Springfield",
  "shipping_state": "IL",
  "shipping_zip": "62701",
  "newsletter": true
}
```

Access form data programmatically:

```rust
let data = form.to_json();
println!("{}", serde_json::to_string_pretty(&data)?);
```

## Example

Run the included example:

```bash
cargo run --example address_form
```

## License

MIT
