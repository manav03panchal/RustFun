use crossterm::event::{self, Event};
use ratatui::{
    layout::Direction,
    layout::{Constraint, Layout},
    text::Text,
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
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(20),
            Constraint::Percentage(40),
        ])
        .split(frame.area());
    let text = Text::raw("Hello, World!");
    frame.render_widget(
        ratatui::widgets::Paragraph::new(text).alignment(ratatui::layout::Alignment::Center),
        chunks[1],
    );
}
