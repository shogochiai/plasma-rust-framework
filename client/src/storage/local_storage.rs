extern crate web_sys;

use super::error::{Error, ErrorKind};
use super::kvs_storage::KVSStorage;

pub struct LocalStorage {
    storage: web_sys::Storage,
}

impl LocalStorage {
    pub fn new() -> Option<LocalStorage> {
        let window = web_sys::window()?;
        if let Ok(Some(local_storage)) = window.local_storage() {
            Some(LocalStorage {
                storage: local_storage,
            })
        } else {
            None
        }
    }
}

impl KVSStorage for LocalStorage {
    fn get(&self, key: &str) -> Result<Option<String>, Error> {
        match self.storage.get(key) {
            Ok(Some(val)) => Ok(Some(val)),
            Ok(None) => Ok(None),
            Err(_) => Err(Error::from(ErrorKind::Io)), // TODO: error
        }
    }

    fn set(&self, key: &str, value: &str) -> Result<(), Error> {
        match self.storage.set(key, value) {
            Ok(()) => Ok(()),
            Err(_) => Err(Error::from(ErrorKind::Io)), // TODO: error
        }
    }

    fn delete(&self, key: &str) -> Result<(), Error> {
        match self.storage.delete(key) {
            Ok(()) => Ok(()),
            Err(_) => Err(Error::from(ErrorKind::Io)), // TODO: error
        }
    }
}
