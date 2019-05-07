extern crate ethereum_types;

use super::transaction::Transaction;
use bytes::Bytes;

pub struct SignedTransaction {
    transactions: [Transaction; 4],
}
