use crate::{
    event::Key,
    record_store::{ Archetype, ProductivityStatus, ActivityRecord },
    state::{ AppState },
    classifier::Classifiable,
    tui::{
        style as STYLE,
        components::{ ToWidgets, cow },
        routes::{ Route, TUIFrame, StatefulTUIComponent },
    }
};
use tui::{
    layout::{ Alignment, Constraint, Direction, Layout },
    widgets::{ Block, Borders, Paragraph, Text },
};
use std::{
    time::Duration
};

const CAPTION_AFK: &'static str = r#"
    █████╗ ███████╗██╗  ██╗
   ██╔══██╗██╔════╝██║ ██╔╝
   ███████║█████╗  █████╔╝ 
   ██╔══██║██╔══╝  ██╔═██╗ 
   ██║  ██║██║     ██║  ██╗
   ╚═╝  ╚═╝╚═╝     ╚═╝  ╚═╝
"#;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DisplayArchetype {
    pub archetype: Archetype,
    pub productivity: ProductivityStatus,
}

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
                            Text::Styled(cow("Active window:"), *STYLE::STYLE_TEXT_HEADER),
                            Text::Raw(cow("\n")),
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

/// Main screen of the app
/// Displays:
/// - Current focused window info
///    - Window title
///    - Application name
///    - Application class
///    - Productivity rating
/// - Total amound of time tracked today
/// - Productivity line-chart
/// - List of today's records (same info as for current focused window)
/// - Pause/resume tracking button
#[derive(Debug, Default, Clone)]
pub struct RouteMain {
    display: Option<DisplayArchetype>,
    records: Vec<ActivityRecord>,
    tracking_time: Duration
}
impl Route for RouteMain {}

impl From<&AppState> for RouteMain {
    fn from(state: &AppState) -> Self {
        let records = state.store().query_records()
            .unwrap_or(vec![]);
        let tracking_time = {
            records.iter().fold(
                Duration::from_secs(0),
                |duration, data| {
                    let (start, end) = data.time_range;

                    duration + end.duration_since(start)
                        .unwrap_or(Duration::from_secs(0))
                }
            )
        };
        Self {
            display: Option::<DisplayArchetype>::from(state),
            records,
            tracking_time
        }
    }
}

impl StatefulTUIComponent for RouteMain {
    fn handle_key(&mut self, event: Key) {
        
    }
    fn tick(&mut self, app_state: &AppState) {
        let records = app_state.store().query_records()
            .unwrap_or(vec![]);
        
        if records.len() != self.records.len() {
            self.tracking_time = {
                records.iter().fold(
                    Duration::from_secs(0),
                    |duration, data| {
                        let (start, end) = data.time_range;
    
                        duration + end.duration_since(start)
                            .unwrap_or(Duration::from_secs(0))
                    }
                )
            };
        }

        self.display = Option::<DisplayArchetype>::from(app_state);
        self.tracking_time += app_state.tracker().get_current_tracking_period();
    }

    fn render(&self, frame: &mut TUIFrame) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .split(frame.size());

        let window_info_text = (&self.display).to_widgets();

        let block = Block::default()
            .title(" Active window info ")
            .borders(Borders::ALL);
        let window_notification = Paragraph::new(window_info_text.iter())
            .block(block.clone())
            .alignment(Alignment::Left);
        
        frame.render_widget(window_notification, chunks[0]);

        let block = Block::default().title(" Block 2 ").borders(Borders::ALL);
        frame.render_widget(block, chunks[1]);
    }
}
