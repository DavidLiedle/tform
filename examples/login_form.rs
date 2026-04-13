//! Example: Minimal login form.
//!
//! Run with: `cargo run --example login_form`

use std::io;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use ratatui_form::{Form, FormResult, MinLength};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut form = Form::builder()
        .title("Log In")
        .text("username", "Username")
            .placeholder("your-handle")
            .required()
            .validator(Box::new(MinLength(3)))
            .done()
        .text("password", "Password")
            .placeholder("at least 8 characters")
            .required()
            .validator(Box::new(MinLength(8)))
            .done()
        .build();

    loop {
        terminal.draw(|frame| {
            form.render(frame.area(), frame.buffer_mut());
        })?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('c')
                && key.modifiers.contains(event::KeyModifiers::CONTROL)
            {
                break;
            }

            form.handle_input(key);

            match form.result() {
                FormResult::Submitted | FormResult::Cancelled => break,
                FormResult::Active => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    match form.result() {
        FormResult::Submitted => {
            println!("Logged in!");
            println!("{}", serde_json::to_string_pretty(&form.to_json()).unwrap());
        }
        FormResult::Cancelled => println!("Login cancelled."),
        FormResult::Active => println!("Form exited."),
    }

    Ok(())
}
