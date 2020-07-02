use std::{
    path::{ Path, PathBuf },
    cmp::Ordering,
    fs::{
        read_dir,
        create_dir,
        ReadDir
    },
    ffi::{ OsStr },
};
use chrono::{
    NaiveDate,
    Local,
    ParseError
};
use rustbreak::{ 
    MemoryDatabase,
    deser::Bincode,
    RustbreakError,
    backend::{ FileBackend, Backend, MemoryBackend },
    Database as RDatabase
};

use super::{ ActivityRecord };

pub type Database = RDatabase<Vec<ActivityRecord>, Box<dyn Backend>, Bincode>;

#[derive(Debug)]
pub enum RecordStoreError {
    FailedToSwitchDB,
    DBFailed(RustbreakError)
}

impl std::fmt::Display for RecordStoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RecordStoreError::FailedToSwitchDB => write!(f, "Failed to switch database to a new file"),
            RecordStoreError::DBFailed(err) => std::fmt::Display::fmt(err, f),
        }
    }
}

impl std::error::Error for RecordStoreError {
    fn description(&self) -> &str {
        match self {
            RecordStoreError::FailedToSwitchDB => "Failed to switch database to a new file",
            RecordStoreError::DBFailed(_) => "Internal DB error"
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

pub fn date_from_file_name (file_name: &OsStr) -> Result<NaiveDate, ParseError> {
    let file_name_str = &*file_name.to_string_lossy();
    NaiveDate::parse_from_str(file_name_str, "%Y-%m-%d")
}

/// Compiles a list of dated DB records found in a given directory.
///
/// Iterates through the files, assuming that if a given file
/// has a date-like name - it contains records for that date.
///
/// Returns a vector of dates backed-up in the filesystem, sorted
/// sorted in order from most recent one.
pub fn get_db_dates(dir: ReadDir) -> Vec<NaiveDate> {
    let mut dates: Vec<NaiveDate> = vec![];

    for result in dir {
        match result {
            Ok(entry) => {
                let path = entry.path();
                let name = path.file_stem().unwrap();
                match date_from_file_name(name) {
                    Ok(date) => { dates.push(date); },
                    _ => {}
                }
            },
            _ => {}
        }
    }

    // Sort available dates in order from most recent to least recent
    dates.sort_by(|right, left|
        if right > left {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    );
    dates
}

pub fn get_path_for_db(dir_path: &Path, date: &NaiveDate) -> PathBuf {
    let date_str = date.format("%Y-%m-%d").to_string();
    let file_path = format!("{}.db", date_str);

    dir_path.join(file_path)
}

pub fn soft_insert_date(dates: &mut Vec<NaiveDate>, date: NaiveDate) {
    if dates.len() > 0 {
        let most_recent = dates[0];
        if most_recent != date {
            dates.insert(0, date);
        }
    } else {
        dates.push(date);
    }
    
}

pub fn create_memory_db () -> Result<Database, RustbreakError> {
    let db = MemoryDatabase::<Vec<ActivityRecord>, Bincode>::memory(vec![])?;

    Ok(db.with_backend(Box::new(MemoryBackend::default())))
}

pub fn switch_db_to_date(
    db: Database,
    date: &NaiveDate,
    dir_path: &Path) -> Result<Database, RecordStoreError> {
        let path = get_path_for_db(dir_path, date);
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

pub fn create_file_db_for_current_date(dates: &mut Vec<NaiveDate>, dir_path: &Path)
    -> Result<Database, RecordStoreError> {
    let current_date = Local::today().naive_local();
    let memory_db = create_memory_db()?;
    
    soft_insert_date(dates, current_date);
    switch_db_to_date(memory_db, &current_date, dir_path)
}

#[cfg(test)]
mod tests {
    use super::{
        super::{ RecordStoreConfig },
        *
    };

    #[test]
    fn soft_insert_to_empty_date_vec() {
        let mut dates: Vec<NaiveDate> = vec![];
        let current_date = Local::today().naive_local();
    
        soft_insert_date(&mut dates, current_date);
        assert_eq!(dates.len(), 1);
    }

    #[test]
    fn soft_insert_to_non_empty_vec() {
        let current_date = Local::today().naive_local();
        let mut dates = vec![
            NaiveDate::from_ymd(2020, 05, 06),
            NaiveDate::from_ymd(2020, 05, 05)
        ];
        soft_insert_date(&mut dates, current_date);

        assert_eq!(dates, vec![
            current_date,
            NaiveDate::from_ymd(2020, 05, 06),
            NaiveDate::from_ymd(2020, 05, 05)
        ])
    }
    
    #[test]
    fn date_from_os_string() {
        let string = OsStr::new("2020-05-05");
        let result = date_from_file_name(&string);
        let expected_date = NaiveDate::from_ymd(2020, 05, 05);
    
        assert_eq!(result, Ok(expected_date));
    }
    
    #[test]
    fn db_access() {
        let path = RecordStoreConfig::test_instance().data_dir;
        let result = get_dir(path.as_path());
    
        assert!(match result {
            Err(_) => false,
            _ => true
        })
    }
    
    #[test]
    fn db_access_no_dir() {
        let path = Path::new("./test-data/non-existent");
        let result = get_dir(path);
    
        assert!(match result {
            Err(_) => false,
            _ => true
        });
    
        std::fs::remove_dir(path).unwrap();
    }
    
    #[test]
    fn getting_dates_from_dir() {
        let path = RecordStoreConfig::test_instance().data_dir;
        let dir = get_dir(path.as_path()).unwrap();
        let dates = get_db_dates(dir);
    
        assert_eq!(dates, vec![
            NaiveDate::from_ymd(2020, 05, 06),
            NaiveDate::from_ymd(2020, 05, 05)
        ]);
    }
    
    #[test]
    fn creating_db() {
        let path = RecordStoreConfig::test_instance().data_dir;
        let dir = get_dir(path.as_path()).unwrap();
        let mut dates = get_db_dates(dir);
    
        let current_date = Local::today().naive_local();
        let db_file_path = get_path_for_db(path.as_path(), &current_date);
    
        let db = create_file_db_for_current_date(&mut dates, path.as_path());
    
        assert!(match db {
            Ok(_) => true,
            Err(err) => {
                dbg!(err);
                false
            }
        });
    
        std::fs::remove_file(&db_file_path).unwrap();
    }
}

