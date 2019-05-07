extern crate ethereum_types;

use super::state_update::StateUpdate;
use bytes::Bytes;

pub struct Transaction {
    state_update: StateUpdate,
    transaction_witness: Bytes,
}
