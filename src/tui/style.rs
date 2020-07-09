use tui::style::{Color, Modifier, Style};

lazy_static! {
    pub static ref STYLE_TEXT_WARNING: Style = Style::default().fg(Color::Yellow);
    pub static ref STYLE_TEXT_ERROR: Style = Style::default().fg(Color::Red);
    pub static ref STYLE_TEXT_HEADER: Style =
        Style::default().modifier(Modifier::BOLD).fg(Color::Cyan);

    pub static ref STYLE_TEXT_NEUTRAL: Style =
        Style::default().fg(Color::DarkGray).modifier(Modifier::ITALIC);
    pub static ref STYLE_TEXT_PRODUCTIVE: Style =
        Style::default().fg(Color::LightMagenta).modifier(Modifier::ITALIC);
    pub static ref STYLE_TEXT_LEISURE: Style =
        Style::default().fg(Color::LightGreen).modifier(Modifier::ITALIC);
}
