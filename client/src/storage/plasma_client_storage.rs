use super::error::Error;
use super::kvs_storage::KVSStorage;

pub struct BlockHeader {
    block_number: u32,
    value: String,
}

impl BlockHeader {
    pub fn new(block_number: u32, value: String) -> BlockHeader {
        BlockHeader {
            block_number,
            value,
        }
    }
}

pub trait PlasmaClientStorage<T: KVSStorage> {
    fn new(storage: T) -> Self;
    fn add_proof(&self, key: &str, block_number: u32, value: String) -> Result<(), Error>;
    fn get_proof(&self, key: &str, block_number: u32) -> Result<Option<String>, Error>;
    fn add_block_header(&self, block_number: u32, value: String) -> Result<(), Error>;
    fn get_block_header(&self, block_number: u32) -> Result<String, Error>;
    fn search_block_header(
        &self,
        from_block_number: u32,
        to_block_number: u32,
    ) -> Result<Vec<BlockHeader>, Error>;
    fn add_action(&self, id: String, block_number: u32, value: String) -> Result<(), Error>;
    fn search_actions(&self, block_number: u32) -> Result<Vec<BlockHeader>, Error>;
}

pub struct ClientStorage<T: KVSStorage> {
    kvs_storage: T,
}

impl<T: KVSStorage> PlasmaClientStorage<T> for ClientStorage<T> {
    fn new(kvs_storage: T) -> Self {
        ClientStorage { kvs_storage }
    }

    fn add_proof(&self, key: &str, block_number: u32, value: String) -> Result<(), Error> {
        Ok(())
    }

    fn get_proof(&self, key: &str, block_number: u32) -> Result<Option<String>, Error> {
        Ok(Some(String::from("a")))
    }

    fn add_block_header(&self, block_number: u32, value: String) -> Result<(), Error> {
        Ok(())
    }

    fn get_block_header(&self, block_number: u32) -> Result<String, Error> {
        Ok(String::from("a"))
    }

    fn search_block_header(
        &self,
        from_block_number: u32,
        to_block_number: u32,
    ) -> Result<Vec<BlockHeader>, Error> {
        let b = BlockHeader::new(0, String::from("i"));
        let mut v = Vec::new();
        v.push(b);
        Ok(v)
    }

    fn add_action(&self, id: String, block_number: u32, value: String) -> Result<(), Error> {
        Ok(())
    }

    fn search_actions(&self, block_number: u32) -> Result<Vec<BlockHeader>, Error> {
        let b = BlockHeader::new(0, String::from("i"));
        let mut v = Vec::new();
        v.push(b);
        Ok(v)
    }
}
