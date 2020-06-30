use std::path::{ PathBuf };
use directories::ProjectDirs;

pub struct RecordStoreConfig {
    pub data_dir: PathBuf,
}

impl From<&ProjectDirs> for RecordStoreConfig {
    fn from(dirs: &ProjectDirs) -> Self {
        let data_dir = dirs.data_dir().to_owned();

        Self {
            data_dir,
        }
    }
}

#[cfg(test)]
pub mod testable {
    use std::path::PathBuf;
    pub use super::RecordStoreConfig;

    impl RecordStoreConfig {
        pub fn test_instance() -> Self {
            let data_dir = PathBuf::from("./test-data/db_access");
            Self {
                data_dir
            }
        }
    }
}