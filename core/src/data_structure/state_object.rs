extern crate ethereum_types;
extern crate rlp;

use bytes::Bytes;
use ethereum_types::Address;
use rlp::{Decodable, DecoderError, Encodable, Rlp, RlpStream};

pub struct StateObject {
    predicate: Address,
    parameters: Bytes,
}

impl StateObject {
    pub fn new(predicate: Address, parameters: Bytes) -> StateObject {
        return StateObject {
            predicate: predicate,
            parameters: parameters,
        };
    }
}

impl Encodable for StateObject {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(2);
        s.append(&self.predicate);
        s.append(&self.parameters.as_ref());
    }
}

impl Decodable for StateObject {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        let bytes_result: Result<Vec<u8>, DecoderError> = rlp.val_at(1);
        bytes_result.map(|bytes| {
            return StateObject {
                predicate: rlp.val_at(0).unwrap_or(Address::zero()),
                parameters: Bytes::from(bytes),
            };
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Bytes;
    use super::DecoderError;
    use super::StateObject;
    use ethereum_types::Address;

    #[test]
    fn test_rlp_encode() {
        let message = "Hello World".as_bytes();
        let message_bytes = Bytes::from(message);
        let _state_object = StateObject::new(Address::zero(), message_bytes);
        let encoded = rlp::encode(&_state_object);
        let _decoded: StateObject = rlp::decode(&encoded).unwrap();
        assert_eq!(_decoded.predicate, _state_object.predicate);
    }

    #[test]
    fn fail_to_decode() {
        let animal = "cat";
        let encoded = rlp::encode(&animal);
        let decoded: Result<StateObject, DecoderError> = rlp::decode(&encoded);
        assert_eq!(decoded.is_err(), true);
    }

}
