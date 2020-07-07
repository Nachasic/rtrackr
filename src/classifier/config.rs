use toml;
use std::fs::File;

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct Rule {
    pub for_name: Option<Vec<String>>,
    pub for_class: Option<Vec<String>>,
    pub for_title: Option<Vec<String>>,

    pub title_contains_any: Option<Vec<String>>,
    pub title_contains_all: Option<Vec<String>>,
    pub title_is: Option<String>,
    pub title_starts_with: Option<String>,
    pub title_ends_with: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Activity {
    pub name: Option<String>,
    pub productivity: Option<i8>,
    pub rule: Option<Vec<Rule>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClassifierConfig {
    pub name: Option<String>,
    pub activity: Option<Vec<Activity>>
}

impl Default for ClassifierConfig {
    fn default() -> Self {
        let config = r#"
        name = "Home computer"
        
        [[activity]]
            name = "coding"
        
            # 1 = Productive, -1 = Leisure, 0 = Neutral
            productivity = 1
        
            [[activity.rule]]
                for_class = ["code-oss"]
                title_conains_any = ["rtrackr", "frontend"]
        
            [[activity.rule]]
                # Grabbing by app name
                for_name = ["Navigator", "Google-Chrome"]
        
                # Providing multiple criteria is equivalent to "OR" operation
                title_contains_any = ["rtrackr"]
                # title_contains_all = ["github"]
                # title_is = ""
                # title_ends_with = ""
                # title_starts_with = ""
        "#;
        toml::from_str(config).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_config() {
        let config = ClassifierConfig::default();
        let result_toml = toml::to_string_pretty(&config);

        assert_eq!(result_toml.is_ok(), true);
        dbg!(result_toml.unwrap());
    }
}