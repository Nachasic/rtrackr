use super::style as STYLE;
use crate::{ record_store::Archetype };
use std::borrow::Cow;
use tui::widgets::Text;

pub fn cow(str: &str) -> Cow<'_, str> {
    Cow::Borrowed(str)
}

#[derive(Debug, Clone)]
pub struct TUIWindowInfo<'a> {
    pub archetype: &'a Option<Archetype>
}

pub trait ToWidgets {
    type Res: std::iter::IntoIterator;
    fn to_widgets(&self) -> Self::Res;
}

impl <'a> ToWidgets for TUIWindowInfo<'a> {
    type Res = Vec<Text<'a>>;
    fn to_widgets(&self) -> Self::Res {
        match self.archetype {
            None => vec![Text::Styled(cow("No active window \n"), *STYLE::STYLE_TEXT_WARNING)],
            Some(Archetype::AFK) => vec![Text::Styled(cow("AFK \n"), *STYLE::STYLE_TEXT_WARNING)],
            Some(Archetype::ActiveWindow(title, name, ..)) => vec![
                Text::Styled(cow("Active window:"), *STYLE::STYLE_TEXT_HEADER),
                Text::Raw(cow("\n")),
                Text::Raw(cow("\n")),
                Text::Styled(cow("Title: "), *STYLE::STYLE_TEXT_HEADER),
                Text::Raw(cow(title.as_str())),
                Text::Raw(cow("\n")),
                Text::Styled(cow("Application: "), *STYLE::STYLE_TEXT_HEADER),
                Text::Raw(cow(name.as_str())),
            ]
        }
    }
}
