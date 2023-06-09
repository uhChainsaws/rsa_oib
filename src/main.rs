#![allow(non_snake_case)]
#![allow(mixed_script_confusables)]

mod ui;
mod app;
mod rsa;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{CrosstermBackend}, Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    // create app and run it
    let app = crate::app::App::new("rsa");
    let tick_rate: std::time::Duration = std::time::Duration::from_millis(200);
    let res = crate::app::run_app(&mut terminal, app, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("хыыы еррор чзх");
        println!("{:?}", err);
    }

    Ok(())
}

