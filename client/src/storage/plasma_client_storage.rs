use super::block_header::BlockHeader;
use super::error::Error;

pub trait PlasmaClientStorage {
    fn add_proof(&self, key: &str, block_number: u32, value: String) -> Result<(), Error>;
    fn get_proof(&self, key: &str, block_number: u32) -> Result<Option<String>, Error>;
    fn add_block_header(&self, block_number: u32, value: String) -> Result<(), Error>;
    fn get_block_header(&self, block_number: u32) -> Result<BlockHeader, Error>;
    fn search_block_header(
        &self,
        from_block_number: u32,
        to_block_number: u32,
    ) -> Result<Vec<BlockHeader>, Error>;
    fn add_action(&self, id: String, block_number: u32, value: String) -> Result<(), Error>;
    fn search_actions(&self, block_number: u32) -> Result<Vec<BlockHeader>, Error>;
}
