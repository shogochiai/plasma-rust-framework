use crate::error::Error;

pub enum Batch<K, V> {
    BatchPut { key: K, value: V },
    BatchDel { key: K },
}

pub enum IteratorOptions<K> {
    /// Greater than equal
    Gte(K),
    /// Litteler than equal
    Lte(K),
}

pub struct KeyValue<K, V> {
    key: K,
    value: V,
}

pub trait Iterator<K, V> {
    fn next(&self) -> Result<KeyValue<K, V>, Error>;
}

pub trait KeyValueStore<K, V> {
    fn get(&self, key: K) -> Result<Option<V>, Error>;
    fn put(&mut self, key: K, value: V) -> Result<(), Error>;
    fn del(&self, key: K) -> Result<(), Error>;
    fn has(&self, key: K) -> Result<bool, Error>;
    fn batch(&self, operations: &[Batch<K, V>]) -> Result<(), Error>;
    fn iterator(&self, option: IteratorOptions<K>) -> Result<Box<Iterator<K, V>>, Error>;
}
