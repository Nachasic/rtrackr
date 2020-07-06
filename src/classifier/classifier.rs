use super::{
    config::Machine,
    rules::RuleInternal,
    activities::ActivityInternal,
    super::record_store::{
        ActivityRecord,
        Archetype
    }
};

pub struct Classifier {
    machine_name: String,
    activities: Vec<ActivityInternal>
}

impl From<Machine> for Classifier {
    fn from(config: Machine) -> Self {
        Self {
            machine_name: config.name.unwrap_or(String::from("unnamed machine")),
            activities: match config.activity {
                Some(conf_acts) => {
                    let mut acts: Vec<ActivityInternal> = vec![];

                    for act in conf_acts {
                        match Option::<ActivityInternal>::from(act) {
                            Some(internal_act) => { acts.push(internal_act) },
                            None => {},
                        }
                    };

                    acts
                },
                None => vec![]
            }
        }
    }
}

impl Classifier {
    pub fn classify(&self, record: &mut ActivityRecord) {
        let activities = &self.activities;
        let arch = &record.archetype;

        match arch {
            Archetype::AFK => { record.is_productive = None; },
            Archetype::ActiveWindow(title, name, class) => {
                for activity in activities {
                    let productivity = if activity.productivity > 0 {
                        Some(true)
                    } else if activity.productivity < 0 {
                        Some(false)
                    } else {
                        None
                    };
        
                    for rule in &activity.rules {
                        if rule.apply(&name, &class, &title) {
                            record.is_productive = productivity;
                        }
                    }
                }
            }
        }   
    }
}

