use chrono::NaiveDate;
use std::path::Path;
use super::utils::{
    Database,
    create_memory_db,
    switch_db_to_date,
    RecordStoreError
};

pub struct DB {
    db: Box<Database>,
}

impl DB {
    pub fn new(db: Database) -> Result<Self, RecordStoreError> {
        let me = Self {
            db: Box::new(db),
        };
        me.db.load()?;
        Ok(me)
    }

    pub fn expose(&self) -> &Database {
        &self.db
    }

    pub fn switch_to_date(&mut self, date: &NaiveDate, dir_path: &Path) -> Result<(), RecordStoreError> {
        let memory_db = create_memory_db()?;
        self.db.save()?;
        self.db = Box::new(
            switch_db_to_date(memory_db, date, dir_path)?
        );

        self.db.load()?;
        Ok({})
    }
}

