#[cfg(any(linux, android))]
use kvdb_rocksdb::Database;

// use crate::traits::kvs::KeyValueStore;

#[cfg(any(linux, android))]
pub struct CoreDb {
    db: Option<Database>,
}

#[cfg(any(linux, android))]
impl Default for CoreDb {
    fn default() -> Self {
        Self { db: None }
    }
}

#[cfg(any(linux, android))]
pub struct CoreDb {}

#[cfg(all(not(linux), not(android)))]
pub struct CoreDb {}

/*
#[cfg(linux)]
impl KeyValueStore for CoreDb {
    fn open(&mut self) {
        let tempdir = TempDir::new("demo").unwrap();
        let path = tempdir.path();

        let mut options = Options::new();
        options.create_if_missing = true;
        self.db = Database::open(path, options).ok();
    }
}
*/
