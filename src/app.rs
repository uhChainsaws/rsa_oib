use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

use crate::rsa::{encrypt, decrypt};

#[derive(Debug)]
pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub inputting: bool,
    pub p: Option<num::BigInt>,
    pub q: Option<num::BigInt>,
    pub n: Option<num::BigInt>,
    pub e: num::BigInt,
    pub d: Option<num::BigInt>,
    pub encrypted: Option<num::BigInt>,
    pub decrypted: Option<num::BigInt>,
    pub message: String,
}

impl<'a> App<'a> {  
    pub fn on_tick(&mut self) {
        if let (&Some(p), &Some(q)) = (&self.p.as_ref(), &self.q.as_ref()) {
            if self.message.len()>0 {
                let (e, n, d) = crate::rsa::generate_keypair_pq(p, q);
                (self.e, self.n, self.d) = (e, Some(n.clone()), Some(d.clone()));    
                let mess = num::bigint::BigInt::from_bytes_be(num::bigint::Sign::Plus, self.message.as_bytes());
                self.encrypted = Some(encrypt(&mess, &self.e, &n));
                self.decrypted = Some(decrypt(&self.encrypted.clone().unwrap(), &d, &n));
            }
        }
    }

    pub fn on_key(&mut self, key: KeyEvent) {
        if self.inputting {
            match key.code {
                KeyCode::Enter | KeyCode::Esc => {
                    self.inputting = false
                }
                KeyCode::Char(c) => {
                    self.message.push(c);
                }
                KeyCode::Backspace => {
                    self.message.pop();
                }
                _ => {}
            }
        }
        else {
            match key.code {
                KeyCode::Char('q') => {
                    self.should_quit = true;
                }
                KeyCode::Char('r') => {
                    (self.p, self.q) = (Some(crate::rsa::gen_a_prime()), Some(crate::rsa::gen_a_prime()))
                }
                KeyCode::Char('i') => {
                    self.inputting = !self.inputting
                }
                _ => {}
            }
        }
    }

    pub fn new(title: &'a str) -> App<'a> {
        App { 
            title,
            should_quit: false,
            inputting: false,
            p: None,
            q: None,
            n: None,
            e: num::bigint::ToBigInt::to_bigint(&65537).unwrap(),
            d: None,
            decrypted: None,
            encrypted: None,
            message: String::from("what a mess"),
        }
    }
}
pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| crate::ui::draw(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.kind {
                    event::KeyEventKind::Press | event::KeyEventKind::Repeat => app.on_key(key),
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
        if app.should_quit {
            return Ok(());
        }
    }
}