extern crate ethereum_types;
extern crate rlp;

use super::state_object::StateObject;
use ethereum_types::Address;
use rlp::{Decodable, DecoderError, Encodable, Rlp, RlpStream};

pub struct StateUpdate {
    start: u64,
    end: u64,
    block: u64,
    plasma_contract: Address,
    new_state: StateObject,
}

impl StateUpdate {
    pub fn new(
        start: u64,
        end: u64,
        block: u64,
        plasma_contract: Address,
        new_state: StateObject,
    ) -> StateUpdate {
        StateUpdate {
            start,
            end,
            block,
            plasma_contract,
            new_state,
        }
    }
}

impl Encodable for StateUpdate {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(5);
        s.append(&self.start);
        s.append(&self.end);
        s.append(&self.block);
        s.append(&self.plasma_contract);
        s.append(&self.new_state);
    }
}

impl Decodable for StateUpdate {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        let start: u64 = rlp.val_at(0)?;
        let end: u64 = rlp.val_at(1)?;
        let block: u64 = rlp.val_at(2)?;
        let plasma_contract: Address = rlp.val_at(3)?;
        let new_state: StateObject = rlp.val_at(4)?;
        Ok(StateUpdate {
            start,
            end,
            block,
            plasma_contract,
            new_state,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::DecoderError;
    use super::StateObject;
    use super::StateUpdate;
    use bytes::Bytes;
    use ethereum_types::Address;

    #[test]
    fn test_rlp_encode() {
        let message_bytes = Bytes::from(&b"parameters"[..]);
        let state_object = StateObject::new(Address::zero(), message_bytes);
        let state_update = StateUpdate::new(0, 0, 0, Address::zero(), state_object);
        let encoded = rlp::encode(&state_update);
        let _decoded: StateUpdate = rlp::decode(&encoded).unwrap();
        assert_eq!(_decoded.start, state_update.start);
    }

    #[test]
    fn fail_to_decode() {
        let failtodecode = "failtodecode";
        let encoded = rlp::encode(&failtodecode);
        let decoded: Result<StateUpdate, DecoderError> = rlp::decode(&encoded);
        assert_eq!(decoded.is_err(), true);
    }
}
