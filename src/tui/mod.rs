mod style;
mod components;
use std::{
    io,
};
use crossterm::{
    terminal::enable_raw_mode,
};
use tui::{
    Terminal,
    backend::{
        CrosstermBackend
    },
    widgets::{ Block, Borders, Text, Paragraph },
    layout::{ Layout, Constraint, Direction, Alignment },
};
use crate::{
    AppState,
};
use components::*;

pub struct Tui {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl Tui {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        enable_raw_mode()?;
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        Ok(Self {
            terminal: Terminal::new(backend)?
        })
    }

    pub fn draw(&mut self, state: &mut AppState) -> io::Result<()> {
        let tui_window_info = match &state.active_window_info {
            Some(info) => TUIWindowInfo::from(info),
            None => TUIWindowInfo::default()
        };

        self.terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(40),
                        Constraint::Percentage(60),
                    ].as_ref()
                )
                .split(f.size());

            let text = tui_window_info.to_widgets();
            let block = Block::default()
                .title("Active window info")
                .borders(Borders::ALL);
            let window_info = Paragraph::new(text.iter())
                .block(block.clone())
                .alignment(Alignment::Left);
            f.render_widget(window_info, chunks[0]);

            let block = Block::default()
                .title("Block 2")
                .borders(Borders::ALL);
            f.render_widget(block, chunks[1]);
        })
    }
}