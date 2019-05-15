extern crate ethereum_types;

use crate::data_structure::block::Block;
use crate::data_structure::error::Error;
use crate::data_structure::transaction::Transaction;
use ethereum_types::H256;

pub struct BlockGenerator {}

impl BlockGenerator {
    pub fn generate(transactions: &[Transaction]) -> Result<Block, Error> {
        // TODO: caluculate merkle root
        // copy all transactions
        Ok(Block::new(transactions, H256::zero()))
    }
}
