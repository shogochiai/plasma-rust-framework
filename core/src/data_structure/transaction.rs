extern crate ethereum_types;
extern crate rlp;
extern crate tiny_keccak;

use ethabi::Token;
use ethereum_types::Address;
use rlp::{Decodable, DecoderError, Encodable, Rlp, RlpStream};
use tiny_keccak::Keccak;

#[derive(Debug)]
pub enum Error {
    VerifyError,
}

#[derive(Clone, Debug)]
pub struct Witness {
    v: Vec<u8>,
    r: Vec<u8>,
    s: u64,
}

impl Witness {
    pub fn new(v: &[u8], r: &[u8], s: u64) -> Self {
        Witness {
            v: v.to_vec(),
            r: r.to_vec(),
            s,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Transaction {
    plasma_contract_address: Address,
    start: u64,
    end: u64,
    method_id: Vec<u8>,
    parameters: Vec<u8>,
    witness: Witness,
}

impl Transaction {
    pub fn new(
        plasma_contract_address: Address,
        start: u64,
        end: u64,
        method_id: &[u8],
        parameters: &[u8],
        witness: &Witness,
    ) -> Transaction {
        Transaction {
            plasma_contract_address,
            start,
            end,
            method_id: method_id.to_vec(),
            parameters: parameters.to_vec(),
            witness: witness.clone(),
        }
    }
    pub fn to_body_abi(&self) -> Vec<u8> {
        ethabi::encode(&[
            Token::Address(self.plasma_contract_address),
            Token::Uint(self.start.into()),
            Token::Uint(self.end.into()),
            Token::FixedBytes(self.method_id.clone()),
            Token::Bytes(self.parameters.clone()),
        ])
    }
    pub fn to_abi(&self) -> Vec<u8> {
        ethabi::encode(&[
            Token::Address(self.plasma_contract_address),
            Token::Uint(self.start.into()),
            Token::Uint(self.end.into()),
            Token::FixedBytes(self.method_id.clone()),
            Token::Bytes(self.parameters.clone()),
            Token::FixedBytes(self.witness.v.clone()),
            Token::FixedBytes(self.witness.r.clone()),
            Token::Uint(self.witness.s.into()),
        ])
    }
    pub fn from_abi(data: &[u8]) -> Result<Self, ethabi::Error> {
        let decoded = ethabi::decode(
            &[
                ethabi::ParamType::Address,
                ethabi::ParamType::Uint(16),
                ethabi::ParamType::Uint(16),
                ethabi::ParamType::FixedBytes(1),
                ethabi::ParamType::Bytes,
                ethabi::ParamType::FixedBytes(32),
                ethabi::ParamType::FixedBytes(32),
                ethabi::ParamType::Uint(1),
            ],
            data,
        )?;
        let v = &decoded[5].clone().to_fixed_bytes().unwrap();
        let r = &decoded[6].clone().to_fixed_bytes().unwrap();
        let s = decoded[7].clone().to_uint().unwrap().as_u64();
        Ok(Transaction::new(
            decoded[0].clone().to_address().unwrap(),
            decoded[1].clone().to_uint().unwrap().as_u64(),
            decoded[2].clone().to_uint().unwrap().as_u64(),
            &decoded[3].clone().to_fixed_bytes().unwrap(),
            &decoded[4].clone().to_bytes().unwrap(),
            &Witness::new(&v, &r, s),
        ))
    }
    pub fn create_method_id(value: &[u8]) -> Vec<u8> {
        let mut hasher = Keccak::new_sha3_256();
        hasher.update(value);
        let mut result: [u8; 32] = [0; 32];
        hasher.finalize(&mut result);
        result.to_vec()
    }
}

impl Encodable for Transaction {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(1);
        s.append(&self.to_abi());
    }
}

impl Decodable for Transaction {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        let abi: Vec<u8> = rlp.list_at(0)?;
        Ok(Transaction::from_abi(&abi).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::Transaction;
    use super::Witness;
    use ethereum_types::Address;

    #[test]
    fn test_abi_encode() {
        let parameters_bytes = Vec::from(&b"parameters"[..]);
        let transaction = Transaction::new(
            Address::zero(),
            0,
            100,
            &Transaction::create_method_id(&b"send(address)"[..]),
            &parameters_bytes,
            &Witness::new(&parameters_bytes, &parameters_bytes, 0),
        );
        let encoded = transaction.to_abi();
        let decoded: Transaction = Transaction::from_abi(&encoded).unwrap();
        assert_eq!(decoded.start, transaction.start);
    }

}
