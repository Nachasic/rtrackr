use rustbreak::{ 
    FileDatabase,
    MemoryDatabase,
    deser::Bincode,
};
use std::fmt::Debug;

use serde::Serialize;
use serde::de::DeserializeOwned;

/// Unused shit to make MemoryDatabase and FileDatabase work
/// Through the same trait. Didn't work for some reason ┐( ᐛ )┌
pub trait DB<Data>
where Data: Serialize + DeserializeOwned + Debug + Clone + Send {
    fn write<T, R>(&self, task: T) -> rustbreak::error::Result<R>
        where T: FnOnce(&mut Data) -> R;

    fn read<T, R>(&self, task: T) -> rustbreak::error::Result<R>
        where T: FnOnce(&Data) -> R;
    
    fn load(&self) -> rustbreak::error::Result<()>;

    fn save(&self) -> rustbreak::error::Result<()>;

    fn get_data(&self, load: bool) -> rustbreak::error::Result<Data>;

    fn put_data(&self, new_data: Data, save: bool) -> rustbreak::error::Result<()>;
}

impl <Data> DB<Data> for FileDatabase<Data, Bincode>
where Data: Serialize + DeserializeOwned + Debug + Clone + Send {
    fn write<T, R>(&self, task: T) -> rustbreak::error::Result<R>
        where T: FnOnce(&mut Data) -> R {
            FileDatabase::write(&self, task)
        }

    fn read<T, R>(&self, task: T) -> rustbreak::error::Result<R>
        where T: FnOnce(&Data) -> R {
            FileDatabase::read(&self, task)
        }
    
    fn load(&self) -> rustbreak::error::Result<()> {
        FileDatabase::load(&self)
    }

    fn save(&self) -> rustbreak::error::Result<()> {
        FileDatabase::save(&self)
    }

    fn get_data(&self, load: bool) -> rustbreak::error::Result<Data> {
        FileDatabase::get_data(&self, load)
    }

    fn put_data(&self, new_data: Data, save: bool) -> rustbreak::error::Result<()> {
        FileDatabase::put_data(self, new_data, save)
    }
}

impl <Data> DB<Data> for MemoryDatabase<Data, Bincode>
where Data: Serialize + DeserializeOwned + Debug + Clone + Send {
    fn write<T, R>(&self, task: T) -> rustbreak::error::Result<R>
        where T: FnOnce(&mut Data) -> R {
            MemoryDatabase::write(&self, task)
        }

    fn read<T, R>(&self, task: T) -> rustbreak::error::Result<R>
        where T: FnOnce(&Data) -> R {
            MemoryDatabase::read(&self, task)
        }
    
    fn load(&self) -> rustbreak::error::Result<()> {
        MemoryDatabase::load(&self)
    }

    fn save(&self) -> rustbreak::error::Result<()> {
        MemoryDatabase::save(&self)
    }

    fn get_data(&self, load: bool) -> rustbreak::error::Result<Data> {
        MemoryDatabase::get_data(&self, load)
    }

    fn put_data(&self, new_data: Data, save: bool) -> rustbreak::error::Result<()> {
        MemoryDatabase::put_data(self, new_data, save)
    }
}