use std::path::PathBuf;
use crate::constants::*;

pub struct RecordStoreConfig {
    pub data_dir: PathBuf,
}

fn get_config_from_dbg_file() -> RecordStoreConfig {
    let data_dir = PathBuf::from(DEV_DB_PATH);
    RecordStoreConfig { data_dir }
}

fn get_global_config() -> RecordStoreConfig {
    match directories::ProjectDirs::from(APP_CLASSIFIER, APP_CORP, APP_NAME) {
        Some(dirs) => RecordStoreConfig { data_dir: dirs.data_dir().to_owned() },
        None => get_config_from_dbg_file()
    }
}

impl Default for RecordStoreConfig {
    fn default() -> Self {
        #[cfg(debug_assertions)]
        { get_config_from_dbg_file() }
        
        #[cfg(not(debug_assertions))]
        { get_global_config() }
    }
}
