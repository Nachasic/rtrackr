use super::{
    config::ClassifierConfig,
    rules::RuleInternal,
    activities::ActivityInternal,
    super::record_store::{
        ActivityRecord,
        ProductivityStatus,
        Archetype
    }
};

#[derive(Debug, Default)]
pub struct Classifier {
    machine_name: String,
    activities: Vec<ActivityInternal>
}

impl From<ClassifierConfig> for Classifier {
    fn from(config: ClassifierConfig) -> Self {
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
            Archetype::AFK => { record.productivity = ProductivityStatus::Neutral; },
            Archetype::ActiveWindow(title, name, class) => {
                for activity in activities {
                    let productivity = if activity.productivity > 0 {
                        ProductivityStatus::Productive(activity.name.clone())
                    } else if activity.productivity < 0 {
                        ProductivityStatus::Leisure(activity.name.clone())
                    } else {
                        ProductivityStatus::Neutral
                    };
                    let mut some_rule_applies: bool = false;
        
                    for rule in &activity.rules {
                        if rule.apply(&name, &class, &title) {
                            some_rule_applies = true;
                            break;
                        }
                    }

                    if some_rule_applies {
                        record.productivity = productivity
                    }
                }
            }
        }   
    }
}

