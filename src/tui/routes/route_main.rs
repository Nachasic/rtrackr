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
    records: Vec<ActivityRecord>,
    tracking_time: Duration,

    tracking_data_cache: Vec<(f32, f32)>,
    processed_records: usize,
    total_productive_time_cache: usize,
    total_leisure_time_cache: usize
}
impl Route for RouteMain {}

impl From<&AppState> for RouteMain {
    fn from(state: &AppState) -> Self {
        let mut records = state.store().query_records()
            .unwrap_or(vec![]);
        records.reverse();

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
            records,
            tracking_time,
            tracking_data_cache: vec![],
            processed_records: 0,
            total_productive_time_cache: 0,
            total_leisure_time_cache: 0
        }
    }
}

impl RouteMain {
    fn process_record_at_idx(&self, record: &ActivityRecord) -> (
        Vec<(f32, f32)>,
        usize, usize
    ) {
        let processing_duration = record.time_range.1.duration_since(record.time_range.0)
            .map(|dur| dur.as_secs())
            .unwrap_or(0);
        let seconds_processed = self.tracking_data_cache.len();
        let mut tracking_data: Vec<(f32, f32)> = vec![];
        let mut leisure_inc = 0;
        let mut productive_inc = 0;

        for n in 1..processing_duration as usize {
            match record.productivity {
                crate::record_store::ProductivityStatus::Leisure(_) => leisure_inc += 1,
                crate::record_store::ProductivityStatus::Productive(_) => productive_inc += 1,
                _ => {}
            }

            tracking_data.push((
                (self.total_productive_time_cache + productive_inc) as f32 / (seconds_processed + n) as f32,
                (self.total_leisure_time_cache + leisure_inc) as f32 / (seconds_processed + n) as f32
            ))
        };

        (tracking_data, productive_inc, leisure_inc)
    }
}

impl StatefulTUIComponent for RouteMain {
    fn handle_key(&mut self, event: Key) {
        
    }

    fn before_render(&mut self, _: &AppState) {
        let records_available = self.records.len();
        let processed_records = self.processed_records;

        if processed_records < records_available - 1 {
            let next_record_to_process_idx = std::cmp::min(processed_records, records_available - 1);

            if let Some(record) = self.records.get(next_record_to_process_idx) {
                let (mut new_chunk, prod_inc, leisure_inc) = self.process_record_at_idx(record);
                self.processed_records += 1;
                self.tracking_data_cache.append(&mut new_chunk);
                self.total_productive_time_cache += prod_inc;
                self.total_leisure_time_cache += leisure_inc;
            }
        }
    }

    fn tick(&mut self, app_state: &AppState) {
        let mut records = app_state.store().query_records()
            .unwrap_or(vec![]);
        records.reverse();
        
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
