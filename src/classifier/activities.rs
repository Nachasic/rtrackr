use super::{
    config::{
        Activity
    },
    rules::RuleInternal,
};

#[derive(Debug, Default)]
pub struct ActivityInternal {
    pub name: String,
    pub productivity: i8,
    pub rules: Vec<RuleInternal>
}

impl From<Activity> for Option<ActivityInternal> {
    fn from(act_conf: Activity) -> Option<ActivityInternal> {

        let rules = match act_conf.rule {
            Some(raw_rules) => {
                let mut rules_vec: Vec<RuleInternal> = vec![];

                for raw_rule in raw_rules {
                    match Option::<RuleInternal>::from(raw_rule) {
                        Some(rule) => rules_vec.push(rule),
                        _ => {}
                    }
                };

                rules_vec
            },
            None => vec![]
        };
     
        Some(ActivityInternal {
            name: (&act_conf.name?).clone(),
            productivity: act_conf.productivity.unwrap_or(0),
            rules
        })
    }
}