use crate::predicate::PredicatePlugin;
use plasma_core::data_structure::{StateUpdate, Transaction};

/// Simple ownership predicate
pub struct OwnershipPredicate {}

impl Default for OwnershipPredicate {
    fn default() -> Self {
        OwnershipPredicate {}
    }
}

impl PredicatePlugin for OwnershipPredicate {
    fn execute_state_transition(
        &self,
        input: &StateUpdate,
        _transaction: &Transaction,
    ) -> StateUpdate {
        // should parse transaction.parameters
        // make new state update
        input.clone()
    }
}
