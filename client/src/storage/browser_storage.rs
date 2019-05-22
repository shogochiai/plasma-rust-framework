use super::block_header::BlockHeader;
use super::error::Error;
use super::local_storage::LocalStorage;
use super::plasma_client_storage::PlasmaClientStorage;

pub struct BrowserStorage {
    localStorage: LocalStorage,
}

impl PlasmaClientStorage for BrowserStorage {
    fn add_proof(&self, key: &str, block_number: u32, value: String) -> Result<(), Error> {
        Ok(())
    }

    fn get_proof(&self, key: &str, block_number: u32) -> Result<Option<String>, Error> {
        Ok(Some(String::from("a")))
    }

    fn add_block_header(&self, block_number: u32, value: String) -> Result<(), Error> {
        Ok(())
    }

    fn get_block_header(&self, block_number: u32) -> Result<BlockHeader, Error> {
        Ok(BlockHeader::new(block_number, String::from("a")))
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
