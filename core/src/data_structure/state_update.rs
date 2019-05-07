extern crate ethereum_types;

use super::state_object::StateObject;
use ethereum_types::Address;

pub struct StateUpdate {
    start: u64,
    end: u64,
    block: u64,
    plasma_contract: Address,
    new_state: StateObject,
}
