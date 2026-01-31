//! Example: Address form with shipping information.
//!
//! Run with: `cargo run --example address_form`

use std::io;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use tform::{AddressBlock, Form, FormResult};

fn main() -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create the form
    let mut form = Form::builder()
        .title("Shipping Information")
        .text("name", "Full Name")
            .placeholder("John Doe")
            .required()
            .done()
        .text("email", "Email")
            .placeholder("john@example.com")
            .required()
            .validator(Box::new(tform::Email))
            .done()
        .text("phone", "Phone")
            .placeholder("(555) 123-4567")
            .done()
        .block(AddressBlock::new("shipping").required())
        .checkbox("newsletter", "Subscribe to newsletter")
            .done()
        .checkbox("terms", "I agree to the terms and conditions")
            .required()
            .done()
        .build();

    // Main loop
    loop {
        // Render
        terminal.draw(|frame| {
            let area = frame.area();
            form.render(area, frame.buffer_mut());
        })?;

        // Handle input
        if let Event::Key(key_event) = event::read()? {
            // Quick exit with Ctrl+C
            if key_event.code == KeyCode::Char('c')
                && key_event.modifiers.contains(event::KeyModifiers::CONTROL)
            {
                break;
            }

            form.handle_input(key_event);

            match form.result() {
                FormResult::Submitted => {
                    // Write JSON and exit
                    form.write_json("shipping.json")?;
                    break;
                }
                FormResult::Cancelled => {
                    break;
                }
                FormResult::Active => {}
            }
        }
    }

    // Cleanup terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    // Print result
    match form.result() {
        FormResult::Submitted => {
            println!("Form submitted! Data saved to shipping.json");
            println!("\nForm data:");
            println!("{}", serde_json::to_string_pretty(&form.to_json()).unwrap());
        }
        FormResult::Cancelled => {
            println!("Form cancelled.");
        }
        FormResult::Active => {
            println!("Form exited.");
        }
    }

    Ok(())
}
