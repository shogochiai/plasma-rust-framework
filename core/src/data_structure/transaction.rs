extern crate ethereum_types;
extern crate rlp;

use super::state_update::StateUpdate;
use bytes::Bytes;
use rlp::{Decodable, DecoderError, Encodable, Rlp, RlpStream};

#[derive(Clone, Debug)]
pub struct Transaction {
    state_update: StateUpdate,
    transaction_witness: Vec<u8>,
}

impl Transaction {
    pub fn new(state_update: StateUpdate, transaction_witness: &Bytes) -> Transaction {
        Transaction {
            state_update,
            transaction_witness: transaction_witness.to_vec(),
        }
    }
}

impl Encodable for Transaction {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(2);
        s.append(&self.state_update);
        s.append(&self.transaction_witness);
    }
}

impl Decodable for Transaction {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        let state_update: StateUpdate = rlp.val_at(0)?;
        let transaction_witness: Vec<u8> = rlp.val_at(1)?;
        Ok(Transaction {
            state_update,
            transaction_witness,
        })
    }
}
