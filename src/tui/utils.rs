use crate::{
    record_store::{ Archetype, ProductivityStatus },
    classifier::Classifiable,
    state::AppState,
    tui::{
        style as STYLE,
        components::{ ToWidgets }
    }
};
use tui::{
    widgets::{ Text }
};
use std::borrow::Cow;

pub fn cow(str: &str) -> Cow<str> {
    Cow::from(str)
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DisplayArchetype {
    pub archetype: Archetype,
    pub productivity: ProductivityStatus,
}

impl Classifiable for DisplayArchetype {
    fn get_archetype(&self) -> &Archetype {
        &self.archetype
    }

    fn assign_productivity(&mut self, productivity: ProductivityStatus) {
        self.productivity = productivity;
    }
}

impl From<&AppState> for Option<DisplayArchetype> {
    fn from(app: &AppState) -> Self {
        app.get_current_archetype()
            .clone()
            .map(|arch| {
                let mut display = DisplayArchetype {
                    archetype: arch,
                    productivity: ProductivityStatus::Neutral
                };
                app.classifier().classify(&mut display);
                display
            })
    }
}

const CAPTION_AFK: &'static str = r#"AFK"#;



impl <'a> ToWidgets for &'a ProductivityStatus {
    type Res = Vec<Text<'a>>;
    fn to_widgets(&self) -> Self::Res {
        match self {
            ProductivityStatus::Neutral => vec![Text::Styled(cow("Neutral"), *STYLE::STYLE_TEXT_NEUTRAL)],
            ProductivityStatus::Productive(activity) => vec![
                Text::Styled(cow("Productive ("), *STYLE::STYLE_TEXT_PRODUCTIVE),
                Text::Styled(cow(&activity), *STYLE::STYLE_TEXT_PRODUCTIVE),
                Text::Styled(cow(")"), *STYLE::STYLE_TEXT_PRODUCTIVE),
            ],
            ProductivityStatus::Leisure(activity) => vec![
                Text::Styled(cow("Leisure ("), *STYLE::STYLE_TEXT_LEISURE),
                Text::Styled(cow(&activity), *STYLE::STYLE_TEXT_LEISURE),
                Text::Styled(cow(")"), *STYLE::STYLE_TEXT_LEISURE),
            ]
        }
    }
}

impl <'a> ToWidgets for &'a Option<DisplayArchetype> {
    type Res = Vec<Text<'a>>;
    fn to_widgets(&self) -> Self::Res {
        match self {
            Some(data) => {
                let mut msg = match &data.archetype {
                    Archetype::AFK => vec![
                        Text::Styled(cow(CAPTION_AFK), *STYLE::STYLE_TEXT_WARNING),
                        Text::Raw(cow("\n")),
                        Text::Styled(cow("Productivity: "), *STYLE::STYLE_TEXT_HEADER),
                    ],
                    Archetype::ActiveWindow(title, name, ..) => vec![
                            // Text::Styled(cow("Active window:"), *STYLE::STYLE_TEXT_HEADER),
                            // Text::Raw(cow("\n")),
                            Text::Styled(cow("Title: "), *STYLE::STYLE_TEXT_HEADER),
                            Text::Raw(cow(title.as_str())),
                            Text::Raw(cow("\n")),
                            Text::Styled(cow("Application: "), *STYLE::STYLE_TEXT_HEADER),
                            Text::Raw(cow(name.as_str())),
                            Text::Raw(cow("\n")),
                            Text::Styled(cow("Productivity: "), *STYLE::STYLE_TEXT_HEADER),]
                };
                msg.append(&mut (&data.productivity).to_widgets());
                msg
            }
            None => vec![Text::Styled(cow("No active window \n"), *STYLE::STYLE_TEXT_WARNING)],
        }
    }
}
