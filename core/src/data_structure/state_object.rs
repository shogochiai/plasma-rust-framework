extern crate ethereum_types;

use bytes::Bytes;
use ethereum_types::Address;

pub struct StateObject {
    predicate: Address,
    parameters: Bytes,
}

#[cfg(test)]
mod tests {
    use super::Bytes;
    use super::StateObject;
    use ethereum_types::Address;

    #[test]
    fn test_build() {
        let message = "Hello World".as_bytes();
        let message_bytes = Bytes::from(message);
        let _state_object = StateObject {
            predicate: Address::zero(),
            parameters: message_bytes,
        };
    }
}
