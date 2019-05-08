extern crate ethereum_types;
extern crate rlp;

use super::signed_transaction::SignedTransaction;
use bytes::Bytes;
use ethereum_types::H256;
use rlp::{Decodable, DecoderError, Encodable, Rlp, RlpStream};

pub struct IncludedTransaction {
    signed_transaction: SignedTransaction,
    root: H256,
    proof: Bytes,
    index: u8,
}

impl IncludedTransaction {
    pub fn new(signed_transaction: SignedTransaction, root: H256, proof: Bytes, index: u8) -> Self {
        IncludedTransaction {
            signed_transaction,
            root,
            proof,
            index,
        }
    }
}

impl Encodable for IncludedTransaction {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(4);
        s.append(&self.signed_transaction);
        s.append(&self.root);
        s.append(&self.proof.as_ref());
        s.append(&self.index);
    }
}

impl Decodable for IncludedTransaction {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        let signed_transaction_result: Result<SignedTransaction, DecoderError> = rlp.val_at(0);
        let proof_result: Result<Vec<u8>, DecoderError> = rlp.val_at(2);
        signed_transaction_result.and_then(|signed_transaction| {
            proof_result.map(|proof| IncludedTransaction {
                signed_transaction,
                root: rlp.val_at(1).unwrap_or_else(|_| H256::zero()),
                proof: Bytes::from(proof),
                index: rlp.val_at(3).unwrap_or(0),
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::IncludedTransaction;
    use super::H256;
    use crate::data_structure::signed_transaction::SignedTransaction;
    use bytes::Bytes;

    #[test]
    fn test_rlp_encode() {
        let proof_bytes = Bytes::from(&b"proof"[..]);
        let signed_transaction = SignedTransaction {
            transactions: vec![],
        };
        let included_transaction =
            IncludedTransaction::new(signed_transaction, H256::zero(), proof_bytes, 0);
        let encoded = rlp::encode(&included_transaction);
        let _decoded: IncludedTransaction = rlp::decode(&encoded).unwrap();
        assert_eq!(_decoded.root, included_transaction.root);
    }

}
