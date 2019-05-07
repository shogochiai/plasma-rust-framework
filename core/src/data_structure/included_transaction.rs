extern crate ethereum_types;

use super::signed_transaction::SignedTransaction;
use bytes::Bytes;
use ethereum_types::H256;

pub struct IncludedTransaction {
    signed_transaction: SignedTransaction,
    root: H256,
    proof: Bytes,
    index: u8,
}
