use super::config::RecordStoreConfig;
use super::db::DB;
use super::{
    utils::{create_file_db, create_memory_db, get_dir, Database, RecordStoreError},
    ActivityRecord,
};
use chrono::{Local, NaiveDate};
use std::path::Path;

pub struct RecordStore<'a> {
    config: &'a RecordStoreConfig,
    db: DB,
}

impl<'a> RecordStore<'a> {
    pub fn new(config: &'a RecordStoreConfig) -> Result<Self, RecordStoreError> {
        let db = Self::try_create_file_db(config.data_dir.as_path())?;

        Ok(Self {
            config,
            db: DB::new(db)?,
        })
    }

    fn try_create_file_db(data_path: &Path) -> Result<Database, RecordStoreError> {
        match get_dir(data_path) {
            Ok(_) => create_file_db(data_path).or(Self::create_memory_db()),
            Err(err) => {
                eprintln!("{}{}", [
                    "Could not access application's data directory to access database files.",
                    "Will proceed with in-memory database for now.",
                    "Your tracking data WILL NOT be saved once the application is closed.",
                    "If this issue persists you can report it at https://github.com/Nachasic/rtrackr/issues"
                ].join("\n"), err);
                Self::create_memory_db()
            }
        }
    }

    fn create_memory_db() -> Result<Database, RecordStoreError> {
        create_memory_db().map_err(RecordStoreError::from)
    }

    pub fn query_dates(&self) -> Result<Vec<NaiveDate>, RecordStoreError> {
        self.db.get_available_dates()
    }

    pub fn push_record(&self, record: ActivityRecord) -> Result<(), RecordStoreError> {
        let current_date = Local::today().naive_local();
        self.db
            .write_records(&current_date, |records| records.push(record))?;

        Ok(())
    }

    pub fn query_records(&self) -> Result<Vec<ActivityRecord>, RecordStoreError> {
        let current_date = Local::today().naive_local();
        let mut result: Vec<ActivityRecord> = vec![];

        self.db
            .read_records(&current_date, |records| *(&mut result) = records.clone())?;

        Ok(result)
    }

    pub fn query_records_by_date(
        &mut self,
        date: &NaiveDate,
    ) -> Result<Vec<ActivityRecord>, RecordStoreError> {
        let mut result: Vec<ActivityRecord> = vec![];

        self.db
            .read_records(date, |records| *(&mut result) = records.clone())?;

        Ok(result)
    }
}
