use crate::error::Error;
use crate::traits::db::DatabaseTrait;
use crate::traits::kvs::{Batch, KeyValueStore, KvsIterator};
use kvdb::{DBTransaction, KeyValueDB};
use kvdb_memorydb::InMemory;
#[cfg(any(linux, android))]
use kvdb_rocksdb::Database;
use crate::range::Range;

#[cfg(any(linux, android))]
pub struct RangeDb {
    db: Database,
}

#[cfg(any(linux, android))]
impl DatabaseTrait for RangeDb {
    fn open(_dbname: &str) -> Self {
        Self {
            db: Database::open_default(_dbname).unwrap(),
        }
    }
}

/// test
#[cfg(all(not(linux), not(android)))]
pub struct RangeDb {
    db: InMemory,
}

#[cfg(all(not(linux), not(android)))]
impl DatabaseTrait for RangeDb {
    fn open(_dbname: &str) -> Self {
        RangeDb {
            db: Default::default(),
        }
    }
    fn close(&self) {}
}

#[cfg(all(not(linux), not(android)))]
impl KeyValueStore for RangeDb {
    fn get(&self, start: u64, end: u64) -> Result<Vec<Range>, Error> {
        let iter = self.db.iter_from_prefix(None, start.to_be_bytes()).unwrap();
        
        let mut result = vec![];
        while {
            iter.next()
            if start < range.get_end() {
                result.push(range.clone());
                if !range.intersect(start, end) {
                    break;
                }
            }
        }
    }
    fn del(&self, start: u64, end: u64) -> Result<Vec<Range>, Error>;
    fn put(&mut self, start: u64, end: u64, value: &[u8]) -> Result<(), Error>;

    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, Error> {
        self.db
            .get(None, key)
            .map_err(|_e| Error::DB)
            .map(|v| v.map(|v| v.to_vec()))
    }
    fn put(&mut self, key: &[u8], value: &[u8]) -> Result<(), Error> {
        let mut tr = DBTransaction::new();
        tr.put(None, key, value);
        self.db.write(tr).map_err(|_e| Error::DB)
    }
    fn del(&self, key: &[u8]) -> Result<(), Error> {
        let mut tr = DBTransaction::new();
        tr.delete(None, key);
        self.db.write(tr).map_err(|_e| Error::DB)
    }
    fn has(&self, _key: &[u8]) -> Result<bool, Error> {
        Ok(true)
    }
    fn batch(&self, _operations: &[Batch]) -> Result<(), Error> {
        Ok(())
    }
    fn iterator(&self, _prefix: &[u8]) -> Result<Box<KvsIterator + 'static>, Error> {
        Err(Error::DB)
    }
}
