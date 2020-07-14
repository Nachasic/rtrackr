use super::*;
use std::time::{
    SystemTime, Duration
};

#[derive(Debug)]
pub struct RecordTracker {
    time_of_first_submission: SystemTime,
    current_archetype: Option<Archetype>
}

impl RecordTracker {
    pub fn new() -> Self {
        Self {
            time_of_first_submission: SystemTime::now(),
            current_archetype: None
        }
    }

    pub fn get_current_archetype(&self) -> &Option<Archetype> {
        &self.current_archetype
    }

    pub fn ping(&mut self, arch: Option<Archetype>) -> Option<ActivityRecord> {
        let current = &self.current_archetype;

        if match (current, &arch)  {
            (Some(current_arch), Some(in_arch)) => current_arch != in_arch,
            (None, None) => false,
            _ => true
        } {
            let start_time = self.time_of_first_submission.clone();
            let mut result: Option<ActivityRecord> = None;

            match (current, &arch) {
                (Some(cur_arch), Some(_)) |
                (Some(cur_arch), None) => {
                    result = Some(
                        Self::produce_record(cur_arch.clone(), start_time)
                    )
                },
                (None, Some(_)) => {},
                _ => unreachable!()
            }
            
            self.current_archetype = arch;
            self.time_of_first_submission = SystemTime::now();
            result
        } else {
            None
        }
    }

    fn produce_record(archetype: Archetype, start_time: SystemTime) -> ActivityRecord {
        let end_time = SystemTime::now();

        ActivityRecord {
            archetype,
            productivity: ProductivityStatus::Neutral,
            time_range: (start_time, end_time),
        }
    }

    pub fn get_current_tracking_period(&self) -> Duration {
        SystemTime::now().duration_since(self.time_of_first_submission).unwrap_or(Duration::from_secs(0))
    }
}

#[test]
fn report_production() {
    let mut tracker = RecordTracker::new();
    let arch = Archetype::ActiveWindow(
        String::from("title"),
        String::from("my_app"),
        String::from("basic app"),
    );
    let arch2 = (&arch).clone();

    let report1 = tracker.ping(Some(arch));
    assert_eq!(report1.is_some(), false);

    let report2 = tracker.ping(None);
    assert_eq!(report2.is_some(), true);

    let report3 = tracker.ping(None);
    assert_eq!(report3.is_some(), false);

    let report4 = tracker.ping(Some(Archetype::AFK));
    assert_eq!(report4.is_some(), false);

    let report5 = tracker.ping(Some(arch2));
    assert_eq!(report5.is_some(), true);

    let report6 = tracker.ping(None);
    assert_eq!(report6.is_some(), true);
}
