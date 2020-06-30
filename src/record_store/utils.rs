use std::{
    path::Path,
    cmp::Ordering,
    fs::{
        read_dir,
        create_dir,
        ReadDir
    },
    ffi::OsString
};
use chrono::{
    NaiveDate,
    ParseError
};

/// Gets application's data directory where activity records are stored.
///
/// If such directory doesn't exist, attempts to create one
pub fn get_dir(dir_path: &Path) -> Result<ReadDir, Box<dyn std::error::Error>> {
    Ok(match read_dir(dir_path) {
        Ok(data) => data,
        Err(_) => {
            create_dir(dir_path)?;
            read_dir(dir_path)?
        }
    })
}

pub fn date_from_file_name (file_name: &OsString) -> Result<NaiveDate, ParseError> {
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
                match date_from_file_name(&entry.file_name()) {
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

#[test]
fn date_from_os_string() {
    let string = OsString::from("2020-05-05");
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