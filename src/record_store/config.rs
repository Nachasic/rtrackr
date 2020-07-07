use directories::ProjectDirs;
use std::path::PathBuf;

pub struct RecordStoreConfig {
    pub data_dir: PathBuf,
}

impl From<&ProjectDirs> for RecordStoreConfig {
    fn from(dirs: &ProjectDirs) -> Self {
        let data_dir = dirs.data_dir().to_owned();

        Self { data_dir }
    }
}

impl Default for RecordStoreConfig {
    fn default() -> Self {
        let data_dir = PathBuf::from("./dev-data/db_access");
        Self { data_dir }
    }
}
