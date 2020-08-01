mod components;
mod style;
mod routes;
mod utils;
use crossterm::terminal::enable_raw_mode;
use std::io;
use tui::{
    backend::CrosstermBackend,
    Terminal,
    layout:: { Layout, Direction, Constraint },
    widgets::{ Block, Borders }
};
use crate::AppState;
use components::active_window_info::*;

pub use routes::*;

pub struct Tui {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    current_route_component: Box<dyn StatefulTUIComponent>,
    active_window_component: ActiveWindowInfo
}

impl Tui {
    pub fn new(state: &AppState) -> Result<Self, Box<dyn std::error::Error>> {
        enable_raw_mode()?;
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let current_route_component = Box::new(RouteMain::from(state));
        let active_window_component = ActiveWindowInfo::from(state);

        Ok(Self {
            terminal: Terminal::new(backend)?,
            current_route_component,
            active_window_component
        })
    }

    pub fn switch_route(&mut self, route: Routes, state: &AppState) {
        match route {
            Routes::Main => self.current_route_component = Box::new(RouteMain::from(state))
        }
    }

    pub fn tick(&mut self, state: &AppState) {
        self.active_window_component.tick(state);
        self.current_route_component.tick(state);
    }

    pub fn clear(&mut self) -> io::Result<()> {
        self.terminal.hide_cursor()?;
        self.terminal.clear()
    }

    pub fn draw(&mut self) -> std::io::Result<()> {
        let component = &self.current_route_component;
        let active_window_component = &self.active_window_component;
        
        self.terminal.draw(|ref mut f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Length(5), Constraint::Min(0), Constraint::Length(3)].as_ref())
                .split(f.size());

            let header_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)].as_ref())
                .split(chunks[0]);
            
            let footer_block = Block::default()
                .title(" Hint ")
                .borders(Borders::ALL);
            
            active_window_component.render(f, header_chunks[1]);
            component.render(f, chunks[1]);
            f.render_widget(footer_block, chunks[2]);
        })
    }
}

pub use self::{
    components::*,
};