use chrono::{ 
    Local,
    DateTime,
    NaiveDate,
    ParseError
};
use rustbreak::{ 
    Database,
    MemoryDatabase,
    FileDatabase,
    deser::Bincode
};
use directories::{
    ProjectDirs
};
use std::{
    path::Path,
    cmp::Ordering,
    fs::{
        read_dir,
        create_dir,
        ReadDir,
        DirEntry
    },
    ffi::OsString
};

pub struct RecordStore {
    is_file_db: bool,
    available_date_records: Vec<NaiveDate>
}

impl Default for RecordStore {
    fn default() -> Self {
        Self {
            is_file_db: false,
            available_date_records: vec![],
        }
    }
}

impl RecordStore {
    fn with_db(&mut self) -> &mut Self {
        let path = ProjectDirs::from(
            "", 
            "Immortal Science", 
            "rtrackr");
        match path {
            Some(dirs) => self.create_file_db(dirs),
            None => {
                eprintln!("{}", [
                    "Could not retrieve application data paths from OS.",
                    "Your OS may not be supported - plase submit an issue at",
                    "https://github.com/Nachasic/rtrackr/issues"
                ].join("\n"));
                self.create_memory_db()
            }
        }
    }

    fn create_file_db(&mut self, dirs: ProjectDirs) -> &mut Self{
        let data_path = dirs.data_dir();

        match Self::get_dir(data_path) {
            Ok(dir) => {
                self.available_date_records = Self::get_db_dates(dir);
                // create DB for current date
                self
            },
            Err(err) => {
                eprintln!("{}{}",
                    [
                        "Could not access application's data directory to create database files.",
                        "Will proceed with in-memory database for now.",
                        "Your tracking data WILL NOT be saved once the application is closed.",
                        "If this issue persists you can report it at https://github.com/Nachasic/rtrackr/issues"
                    ].join("\n"),err);
                self.create_memory_db()
            }
        }
    }

    /// Gets application's data directory where activity records are stored.
    ///
    /// If such directory doesn't exist, attempts to create one
    fn get_dir(dir_path: &Path) -> Result<ReadDir, Box<dyn std::error::Error>> {
        Ok(match read_dir(dir_path) {
            Ok(data) => data,
            Err(_) => {
                create_dir(dir_path)?;
                read_dir(dir_path)?
            }
        })
    }

    /// Compiles a list of dated DB records found in a given directory.
    ///
    /// Iterates through the files, assuming that if a given file
    /// has a date-like name - it contains records for that date.
    ///
    /// Returns a vector of dates backed-up in the filesystem, sorted
    /// sorted in order from most recent one.
    fn get_db_dates(dir: ReadDir) -> Vec<NaiveDate> {
        let mut dates: Vec<NaiveDate> = vec![];

        for result in dir {
            match result {
                Ok(entry) => {
                    match Self::date_from_file_name(&entry.file_name()) {
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

    fn date_from_file_name (file_name: &OsString) -> Result<NaiveDate, ParseError> {
        let file_name_str = &*file_name.to_string_lossy();
        NaiveDate::parse_from_str(file_name_str, "%Y-%m-%d")
    }

    fn create_memory_db(&mut self) -> &mut Self {
        self
    }
}

#[test]
fn simple_test() {
    let date_str = Local::now().date().format("%Y-%m-%d").to_string();
    let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d");

    dbg!(date_str);
    dbg!(date);
}

#[test]
fn date_sorting_test() {
    fn produce_date(date: &str) -> NaiveDate {
        NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap()
    };
    let mut dates = vec![
        produce_date("2020-06-30"),
        produce_date("2020-06-29"),
        produce_date("2020-06-12"),
        produce_date("2020-05-17"),
        produce_date("2020-05-29"),
        produce_date("2020-06-14")
    ];

    dates.sort_by(|right, left|
        if right > left {
            Ordering::Less
        } else {
            Ordering::Greater
        }
        
    );

    dbg!(dates);
}