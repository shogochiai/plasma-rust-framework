extern crate ethabi;
extern crate rlp;

use super::state_object::StateObject;
use ethabi::Token;
use ethereum_types::Address;

#[derive(Clone, Debug, PartialEq)]
pub struct StateUpdate {
    state_object: StateObject,
    start: u64,
    end: u64,
    block_number: u64,
    plasma_contract: Address,
}

impl StateUpdate {
    pub fn new(
        state_object: &StateObject,
        start: u64,
        end: u64,
        block_number: u64,
        plasma_contract: Address,
    ) -> Self {
        StateUpdate {
            state_object: state_object.clone(),
            start,
            end,
            block_number,
            plasma_contract,
        }
    }
    pub fn to_abi(&self) -> Vec<u8> {
        ethabi::encode(&[
            Token::Bytes(self.state_object.to_abi()),
            Token::Uint(self.start.into()),
            Token::Uint(self.end.into()),
            Token::Uint(self.block_number.into()),
            Token::Address(self.plasma_contract),
        ])
    }
    pub fn from_abi(data: &[u8]) -> Result<Self, ethabi::Error> {
        let decoded: Vec<Token> = ethabi::decode(
            &[
                ethabi::ParamType::Bytes,
                ethabi::ParamType::Uint(8),
                ethabi::ParamType::Uint(8),
                ethabi::ParamType::Uint(8),
                ethabi::ParamType::Address,
            ],
            data,
        )?;
        let state_object = decoded[0].clone().to_bytes().unwrap();
        let start = decoded[1].clone().to_uint().unwrap();
        let end = decoded[2].clone().to_uint().unwrap();
        let block_number = decoded[3].clone().to_uint().unwrap();
        let plasma_contract = decoded[4].clone().to_address().unwrap();
        Ok(StateUpdate::new(
            &StateObject::from_abi(&state_object).unwrap(),
            start.as_u64(),
            end.as_u64(),
            block_number.as_u64(),
            plasma_contract,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::StateObject;
    use super::StateUpdate;
    use ethereum_types::Address;

    #[test]
    fn test_abi_encode() {
        let parameters_bytes = Vec::from(&b"parameters"[..]);
        let state_object = StateObject::new(Address::zero(), &parameters_bytes);

        let state_update = StateUpdate::new(&state_object, 0, 100, 1, Address::zero());
        let encoded = state_update.to_abi();
        let decoded: StateUpdate = StateUpdate::from_abi(&encoded).unwrap();
        assert_eq!(decoded.start, state_update.start);
    }

}
