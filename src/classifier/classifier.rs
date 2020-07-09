use super::{
    config::ClassifierConfig,
    activities::ActivityInternal,
    super::record_store::{
        ProductivityStatus,
        Archetype
    }
};

pub trait Classifiable {
    fn get_archetype(&self) -> &Archetype;
    fn assign_productivity(&mut self, productivity: ProductivityStatus);
}

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
    pub fn classify(&self, record: &mut dyn Classifiable) {
        let activities = &self.activities;
        let arch = record.get_archetype();

        match arch {
            Archetype::AFK => { record.assign_productivity(ProductivityStatus::Neutral); },
            Archetype::ActiveWindow(title, name, class) => {
                let mut some_activity_applies: bool = false;
                let mut productivity: ProductivityStatus = ProductivityStatus::Neutral;

                'activities: for activity in activities {
                    productivity = if activity.productivity > 0 {
                        ProductivityStatus::Productive(activity.name.clone())
                    } else if activity.productivity < 0 {
                        ProductivityStatus::Leisure(activity.name.clone())
                    } else {
                        ProductivityStatus::Neutral
                    };
        
                    for rule in &activity.rules {
                        if rule.apply(&name, &class, &title) {
                            some_activity_applies = true;
                            break 'activities;
                        }
                    }
                }

                if some_activity_applies {
                    record.assign_productivity(productivity);
                }
            }
        }   
    }
}

