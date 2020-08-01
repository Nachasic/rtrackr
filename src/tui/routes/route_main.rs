use crate::{
    event::Key,
    record_store::{ ActivityRecord },
    state::{ AppState },
    tui::{
        components::{ StatefulTUIComponent, TUIFrame },
        routes::{ Route },
        utils::*,
    }
};
use tui::{
    layout::{ Alignment, Constraint, Direction, Layout },
    widgets::{ Block, Borders, Paragraph },
};
use std::{
    time::Duration
};


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

    fn render(&self, frame: &mut TUIFrame, chunk: tui::layout::Rect) {
        // let chunks = Layout::default()
        //     .direction(Direction::Horizontal)
        //     .margin(1)
        //     .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        //     .split(frame.size());

        // let window_info_text = (&self.display).to_widgets();

        // let block = Block::default()
        //     .title(" Active window info ")
        //     .borders(Borders::ALL);
        // let window_notification = Paragraph::new(window_info_text.iter())
        //     .block(block.clone())
        //     .alignment(Alignment::Left);
        
        // frame.render_widget(window_notification, chunks[0]);

        // let block = Block::default().title(" Block 2 ").borders(Borders::ALL);
        // frame.render_widget(block, chunks[1]);
    }
}
