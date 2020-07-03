use super::style as STYLE;
use crate::WindowInfo;
use std::borrow::Cow;
use tui::widgets::Text;

pub fn cow(str: &str) -> Cow<'_, str> {
    Cow::Borrowed(str)
}

#[derive(Default, Debug, Clone)]
pub struct TUIWindowInfo {
    title: String,
    app_name: String,
    app_class: String,
}

// TODO impl From<&WindowInfo> for Paragraph
impl From<&WindowInfo> for TUIWindowInfo {
    fn from(info: &WindowInfo) -> Self {
        let [title, app_name, app_class] = info.get_strings();
        Self {
            title,
            app_class,
            app_name,
        }
    }
}

impl TUIWindowInfo {
    pub fn to_widgets(&self) -> [Text; 8] {
        let default_string = String::default();
        if self.app_name == default_string
            || self.title == default_string
            || self.app_class == default_string
        {
            return self.to_widgets_none();
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
