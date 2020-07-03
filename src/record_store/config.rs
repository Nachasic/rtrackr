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

#[cfg(test)]
pub mod testable {
    pub use super::RecordStoreConfig;
    use std::path::PathBuf;

    impl RecordStoreConfig {
        pub fn test_instance() -> Self {
            let data_dir = PathBuf::from("./test-data/db_access");
            Self { data_dir }
        }
    }
}
