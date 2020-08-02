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
    widgets::{ Block, Borders, Paragraph, Chart, Axis, Dataset as TUIDataset },
    symbols,
    style::{ Style, Color },
};
use std::{
    time::Duration
};

type DataPoint = (f64, f64);
type Dataset = Vec<DataPoint>;
struct RecordProcessingResult {
    leusure_avg: DataPoint,
    prod_avg: DataPoint,
    duration: f64
}

const PROCESSING_QUEUE_SIZE: usize = 100;

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
    /// Total duration of tracking
    tracking_time: Duration,

    /// Dataset of all previous productive activities
    dataset_prod_cache: Dataset,
    /// Dataset of all previous leusire activity
    dataset_leisure_cache: Dataset,

    /// Dataset for current productive activity
    dataset_prod: Dataset,
    /// Dataset for current leisure activity
    dataset_leisure: Dataset,


    /// Number of records processed and cached
    processed_records: usize,

    /// Total amount of tracking time processed and cached
    processed_time_cached: usize,
    /// Total amount of productive time processed and cached
    total_productive_time_cache: usize,
    /// Total amount of leisure time processed and cached
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
            processed_records: 0,
            total_productive_time_cache: 0,
            total_leisure_time_cache: 0,

            dataset_leisure_cache: vec![(0.0, 0.0)],
            dataset_prod_cache: vec![(0.0, 0.0)],
            dataset_leisure: vec![],
            dataset_prod: vec![],
            processed_time_cached: 0,
        }
    }
}

impl RouteMain {
    /// Calculates an appendage to average productive dataset
    /// based on currently cached metrics
    fn process_record(&self, record: &ActivityRecord) -> RecordProcessingResult {
        let record_length_secs = record.time_range.1.duration_since(record.time_range.0)
            .map(|dur| dur.as_secs())
            .unwrap_or(0) as f64;
        let seconds_processed = self.processed_time_cached as f64;
        let new_time_point = seconds_processed + record_length_secs;

        match record.productivity {
            crate::record_store::ProductivityStatus::Leisure(_) => RecordProcessingResult {
                prod_avg: (
                    new_time_point,
                    self.total_productive_time_cache as f64 / (seconds_processed + record_length_secs),
                ),
                leusure_avg: (
                    new_time_point,
                    (self.total_leisure_time_cache as f64 + record_length_secs) / (seconds_processed + record_length_secs),
                ),
                duration: record_length_secs
            },
            crate::record_store::ProductivityStatus::Productive(_) => RecordProcessingResult {
                prod_avg: (
                    new_time_point,
                    (self.total_productive_time_cache as f64 + record_length_secs) / (seconds_processed + record_length_secs),
                ),
                leusure_avg: (
                    new_time_point,
                    self.total_leisure_time_cache as f64 / (seconds_processed + record_length_secs),
                ),
                duration: record_length_secs
            },
            _ => RecordProcessingResult {
                prod_avg: (
                    new_time_point,
                    self.total_productive_time_cache as f64 / (seconds_processed + record_length_secs),
                ),
                leusure_avg: (
                    new_time_point,
                    self.total_leisure_time_cache as f64 / (seconds_processed + record_length_secs),
                ),
                duration: record_length_secs
            }
        }
    }
}

impl StatefulTUIComponent for RouteMain {
    fn handle_key(&mut self, event: Key) {
        
    }

    fn before_render(&mut self, _: &AppState) {
        let records_available = self.records.len();
        let processed_records = self.processed_records;
        let mut queue_index: usize = 0;

        while records_available > 1
            && queue_index < PROCESSING_QUEUE_SIZE 
            && processed_records + queue_index < records_available - 1 {
                let next_record_to_process_idx = std::cmp::min(processed_records, records_available - 1);
                if let Some(record) = self.records.get(next_record_to_process_idx) {
                    let result = self.process_record(record);

                    self.dataset_prod_cache.push(result.prod_avg);
                    self.dataset_leisure_cache.push(result.leusure_avg);

                    match record.productivity {
                        crate::record_store::ProductivityStatus::Leisure(_) => self.total_leisure_time_cache += result.duration as usize,
                        crate::record_store::ProductivityStatus::Productive(_) => self.total_productive_time_cache += result.duration as usize,
                        _ => {}
                    };

                    self.processed_time_cached += result.duration as usize;
                    queue_index += 1;
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
            self.records = records;
        }
        self.tracking_time += app_state.tracker().get_current_tracking_period();
    }

    fn render(&self, frame: &mut TUIFrame, chunk: tui::layout::Rect) {
        let max_val = self.dataset_prod_cache.iter()
            .map(|val| val.1)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        
        let data = [
            TUIDataset::default()
                .name("productive")
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(
                    Color::LightMagenta
                ))
                .data(&self.dataset_prod_cache)
        ];
        let info = format!("{:?}", max_val);
        let chart_block = Block::default()
                .title(&info)
                .borders(Borders::NONE);
        let chart = Chart::default()
                .block(chart_block)
                .x_axis(
                    Axis::default()
                    .title("Tracked time")
                    .labels(&["start", "finish"])
                    .bounds([0.0, self.processed_time_cached as f64])
                )
                .y_axis(
                    Axis::default()
                    .title("Average productivity")
                    .labels(&["min", "max"])
                    .bounds([0.0, 1.2])
                )
                .datasets(&data);
        
        frame.render_widget(chart, chunk);
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
