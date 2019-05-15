extern crate ethereum_types;
extern crate rlp;

use super::transaction::Transaction;
use bytes::Bytes;
use ethereum_types::H256;
use rlp::{Decodable, DecoderError, Encodable, Rlp, RlpStream};

#[derive(Clone, Debug)]
pub struct IncludedTransaction {
    transaction: Transaction,
    root: H256,
    proof: Vec<u8>,
    index: u8,
}

impl IncludedTransaction {
    pub fn new(transaction: Transaction, root: H256, proof: &Bytes, index: u8) -> Self {
        IncludedTransaction {
            transaction,
            root,
            proof: proof.to_vec(),
            index,
        }
    }
}

impl Encodable for IncludedTransaction {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(4);
        s.append(&self.transaction);
        s.append(&self.root);
        s.append(&self.proof);
        s.append(&self.index);
    }
}

impl Decodable for IncludedTransaction {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        let transaction: Transaction = rlp.val_at(0)?;
        let root: H256 = rlp.val_at(1)?;
        let proof: Vec<u8> = rlp.val_at(2)?;
        let index: u8 = rlp.val_at(3)?;
        Ok(IncludedTransaction {
            transaction,
            root,
            proof,
            index,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::IncludedTransaction;
    use crate::data_structure::state_object::StateObject;
    use crate::data_structure::state_update::StateUpdate;
    use crate::data_structure::transaction::Transaction;
    use bytes::Bytes;
    use ethereum_types::{Address, H256};

    #[test]
    fn test_rlp_encode() {
        let message_bytes = Bytes::from(&b"parameters"[..]);
        let state_object = StateObject::new(Address::zero(), &message_bytes);
        let state_update = StateUpdate::new(0, 0, 0, Address::zero(), state_object);

        let proof_bytes = Bytes::from(&b"proof"[..]);
        let transaction = Transaction::new(state_update, &message_bytes);
        let included_transaction =
            IncludedTransaction::new(transaction, H256::zero(), &proof_bytes, 0);
        let encoded = rlp::encode(&included_transaction);
        let _decoded: IncludedTransaction = rlp::decode(&encoded).unwrap();
        assert_eq!(_decoded.root, included_transaction.root);
    }

}
