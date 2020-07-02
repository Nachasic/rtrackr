use chrono::{
    NaiveDate,
    Local,
};
use super::{
    ActivityRecord,
    utils::{
        get_db_dates,
        get_dir,
        soft_insert_date,
        create_file_db_for_current_date,
        create_memory_db,
        RecordStoreError,
        Database
    }
};
use std::path::Path;
use super::config::RecordStoreConfig;
use super::db::DB;

pub struct RecordStore<'a> {
    pub available_date_records: Vec<NaiveDate>,
    pub current_date: NaiveDate,
    config: &'a RecordStoreConfig,
    db: DB
}

impl <'a> RecordStore <'a> {
    pub fn new(config: &'a RecordStoreConfig) -> Result<Self, RecordStoreError> {
        let mut available_date_records: Vec<NaiveDate> = vec![];
        let current_date = Local::today().naive_local();
        let db = Self::try_create_file_db(&mut available_date_records, config.data_dir.as_path())?;

        Ok(Self{
            config, available_date_records, db: DB::new(db)?, current_date
        })
    }

    fn try_create_file_db(available_date_records: &mut Vec<NaiveDate>, data_path: &Path)
    -> Result<Database, RecordStoreError> {
        match get_dir(data_path) {
            Ok(dir) => {
                *available_date_records = get_db_dates(dir);
                create_file_db_for_current_date(available_date_records, data_path)
                    .or(Self::create_memory_db(available_date_records))
            },
            Err(err) => {
                eprintln!("{}{}", [
                    "Could not access application's data directory to access database files.",
                    "Will proceed with in-memory database for now.",
                    "Your tracking data WILL NOT be saved once the application is closed.",
                    "If this issue persists you can report it at https://github.com/Nachasic/rtrackr/issues"
                ].join("\n"), err);
                Self::create_memory_db(available_date_records)
            }
        }
    }

    fn create_memory_db(available_date_records: &mut Vec<NaiveDate>) 
    -> Result<Database, RecordStoreError> {
        let current_date = Local::today().naive_local();
        soft_insert_date(available_date_records, current_date);
        let db = create_memory_db()?;
        Ok(
            db
        )
    }

    pub fn query_dates(&self) -> Vec<NaiveDate> {
        self.available_date_records.clone()
    }
    
    pub fn push_record(&self, record: ActivityRecord) -> Result<(), RecordStoreError> {
        let db = self.db.expose();

        db.write(|db| {
            db.insert(0, record)
        })?;
        db.save()?;

        Ok(())
    }

    pub fn query_records(&self) -> Result<Vec<ActivityRecord>, RecordStoreError> {
        let current_date = self.available_date_records[0];
        let mut result: Vec<ActivityRecord> = vec![];
        let db = self.db.expose();

        db.read(|records|
            *(&mut result) = records.clone())?;


        Ok(result)
    }

    pub fn switch_to_date(&mut self, date: &NaiveDate) -> Result<(), RecordStoreError> {
        let dir_path = self.config.data_dir.as_path();
        
        if &self.current_date == date {
            return Ok({})
        };

        self.db.switch_to_date(date, dir_path)?;
        self.current_date = date.clone();
        Ok({})
    }

    // /// TODO: Wrap DB into a struct that is going to own it
    pub fn query_records_by_date(&mut self, date: &NaiveDate) -> Result<Vec<ActivityRecord>, RecordStoreError> {
        let current_date = self.current_date;
        let mut result: Vec<ActivityRecord> = vec![];
        
        self.switch_to_date(date)?;
        result = Self::query_records(&self)?;
        self.switch_to_date(&current_date)?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ActivityRecord,
        RecordStore,
        super::{
            utils::{ get_path_for_db },
            config::{
                testable::RecordStoreConfig
            },
            Archetype
        },
    };
    use chrono::Local;

    fn remove_today_db_file()  {
        let path = RecordStoreConfig::test_instance().data_dir;
        let current_date = Local::today().naive_local();
        let db_file_path = get_path_for_db(path.as_path(), &current_date);
        std::fs::remove_file(&db_file_path).unwrap();
    }

    fn create_afk_records(n: usize) -> Vec<ActivityRecord> {
        let mut result: Vec<ActivityRecord> = Vec::with_capacity(n);
        let time = std::time::SystemTime::now();

        for _ in 0..n {
            result.push(ActivityRecord {
                time_range: (time.clone(), time.clone()),
                archetype: Archetype::AFK
            })
        };
        result
    }

    #[test]
    fn store_creation() {
        let records = create_afk_records(10);
        let config = RecordStoreConfig::test_instance();
        let store = RecordStore::new(&config).unwrap();

        for record in &records {
            store.push_record(record.clone()).unwrap();
        }
        let result = store.query_records().unwrap();

        assert_eq!(records.len(), result.len());
        remove_today_db_file();
    }

    #[test]
    fn date_querying() {
        let records = create_afk_records(10);
        let config = RecordStoreConfig::test_instance();
        let mut store = RecordStore::new(&config).unwrap();

        for record in &records {
            store.push_record(record.clone()).unwrap();
        }

        let dates = store.query_dates();
        assert_eq!(dates.len(), 3);

        let old_records = store.query_records_by_date(&dates[2]).unwrap();
        assert_eq!(old_records.len(), 1);

        let result = store.query_records().unwrap();
        assert_eq!(result.len(), records.len());

        remove_today_db_file();
    }
}