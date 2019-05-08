extern crate ethereum_types;

use crate::data_structure::block::Block;
use crate::data_structure::error::Error;
use crate::data_structure::signed_transaction::SignedTransaction;
use ethereum_types::H256;

pub struct BlockGenerator {}

impl BlockGenerator {
    pub fn generate(signed_transactions: &[SignedTransaction]) -> Result<Block, Error> {
        // TODO: caluculate merkle root
        // copy all transactions
        Ok(Block::new(signed_transactions, H256::zero()))
    }
}
