use super::{
    ActivityRecord,
    utils::{
        Database,
        RecordStoreError
    }
};
use chrono::{ 
    NaiveDate, 
    Local
};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
struct TrackingDate(String);
impl From<NaiveDate> for TrackingDate {
    fn from(date: NaiveDate) -> Self {
        Self(date.format("%Y-%m-%d").to_string())
    }
}

impl From<&NaiveDate> for TrackingDate {
    fn from(date: &NaiveDate) -> Self {
        Self(date.format("%Y-%m-%d").to_string())
    }
}


impl From<TrackingDate> for NaiveDate {
    fn from(date: TrackingDate) -> Self {
        NaiveDate::parse_from_str(&date.0, "%Y-%m-%d").unwrap()
    }
}


pub struct DB {
    db: Box<Database>,
}

impl DB {
    pub fn new(db: Database) -> Result<Self, RecordStoreError> {
        let me = Self {
            db: Box::new(db),
        };
        me.db.load()?;
        me.db.write(|map| {
            let today_record_date = TrackingDate::from(Local::today().naive_local());
            let has_today_records = map.contains_key(&today_record_date.0);

            if !has_today_records {
                map.insert(today_record_date.0, vec![]);
            }
        })?;
        Ok(me)
    }

    pub fn read_records<F>(&self, date: &NaiveDate, f: F) -> Result<(), RecordStoreError>
    where F: FnOnce(&Vec<ActivityRecord>) {
        let date_record = TrackingDate::from(date);

        self.db.read(|store| {
            match store.get(&date_record.0) {
                Some(data) => { f(data); Ok({}) },
                None => return Err(RecordStoreError::NoDataOnDate(date.clone()))
            }
        })??;
        self.db.save().map_err(RecordStoreError::from)
    }

    pub fn write_records<F>(&self, date: &NaiveDate, f: F) -> Result<(), RecordStoreError>
    where F: FnOnce(&mut Vec<ActivityRecord>) {
        let date_record = TrackingDate::from(date);

        self.db.write(|store| {
            if !store.contains_key(&date_record.0) {
                store.insert(date_record.0.clone(), vec![]);
            }

            match store.get_mut(&date_record.0) {
                Some(records) => f(records),
                None => unreachable!()
            };
        }).map_err(RecordStoreError::from)
    }

    pub fn get_available_dates(&self) -> Result<Vec<NaiveDate>, RecordStoreError> {
        self.db.read(|store| {
            let foo: Vec<NaiveDate> = store.keys()
                .into_iter()
                .map(|key| TrackingDate(key.clone()))
                .map(NaiveDate::from)
                .collect();

            Ok(foo)
        })?
    }
}

