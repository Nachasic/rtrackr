use super::*;
use chrono::NaiveDate;
use rustbreak::{
    backend::{Backend, FileBackend, MemoryBackend},
    deser::Bincode,
    Database as RDatabase, MemoryDatabase, RustbreakError,
};
use std::{
    collections::HashMap,
    fs::{create_dir, read_dir, ReadDir},
    path::{Path, PathBuf},
};

pub type Database = RDatabase<HashMap<String, Vec<ActivityRecord>>, Box<dyn Backend>, Bincode>;

#[derive(Debug)]
pub enum RecordStoreError {
    FailedToSwitchDB,
    DBFailed(RustbreakError),
    NoDataOnDate(NaiveDate),
}

impl std::fmt::Display for RecordStoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RecordStoreError::FailedToSwitchDB => {
                write!(f, "Failed to switch database to a new file")
            }
            RecordStoreError::DBFailed(err) => std::fmt::Display::fmt(err, f),
            RecordStoreError::NoDataOnDate(date) => {
                write!(f, "Given date is not registered in the database {}", date)
            }
        }
    }
}

impl std::error::Error for RecordStoreError {
    fn description(&self) -> &str {
        match self {
            RecordStoreError::FailedToSwitchDB => "Failed to switch database to a new file",
            RecordStoreError::DBFailed(_) => "Internal DB error",
            RecordStoreError::NoDataOnDate(_) => "Given date is not registered in the database",
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }
}

impl From<RustbreakError> for RecordStoreError {
    fn from(err: RustbreakError) -> Self {
        RecordStoreError::DBFailed(err)
    }
}

/// Gets application's data directory where activity records are stored.
///
/// If such directory doesn't exist, attempts to create one
pub fn get_dir(dir_path: &Path) -> Result<ReadDir, std::io::Error> {
    Ok(match read_dir(dir_path) {
        Ok(data) => data,
        Err(_) => {
            create_dir(dir_path)?;
            read_dir(dir_path)?
        }
    })
}

pub fn get_path_for_db(dir_path: &Path) -> PathBuf {
    dir_path.join(String::from("records.db"))
}

pub fn create_memory_db() -> Result<Database, RustbreakError> {
    let db =
        MemoryDatabase::<HashMap<String, Vec<ActivityRecord>>, Bincode>::memory(HashMap::new())?;

    Ok(db.with_backend(Box::new(MemoryBackend::default())))
}

pub fn switch_db(db: Database, dir_path: &Path) -> Result<Database, RecordStoreError> {
    let path = get_path_for_db(dir_path);
    let file_existed = path.as_path().exists();
    let backend = FileBackend::open(path).map_err(|_| RecordStoreError::FailedToSwitchDB)?;
    let db: Database = db.with_backend(Box::new(backend));

    if !file_existed {
        // DB struct tries to load from file upon creation
        // which causes EOF in case of freshly-created file
        // To mitigate that - save db immediately when creating a new file
        db.save()?;
    }

    Ok(db)
}

pub fn create_file_db(dir_path: &Path) -> Result<Database, RecordStoreError> {
    let memory_db = create_memory_db()?;

    switch_db(memory_db, dir_path)
}
