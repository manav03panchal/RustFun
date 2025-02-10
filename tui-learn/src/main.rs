use std::time::Duration;

use color_eyre::{eyre::Context, Result};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    DefaultTerminal, Frame, Terminal,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let app_result = run(terminal).context("app loop failed");
    ratatui::restore();
    app_result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(draw)?;
        if should_quit()? {
            break;
        }
    }
    Ok(())
}

fn draw(frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame.area());
    // let text = Text::raw("Hello, World!");
    let text_chunks = ["top", "bottom"];
    // frame.render_widget(
    //     ratatui::widgets::Paragraph::new(text).alignment(ratatui::layout::Alignment::Center),
    //     chunks[1],
    // );
    for (i, chunk_text) in text_chunks.iter().enumerate() {
        frame.render_widget(
            Paragraph::new(*chunk_text)
                .alignment(Alignment::Center)
                .block(Block::new().borders(Borders::ALL)),
            chunks[i],
        );
    }
}

fn should_quit() -> Result<bool> {
    if event::poll(Duration::from_millis(250)).context("event polling failed")? {
        if let Event::Key(key) = event::read().context("event reading failed")? {
            return Ok(KeyCode::Char('q') == key.code);
        }
    }
    Ok(false)
}

