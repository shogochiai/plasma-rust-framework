extern crate ethereum_types;

use super::signed_transaction::SignedTransaction;

pub struct Block {
    signed_transactions: SignedTransaction,
}
