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
    widgets::{ Block, Borders, Paragraph, Chart, Axis, Dataset as TUIDataset, GraphType },
    symbols,
    style::{ Style, Color },
};
use std::{
    time::Duration
};

type DataPoint = (f64, f64);
type Dataset = Vec<DataPoint>;
struct RecordProcessingResult {
    // leusure_avg: DataPoint,
    prod_avg: DataPoint,
    duration: f64
}

const ROLLING_AVERAGE_TIME_WINDOW: Duration = Duration::from_secs(5 * 60);

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
    // /// Total duration of tracking current record
    tracking_time: Duration,

    current_activity: Option<ActivityRecord>,

    /// Dataset of all previous productive activities
    dataset_prod_cache: Dataset,
    /// Dataset of all previous leusire activity
    // dataset_leisure_cache: Dataset,

    /// Dataset for current productive activity
    last_datapoint: Dataset,



    /// Number of records processed and cached
    processed_records: usize,

    /// Total amount of tracking time processed and cached
    processed_time_cached: usize,
    /// Total amount of productive time processed and cached
    total_productive_time_cache: usize,
}
impl Route for RouteMain {}

impl From<&AppState> for RouteMain {
    fn from(state: &AppState) -> Self {
        let records = state.store().query_records()
            .unwrap_or(vec![]);

        let mut result = Self {
            records,
            tracking_time: Duration::from_secs(0),
            processed_records: 0,
            total_productive_time_cache: 0,
            current_activity: None,
            // total_leisure_time_cache: 0,

            // dataset_leisure_cache: vec![(0.0, 0.0)],
            dataset_prod_cache: vec![(0.0, 0.0)],
            // dataset_leisure: vec![],
            last_datapoint: vec![],
            processed_time_cached: 0,
        };

        result.update_productivity_dataset();
        result.update_last_datapoint();
        result
    }
}

impl RouteMain {
    fn last_records_in_duration(&self, target_duration: Duration) -> Vec<ActivityRecord> {
        let mut result: Vec<ActivityRecord> = vec![];
        let mut records = self.records.iter().rev();
        let mut duration = Duration::from_secs(0);

        while let Some(record) = records.next() {
            if  target_duration < duration { break };

            result.push(record.clone());
            duration += record.duration();
        };
        result.reverse();
        result
    }

    /// Calculates an appendage to average productive dataset
    /// based on currently cached metrics
    fn process_record(&self, record: &ActivityRecord) -> RecordProcessingResult {
        let record_length_secs = std::cmp::max(record.duration().as_secs(), 1) as f64;
        let seconds_processed = self.processed_time_cached as f64;
        let new_time_point = seconds_processed + record_length_secs;

        match record.productivity {
            crate::record_store::ProductivityStatus::Productive(_) => RecordProcessingResult {
                prod_avg: (
                    new_time_point,
                    (self.total_productive_time_cache as f64 + record_length_secs) / (seconds_processed + record_length_secs),
                ),
                duration: record_length_secs
            },
            _ => RecordProcessingResult {
                prod_avg: (
                    new_time_point,
                    self.total_productive_time_cache as f64 / (seconds_processed + record_length_secs),
                ),
                duration: record_length_secs
            }
        }
    }

    fn reset_cache(&mut self) {
        self.processed_time_cached = 0;
        self.total_productive_time_cache = 0;
        self.processed_records = 0;
        self.dataset_prod_cache = vec![];
        self.last_datapoint = vec![];
    }
    
    fn update_productivity_dataset(&mut self) {
        self.reset_cache();
        let last_records = self.last_records_in_duration(ROLLING_AVERAGE_TIME_WINDOW - self.tracking_time);

        for record in last_records {
            let result = self.process_record(&record);

            self.dataset_prod_cache.push(result.prod_avg);
            self.processed_time_cached += result.duration as usize;

            match record.productivity {
                crate::record_store::ProductivityStatus::Productive(_) => self.total_productive_time_cache += result.duration as usize,
                _ => {}
            }
        }

        self.update_last_datapoint();
    }

    fn update_last_datapoint(&mut self) {
        if let Some(ref record) = self.current_activity {
            let result = self.process_record(record);
            self.last_datapoint.push(result.prod_avg);
        }
    }

    fn get_cached_dataset_duration(&self) -> Duration {
        Duration::from_secs_f64(self.dataset_prod_cache.iter()
            .fold(0.0, |acc, (point, _)| acc + point)
        )       
    }
}

impl StatefulTUIComponent for RouteMain {
    fn tick(&mut self, app_state: &AppState) {
        let records = app_state.store().query_records()
            .unwrap_or(vec![]);

        self.tracking_time = app_state.tracker().get_current_tracking_period();
        
        if let Some(arch) = app_state.get_current_archetype() {
            let end_time = std::time::SystemTime::now();
            let start_time = end_time.checked_sub(self.tracking_time).unwrap();
            let mut activity = ActivityRecord {
                archetype: arch.clone(),
                time_range: (start_time, end_time),
                productivity: crate::record_store::ProductivityStatus::Neutral
            };

            app_state.classifier().classify(&mut activity);
            self.current_activity = Some(activity);
        } else {
            self.current_activity = None;
        }
        
        if records.len() != self.records.len() {
            self.records = records;
            self.update_productivity_dataset();
        } else if self.get_cached_dataset_duration() + self.tracking_time > ROLLING_AVERAGE_TIME_WINDOW {
            self.update_productivity_dataset();
        } else {
            self.update_last_datapoint();
        }
    }

    fn render(&self, frame: &mut TUIFrame, chunk: tui::layout::Rect) {
        let all_data: Vec<(f64, f64)> = self.dataset_prod_cache.iter()
            .chain(&self.last_datapoint)
            .cloned()
            .collect();

        let data = [
            TUIDataset::default()
                .name("productive")
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(
                    Color::LightMagenta
                ))
                .graph_type(GraphType::Line)
                .data(
                    &all_data
                )
        ];
        let chart_block = Block::default()
                .borders(Borders::NONE);
        let chart = Chart::default()
                .block(chart_block)
                .x_axis(
                    Axis::default()
                    .title("Time")
                    .labels(&["5 minutes ago", "now"])
                    .bounds([0.0, ROLLING_AVERAGE_TIME_WINDOW.as_secs_f64() + 120.0])
                )
                .y_axis(
                    Axis::default()
                    .title("Productivity")
                    .labels(&["0%", "100%"])
                    .bounds([0.0, 1.2])
                )
                .datasets(&data);
        
        frame.render_widget(chart, chunk);
    }
}
