mod style;
use std::{
    io,
    borrow::Cow
};
use crossterm::{
    terminal::enable_raw_mode,
};
use tui::{
    Terminal,
    backend::{
        CrosstermBackend
    },
    widgets::{ Widget, Block, Borders, Text, Paragraph },
    layout::{ Layout, Constraint, Direction, Alignment },
    style::{ Color, Modifier, Style },
};
use crate::{
    AppState, WindowInfo
};
use style as STYLE;

fn cow(str: &str) -> Cow<'_, str> { Cow::Borrowed(str) }

pub struct Tui {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

#[derive(Default, Debug, Clone)]
pub struct TUIWindowInfo {
    title: String,
    app_name: String,
    app_class: String,
}

impl From<&WindowInfo> for TUIWindowInfo {
    fn from(info: &WindowInfo) -> Self {
        let [title, app_name, app_class] = info.get_strings();
        Self {
            title, app_class, app_name
        }
    }
}

impl TUIWindowInfo {
    pub fn to_widgets(&self) -> [Text; 8] {
        let default_string = String::default();
        if self.app_name == default_string ||
            self.title == default_string ||
            self.app_class == default_string {
                return self.to_widgets_none()
            };

        [
            Text::Styled(cow("Active window:"), *STYLE::STYLE_TEXT_HEADER),
            Text::Raw(cow("\n")),
            Text::Raw(cow("\n")),
            Text::Styled(cow("Title: "), *STYLE::STYLE_TEXT_HEADER),
            Text::Raw(cow(self.title.as_str())),
            Text::Raw(cow("\n")),
            Text::Styled(cow("Application: "), *STYLE::STYLE_TEXT_HEADER),
            Text::Raw(cow(self.app_name.as_str())),
        ]
    }

    fn to_widgets_none(&self) -> [Text; 8] {
        [
            Text::Styled(cow("No active window \n"), *STYLE::STYLE_TEXT_WARNING),
            Text::Raw(cow("\n")),
            Text::Raw(cow("\n")),
            Text::Raw(cow("\n")),
            Text::Raw(cow("\n")),
            Text::Raw(cow("\n")),
            Text::Raw(cow("\n")),
            Text::Raw(cow("\n")),
        ]
    }
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
        let active_window = &state.active_window_info;
        let mut tui_window_info = match &state.active_window_info {
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