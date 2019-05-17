extern crate ethereum_types;

use crate::data_structure::block::Block;
use crate::data_structure::error::{Error, ErrorKind};
use crate::data_structure::transaction::Transaction;
use sum_merkle_tree::{SumMerkleNode, SumMerkleTree};

pub struct BlockGenerator {}

impl BlockGenerator {
    /// Example
    /// ```
    /// use plasma_core::process::block_generator::BlockGenerator;
    /// let result = BlockGenerator::generate(&[]);
    /// ```
    pub fn generate(transactions: &[Transaction]) -> Result<Block, Error> {
        let mut leaves: Vec<SumMerkleNode> = Vec::new();
        let mut previous_end: u64 = 0;
        if transactions.is_empty() {
            return Err(ErrorKind::NoTransactions.into());
        }
        for t in transactions.iter() {
            if previous_end < t.get_start() {
                leaves.push(SumMerkleNode::create_empty_leaf(t.get_start()));
            }
            leaves.push(SumMerkleNode::create_leaf(t.get_end(), &rlp::encode(t)));
            previous_end = t.get_end()
        }
        let tree = SumMerkleTree::generate(&leaves);
        Ok(Block::new(
            // copy all transactions
            transactions,
            tree.get_root().as_slice().into(),
        ))
    }
}
