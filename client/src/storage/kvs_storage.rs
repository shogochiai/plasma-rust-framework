use super::error::Error;

// TODO: implement custom result type
pub trait KVSStorage {
    fn get(&self, key: &str) -> Result<Option<String>, Error>;
    fn set(&self, key: &str, value: &str) -> Result<(), Error>;
    fn delete(&self, key: &str) -> Result<(), Error>;
}
