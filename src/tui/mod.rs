mod components;
mod style;
use crate::AppState;
use components::*;
use crossterm::terminal::enable_raw_mode;
use std::io;
use style as STYLE;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph, Text},
    Terminal,
};

pub struct Tui {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl Tui {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        enable_raw_mode()?;
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        Ok(Self {
            terminal: Terminal::new(backend)?,
        })
    }

    pub fn draw(&mut self, state: &mut AppState) -> io::Result<()> {
        let tui_window_info = TUIWindowInfo {
            archetype: &state.active_window_info
        };

        self.terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
                .split(f.size());

            let window_info_text = tui_window_info.to_widgets();
            let afk_text = [Text::Styled(cow("AFK"), *STYLE::STYLE_TEXT_WARNING)];
            let block = Block::default()
                .title("Active window info")
                .borders(Borders::ALL);
            let window_notification = Paragraph::new(window_info_text.iter())
                .block(block.clone())
                .alignment(Alignment::Left);
            let afk_notification = Paragraph::new(afk_text.iter()).block(block.clone());

            if state.get_afk_seconds() > 10 {
                f.render_widget(afk_notification, chunks[0]);
            } else {
                f.render_widget(window_notification, chunks[0]);
            }

            let block = Block::default().title("Block 2").borders(Borders::ALL);
            f.render_widget(block, chunks[1]);
        })
    }
}
