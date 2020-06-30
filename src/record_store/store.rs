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
use super::utils;

pub struct RecordStore {
    pub is_file_db: bool,
    pub available_date_records: Vec<NaiveDate>,
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
                    "Could not retrieve application data paths from OS to access database files.",
                    "Will proceed with in-memory database for now.",
                    "Your OS may not be supported - plase submit an issue at",
                    "https://github.com/Nachasic/rtrackr/issues"
                ].join("\n"));
                self.create_memory_db()
            }
        }
    }

    fn create_file_db(&mut self, dirs: ProjectDirs) -> &mut Self{
        let data_path = dirs.data_dir();

        match utils::get_dir(data_path) {
            Ok(dir) => {
                self.available_date_records = utils::get_db_dates(dir);
                // create DB for current date
                self
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

    fn create_memory_db(&mut self) -> &mut Self {
        self
    }
}

