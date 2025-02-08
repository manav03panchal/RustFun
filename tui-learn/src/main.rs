use crossterm::event::{self, Event};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

fn main() {
    let mut terminal = ratatui::init();
    loop {
        terminal.draw(draw).expect("Failed to draw frame");

        if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
            break;
        }
    }
    ratatui::restore();
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
