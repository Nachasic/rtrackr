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
    current_route: Routes,
    current_route_component: Box<dyn StatefulTUIComponent>
}

impl Tui {
    pub fn new(state: &AppState) -> Result<Self, Box<dyn std::error::Error>> {
        enable_raw_mode()?;
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let current_route_component = Box::new(RouteMain::from(state));
        Ok(Self {
            terminal: Terminal::new(backend)?,
            current_route: Routes::Main,
            current_route_component
        })
    }

    pub fn tick(&mut self, state: &AppState) {
        self.current_route_component.tick(state);
    }

    pub fn clear(&mut self) -> io::Result<()> {
        self.terminal.hide_cursor()?;
        self.terminal.clear()
    }

    pub fn draw(&mut self) -> std::io::Result<()> {
        let component = &self.current_route_component;
        self.terminal.draw(|ref mut f| component.render(f))
    }
}

pub use self::{
    components::*,
};