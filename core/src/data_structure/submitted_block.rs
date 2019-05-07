extern crate ethereum_types;

use super::signed_transaction::SignedTransaction;
use ethereum_types::H256;

pub struct SubmittedBlock {
    block: Block,
    block_number: u64,
    root: H256,
}
