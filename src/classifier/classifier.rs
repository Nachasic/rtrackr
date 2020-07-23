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
    pub afk_timeout: std::time::Duration,
    machine_name: String,
    activities: Vec<ActivityInternal>
}

impl From<ClassifierConfig> for Classifier {
    fn from(config: ClassifierConfig) -> Self {
        Self {
            afk_timeout: config.afk_interval.map_or(
                std::time::Duration::from_secs(75),
                |secs| std::time::Duration::from_secs(secs)
            ),
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
                let mut productivity: ProductivityStatus = ProductivityStatus::Neutral;

                for activity in activities {
                    let act_prod = if activity.productivity > 0 {
                        ProductivityStatus::Productive(activity.name.clone())
                    } else if activity.productivity < 0 {
                        ProductivityStatus::Leisure(activity.name.clone())
                    } else {
                        ProductivityStatus::Neutral
                    };
        
                    'rules: for rule in &activity.rules {
                        if rule.apply(&name, &class, &title) {
                            productivity = act_prod;
                            break 'rules;
                        }
                    }
                }

                record.assign_productivity(productivity);
            }
        }   
    }
}

