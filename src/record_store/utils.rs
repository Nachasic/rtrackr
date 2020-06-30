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
    Date,
    DateTime,
    Local,
    ParseError
};
use rustbreak::{ 
    Database,
    MemoryDatabase,
    FileDatabase,
    deser::Bincode,
    RustbreakError,
    backend::Backend
};

use super::{ ActivityRecord };

#[derive(Debug, Copy, Clone)]
pub enum EitherOrNone <T, G> {
    Either(T),
    Or(G),
    None
}

impl <T, G> EitherOrNone <T, G> {
    pub fn is_either_or(&self) -> bool {
        match self {
            EitherOrNone::None => true,
            _ => false
        }
    }

    pub fn is_none(&self) -> bool {
        !self.is_either_or()
    }

    pub fn as_ref(&self) -> EitherOrNone<&T, &G> {
        match self {
            EitherOrNone::Either(ref val) => EitherOrNone::Either(val),
            EitherOrNone::Or(ref val) => EitherOrNone::Or(val),
            _ => EitherOrNone::None
        }
    }

    pub fn as_ref_mut(&mut self) -> EitherOrNone<&mut T, &mut G> {
        match self {
            EitherOrNone::Either(ref mut val) => EitherOrNone::Either(val),
            EitherOrNone::Or(ref mut val) => EitherOrNone::Or(val),
            _ => EitherOrNone::None
        }
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

fn get_path_for_new_db(dir_path: &Path, date: &NaiveDate) -> PathBuf {
    let date_str = date.format("%Y-%m-%d").to_string();
    let file_path = format!("{}.db", date_str);

    dir_path.join(file_path)
}

pub fn soft_push_current_date(dates: &mut Vec<NaiveDate>) {
    let current_date = Local::today().naive_local();

    if dates.len() > 0 {
        let most_recent = dates[0];
        if most_recent != current_date {
            dates.push(current_date);
        }
    } else {
        dates.push(current_date);
    }
    
}

pub fn create_db_for_current_date(dates: &mut Vec<NaiveDate>, dir_path: &Path)
    -> Result<FileDatabase<Vec<ActivityRecord>, Bincode>, RustbreakError> {
    let most_recent = dates[0];
    let current_date = Local::today().naive_local();
    if most_recent != current_date {
        dates.push(current_date);
    }

    let db_path = get_path_for_new_db(dir_path, &current_date);

    FileDatabase::<Vec<ActivityRecord>, Bincode>::from_path(db_path, vec![])
}

#[test]
fn soft_push_to_empty_date_vec() {
    let mut dates: Vec<NaiveDate> = vec![];

    soft_push_current_date(&mut dates);
    assert_eq!(dates.len(), 1);
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
    let path = Path::new("./test-data/db_access");
    let result = get_dir(path);

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
    let path = Path::new("./test-data/db_access");
    let dir = get_dir(path).unwrap();
    let dates = get_db_dates(dir);

    assert_eq!(dates, vec![
        NaiveDate::from_ymd(2020, 05, 06),
        NaiveDate::from_ymd(2020, 05, 05)
    ]);
}

#[test]
fn creating_db() {
    let path = Path::new("./test-data/db_access");
    let dir = get_dir(path).unwrap();
    let mut dates = get_db_dates(dir);

    let current_date = Local::today().naive_local();
    let db_file_path = get_path_for_new_db(path, &current_date);

    let db = create_db_for_current_date(&mut dates, path);

    assert!(match db {
        Ok(_) => true,
        Err(err) => {
            dbg!(err);
            false
        }
    });

    std::fs::remove_file(&db_file_path).unwrap();
}
