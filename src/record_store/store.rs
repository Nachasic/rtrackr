use chrono::{
    NaiveDate,
};
use rustbreak::{ 
    MemoryDatabase,
    FileDatabase,
    deser::Bincode,
    RustbreakError
};
use super::{
    ActivityRecord,
    utils::{
        get_db_dates,
        get_dir,
        soft_insert_current_date,
        create_file_db_for_current_date,
        switch_db_to_date,
        EitherOr,
    }
};
use std::path::Path;
use super::config::RecordStoreConfig;

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

pub type Storage = EitherOr<
    Box<FileDatabase<Vec<ActivityRecord>, Bincode>>,
    Box<MemoryDatabase<Vec<ActivityRecord>, Bincode>>
>;

pub struct RecordStore<'a> {
    pub available_date_records: Vec<NaiveDate>,
    config: &'a RecordStoreConfig,
    db: Storage
}

impl <'a> RecordStore <'a> {
    pub fn new(config: &'a RecordStoreConfig) -> Result<Self, RecordStoreError> {
        let mut available_date_records: Vec<NaiveDate> = vec![];
        let db = Self::try_create_file_db(&mut available_date_records, config.data_dir.as_path())?;

        Ok(Self{
            config, available_date_records, db
        })
    }

    fn try_create_file_db(available_date_records: &mut Vec<NaiveDate>, data_path: &Path)
    -> Result<Storage, RecordStoreError> {
        match get_dir(data_path) {
            Ok(dir) => {
                *available_date_records = get_db_dates(dir);
                let db = EitherOr::Either(Box::new(
                    create_file_db_for_current_date(available_date_records, data_path)?
                ));

                Ok(db)
            },
            Err(err) => {
                eprintln!("{}{}", [
                    "Could not access application's data directory to access database files.",
                    "Will proceed with in-memory database for now.",
                    "Your tracking data WILL NOT be saved once the application is closed.",
                    "If this issue persists you can report it at https://github.com/Nachasic/rtrackr/issues"
                ].join("\n"), err);
                Self::create_memory_db(available_date_records).map_err(|err| RecordStoreError::from(err))
            }
        }
    }

    fn create_memory_db(available_date_records: &mut Vec<NaiveDate>) 
    -> Result<Storage, RecordStoreError> {
        soft_insert_current_date(available_date_records);
        let db = MemoryDatabase::<Vec<ActivityRecord>, Bincode>::memory(vec![])?;
        Ok(
            EitherOr::Or(Box::new(db))
        )
    }

    pub fn query_dates(&self) -> &Vec<NaiveDate> {
        &self.available_date_records
    }
    
    pub fn push_record(&self, record: ActivityRecord) -> Result<(), RecordStoreError> {
        match self.db {
            EitherOr::Either(ref db) => {
                db.write(|db| {
                    db.insert(0, record)
                })?;
                db.save()?
            },
            EitherOr::Or(ref db) => {
                db.write(|db| {
                    db.insert(0, record)
                })?;
            }
        };
        Ok(())
    }

    pub fn query_records(&self) -> Result<Vec<ActivityRecord>, RecordStoreError> {
        let current_date = self.available_date_records[0];
        let mut result: Vec<ActivityRecord> = vec![];

        match self.db {
            EitherOr::Either(ref db) => {
                db.read(|records|
                    *(&mut result) = records.clone())?;
            },
            EitherOr::Or(ref db) => {
                db.read(|records|
                    *(&mut result) = records.clone())?;
            }
        };
        Ok(result)
    }

    pub fn query_records_by_date(self, date: &NaiveDate) -> Result<Vec<ActivityRecord>, RecordStoreError> {
        let current_date = self.available_date_records[0];
        let mut result: Vec<ActivityRecord> = vec![];

        if &current_date == date {
            return self.query_records().map_err(|err| RecordStoreError::from(err))
        };

        match self.db {
            EitherOr::Either(db) => {
                let owned_db = *db;
                let switched_db = switch_db_to_date(owned_db, date, self.config.data_dir.as_path())
                    .map_err(|_| RecordStoreError::FailedToSwitchDB)?;

                switched_db.load().map_err(|_| RecordStoreError::FailedToSwitchDB)?;
                switched_db.read(|records| {
                    result = records.clone();
                }).map_err(|_| RecordStoreError::FailedToSwitchDB)?;
                Ok(result)
            },
            _ => return Err(RecordStoreError::FailedToSwitchDB)
        }
    }
}

