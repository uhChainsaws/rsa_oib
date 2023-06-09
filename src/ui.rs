

use crate::app::App;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{
        Block, Borders,
        Paragraph,
    },
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let input_style = Style::default().bg(Color::Rgb(236, 229, 199)).fg(Color::Rgb(17, 106, 123));
    let blankk_style = Style::default().bg(Color::Rgb(205, 194, 174)).fg(Color::Rgb(17, 106, 123));
    let public_style = Style::default().bg(Color::Rgb(194, 222, 220)).fg(Color::Rgb(17, 106, 123));
    let private_style = Style::default().bg(Color::Rgb(17, 106, 123)).fg(Color::Rgb(194, 222, 220));
    let root = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(100),
            // Constraint::Percentage(20), 
        ].as_ref())
        .split(f.size());
    f.render_widget(Block::default().style(blankk_style), f.size());

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4), 
            Constraint::Length(4), 
            Constraint::Length(4), 
            Constraint::Length(4), 
            Constraint::Percentage(30), 
        ].as_ref())
        .split(root[0]);

    let message = Paragraph::new(app.message.as_ref())
        .alignment(tui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("message"))
        .style(if app.inputting {Style::default().bg(Color::Rgb(152, 238, 204))} else {Style::default()});
    
    f.render_widget(message, chunks[0]);
    
    if app.inputting {
        f.set_cursor(
            (chunks[0].x + chunks[0].width + app.message.len() as u16 )/2,
            chunks[0].y + 1,
        )
    }


    let p_text = Paragraph::new(
    {
            if let Some(p) = &app.p {
                format!("{}", p)
            }
            else {
                String::from("not yet set")
            }
        }
        )
        .alignment(tui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("p").style(input_style));
    let q_text = Paragraph::new(
    {
            if let Some(q) = &app.q {
                format!("{}", q)
            }
            else {
                String::from("not yet set")
            }
        }
        )
        .alignment(tui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("q").style(input_style));
    
    let qp_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            // Constraint::Percentage(10),
            Constraint::Percentage(50),
        ].as_ref())
        .split(chunks[1]);

    
    f.render_widget(p_text, qp_chunks[0]);
    f.render_widget(Block::default().style(blankk_style), qp_chunks[1]);
    f.render_widget(q_text, qp_chunks[1]);

    let n_text = Paragraph::new(
        {
                if let Some(n) = &app.n {
                    format!("{}", n)
                }
                else {
                    String::from("not yet set")
                }
            }
            )
            .alignment(tui::layout::Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("n")).style(public_style);

    let d_text = Paragraph::new(
        {
                if let Some(d) = &app.d {
                    format!("{}", d)
                }
                else {
                    String::from("not yet set")
                }
            }
            )
            .alignment(tui::layout::Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("d")).style(private_style);
    f.render_widget(n_text, chunks[2]);
    f.render_widget(d_text, chunks[3]);
    let texxxt: String = {
        if let (Some(encrypted), Some(decrypted)) = (app.encrypted.clone(), app.decrypted.clone()) {
            if app.message.len()>0 {
                let mess = num::bigint::BigInt::from_bytes_be(num::bigint::Sign::Plus, app.message.as_bytes());
                let m_bytes = app.decrypted.clone().unwrap().to_signed_bytes_be();
                match String::from_utf8(m_bytes){
                    Ok(mmm_str) => {format!("\nmessage in number is\n{}\nencrypting with e = {}...\nthe encrypted message is calculated with:\nenc = m^e mod n =\n{}\n\ndecryption is similar:\nc^d mod n =\n{}\n\ndecryption result:\n{}", mess, app.e, encrypted, decrypted, mmm_str)},
                    Err(eeee) => {format!("{}", eeee)}
                }
            }
            else {
                String::from("no message to encrypt!")
            }
        }
        else {
            String::from("...")
        }
    };
    let results = Paragraph::new(texxxt).alignment(tui::layout::Alignment::Center);
    f.render_widget(results, chunks[4])
}
