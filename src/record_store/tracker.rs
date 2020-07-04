use super::*;
use std::time::SystemTime;

#[derive(Debug)]
pub struct RecordTracker {
    current_archetype: Option<Archetype>,
    time_of_first_submission: SystemTime,
}

impl RecordTracker {
    pub fn new() -> Self {
        Self {
            current_archetype: None,
            time_of_first_submission: SystemTime::now(),
        }
    }

    pub fn ping(&mut self, arch: Archetype) -> Option<ActivityRecord> {
        let current = &self.current_archetype;

        match current {
            Some(ref cur_arch) => {
                if cur_arch != &arch {
                    let archetype = cur_arch.clone();
                    let start_time = self.time_of_first_submission.clone();
                    let record = Self::produce_record(archetype, start_time);

                    self.current_archetype = Some(arch);
                    self.time_of_first_submission = SystemTime::now();
                    return Some(record);
                } else {
                    return None;
                }
            }
            None => {
                self.current_archetype = Some(arch);
                self.time_of_first_submission = SystemTime::now();
                return None;
            }
        }
    }

    fn produce_record(archetype: Archetype, start_time: SystemTime) -> ActivityRecord {
        let end_time = SystemTime::now();

        ActivityRecord {
            archetype,
            is_productive: None,
            time_range: (start_time, end_time),
        }
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

    let report = tracker.ping(arch);

    assert!(match report {
        None => true,
        _ => false,
    })
}
