use toml;
use std::{
    fs::File,
    io::Read
};
use crate::constants::*;

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
    pub afk_interval: Option<u64>,
    pub activity: Option<Vec<Activity>>
}

fn get_config_from_dbg_file() -> ClassifierConfig {
        let mut config = String::default();
        let mut config_file = File::open(DEV_CONFIG_PATH)
            .expect(&format!("Could not open dev config file at {}", DEV_CONFIG_PATH));
        
        config_file.read_to_string(&mut config).unwrap();
        toml::from_str(&config).unwrap()
}

fn get_global_config () -> ClassifierConfig {
    match directories::ProjectDirs::from(APP_CLASSIFIER, APP_CORP, APP_NAME) {
        Some(dirs) => {
            let config_path = dirs.data_dir().join("config.toml");
            let mut config: String = String::default();

            match File::open(&config_path) {
                Ok(ref mut file) => {
                    file.read_to_string(&mut config).unwrap();
                    toml::from_str(&config)
                        .expect(&format!("Could not parse config file at {:?}", &config_path))
                },
                _ => get_config_from_dbg_file()
            }
        },
        _ => get_config_from_dbg_file()
    }
}

impl Default for ClassifierConfig {
    fn default() -> Self {
        #[cfg(debug_assertions)]
        { get_config_from_dbg_file() }
        
        #[cfg(not(debug_assertions))]
        { get_global_config() }
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