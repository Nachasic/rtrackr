use std::{
    path::{ Path, PathBuf },
    fs::{
        read_dir,
        create_dir,
        ReadDir
    },
    collections::HashMap,
};
use chrono::NaiveDate;
use rustbreak::{ 
    MemoryDatabase,
    deser::Bincode,
    RustbreakError,
    backend::{ FileBackend, Backend, MemoryBackend },
    Database as RDatabase
};
use super::*;

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
            RecordStoreError::FailedToSwitchDB => write!(f, "Failed to switch database to a new file"),
            RecordStoreError::DBFailed(err) => std::fmt::Display::fmt(err, f),
            RecordStoreError::NoDataOnDate(date) => write!(f, "Given date is not registered in the database {}", date),
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

pub fn create_memory_db () -> Result<Database, RustbreakError> {
    let db = MemoryDatabase::<HashMap<String, Vec<ActivityRecord>>, Bincode>::memory(HashMap::new())?;

    Ok(db.with_backend(Box::new(MemoryBackend::default())))
}

pub fn switch_db(
    db: Database,
    dir_path: &Path) -> Result<Database, RecordStoreError> {
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

        Ok(
            db
        )
    }

pub fn create_file_db(dir_path: &Path)
    -> Result<Database, RecordStoreError> {
    let memory_db = create_memory_db()?;
    
    switch_db(memory_db, dir_path)
}

#[cfg(test)]
mod tests {
    use super::{
        super::{ RecordStoreConfig },
        *
    };

    // #[test]
    // fn soft_insert_to_empty_date_vec() {
    //     let mut dates: Vec<NaiveDate> = vec![];
    //     let current_date = Local::today().naive_local();
    
    //     soft_insert_date(&mut dates, current_date);
    //     assert_eq!(dates.len(), 1);
    // }

    // #[test]
    // fn soft_insert_to_non_empty_vec() {
    //     let current_date = Local::today().naive_local();
    //     let mut dates = vec![
    //         NaiveDate::from_ymd(2020, 05, 06),
    //         NaiveDate::from_ymd(2020, 05, 05)
    //     ];
    //     soft_insert_date(&mut dates, current_date);

    //     assert_eq!(dates, vec![
    //         current_date,
    //         NaiveDate::from_ymd(2020, 05, 06),
    //         NaiveDate::from_ymd(2020, 05, 05)
    //     ])
    // }
    
    // #[test]
    // fn date_from_os_string() {
    //     let string = OsStr::new("2020-05-05");
    //     let result = date_from_file_name(&string);
    //     let expected_date = NaiveDate::from_ymd(2020, 05, 05);
    
    //     assert_eq!(result, Ok(expected_date));
    // }
    
    // #[test]
    // fn db_access() {
    //     let path = RecordStoreConfig::test_instance().data_dir;
    //     let result = get_dir(path.as_path());
    
    //     assert!(match result {
    //         Err(_) => false,
    //         _ => true
    //     })
    // }
    
    // // TODO: make this obsolete with e2e tests
    // #[test]
    // fn db_access_no_dir() {
    //     let path = Path::new("./test-data/non-existent");
    //     let result = get_dir(path);
    
    //     assert!(match result {
    //         Err(_) => false,
    //         _ => true
    //     });
    
    //     std::fs::remove_dir(path).unwrap();
    // }
    
    // #[test]
    // fn getting_dates_from_dir() {
    //     let path = RecordStoreConfig::test_instance().data_dir;
    //     let dir = get_dir(path.as_path()).unwrap();
    //     let dates = get_db_dates(dir);
    
    //     assert_eq!(dates, vec![
    //         NaiveDate::from_ymd(2020, 05, 06),
    //         NaiveDate::from_ymd(2020, 05, 05)
    //     ]);
    // }
    
    // // TODO: make this obsolete with e2e tests
    // #[test]
    // fn creating_db() {
    //     let path = RecordStoreConfig::test_instance().data_dir;
    //     let dir = get_dir(path.as_path()).unwrap();
    //     let mut dates = get_db_dates(dir);
    
    //     let current_date = Local::today().naive_local();
    //     let db_file_path = get_path_for_db(path.as_path(), &current_date);
    
    //     let db = create_file_db_for_current_date(&mut dates, path.as_path());
    
    //     assert!(match db {
    //         Ok(_) => true,
    //         Err(err) => {
    //             dbg!(err);
    //             false
    //         }
    //     });
    
    //     std::fs::remove_file(&db_file_path).unwrap();
    // }
}

