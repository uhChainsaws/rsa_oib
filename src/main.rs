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
    backend::{Backend, CrosstermBackend}, Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {
    println!("\nalive!\n");
    let msg = String::from("yay rsa finally after like two semesters of no cool labbs");

    let p = rsa::gen_a_prime();
    let q = rsa::gen_a_prime();
    let (e,n,d) = rsa::generate_keypair_pq(&p, &q);

    let mess = num::bigint::BigInt::from_bytes_be(num::bigint::Sign::Plus, msg.as_bytes());

    let c = rsa::encrypt(&mess, &e, &n);
    let m = rsa::decrypt(&c, &d, &n);

    println!("publicckey: \n\t[ {} : {} ]\n", n, e);
    println!("private_ooh: \n\t[ {} ]\n", d);

    let m_bytes = m.to_signed_bytes_be();
    let m_str = String::from_utf8(m_bytes).unwrap();
    println!("m_str: {}", m_str);
    println!();

    // setup terminal
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

