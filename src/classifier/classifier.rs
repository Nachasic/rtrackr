use super::{
    config::Machine,
    super::record_store::{
        ActivityRecord,
        Archetype
    }
};

pub struct Classifier {
    machine: Machine
}

impl Classifier {
    pub fn classify(&self, record: &mut ActivityRecord) {
        let activities = &self.machine.activity;

        match activities {
            Some(activities) => {
                for activity in activities {
                    let productivity = activity.productivity;
                }
            },
            _ => unimplemented!()
        }
    }
}

