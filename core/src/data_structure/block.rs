extern crate ethereum_types;

use super::signed_transaction::SignedTransaction;
use ethereum_types::H256;

pub struct Block {
    signed_transactions: Vec<SignedTransaction>,
    root: H256,
}

impl Block {
    pub fn new(signed_transactions: Vec<SignedTransaction>, root: H256) -> Block {
        return Block {
            signed_transactions: signed_transactions,
            root: root,
        };
    }
}
