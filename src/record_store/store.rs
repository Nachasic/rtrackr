use chrono::{
    NaiveDate,
};
use rustbreak::{ 
    MemoryDatabase,
    FileDatabase,
    deser::Bincode,
    RustbreakError
};
use directories::{
    ProjectDirs
};
use super::{
    ActivityRecord,
    utils::{
        get_db_dates,
        get_dir,
        soft_push_current_date,
        create_db_for_current_date,
        EitherOrNone,
    }
};

pub struct RecordStore {
    pub available_date_records: Vec<NaiveDate>,
    db: EitherOrNone<
        Box<FileDatabase<Vec<ActivityRecord>, Bincode>>,
        Box<MemoryDatabase<Vec<ActivityRecord>, Bincode>>
    >
}

impl Default for RecordStore {
    fn default() -> Self {
        Self {
            available_date_records: vec![],
            db: EitherOrNone::None
        }
    }
}

impl RecordStore {
    pub fn with_db(&mut self) -> Result<&mut Self, RustbreakError> {
        let path = ProjectDirs::from(
            "", 
            "Immortal Science", 
            "rtrackr");
        match path {
            Some(dirs) => self.create_file_db(dirs),
            None => {
                eprintln!("{}", [
                    "Could not retrieve application data paths from OS to access database files.",
                    "Will proceed with in-memory database for now.",
                    "Your tracking data WILL NOT be saved once the application is closed.",
                    "Your OS may not be supported - plase submit an issue at",
                    "https://github.com/Nachasic/rtrackr/issues"
                ].join("\n"));
                self.create_memory_db()
            }
        }
    }

    fn create_file_db(&mut self, dirs: ProjectDirs) -> Result<&mut Self, RustbreakError> {
        let data_path = dirs.data_dir();

        match get_dir(data_path) {
            Ok(dir) => {
                self.available_date_records = get_db_dates(dir);
                self.db = EitherOrNone::Either(Box::new(
                    create_db_for_current_date(&mut self.available_date_records, data_path)?
                ));

                Ok(self)
            },
            Err(err) => {
                eprintln!("{}{}", [
                    "Could not access application's data directory to access database files.",
                    "Will proceed with in-memory database for now.",
                    "Your tracking data WILL NOT be saved once the application is closed.",
                    "If this issue persists you can report it at https://github.com/Nachasic/rtrackr/issues"
                ].join("\n"), err);
                self.create_memory_db()
            }
        }
    }

    fn create_memory_db(&mut self) -> Result<&mut Self, RustbreakError> {
        soft_push_current_date(&mut self.available_date_records);
        let db = MemoryDatabase::<Vec<ActivityRecord>, Bincode>::memory(vec![])?;
        self.db = EitherOrNone::Or(Box::new(db));
        Ok(self)
    }
}

