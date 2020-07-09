mod components;
mod style;
mod routes;
use crossterm::terminal::enable_raw_mode;
use std::io;
use tui::{
    backend::CrosstermBackend,
    Terminal,
};
use crate::AppState;
pub use routes::*;

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

    pub fn clear(&mut self) -> io::Result<()> {
        self.terminal.hide_cursor()?;
        self.terminal.clear()
    }

    pub fn draw(&mut self, state: &AppState) -> std::io::Result<()> {
        let route = state.router.get_active_route();

        match route {
            Routes::Main => self.terminal.draw(|ref mut f| RenderTUI::<RouteMain>::render(state, f)),
        }
    }
}

pub use self::{
    components::*,
};