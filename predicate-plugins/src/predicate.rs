use plasma_core::data_structure::{StateUpdate, Transaction};

/// Base class of predicate plugin
pub trait PredicatePlugin {
    fn execute_state_transition(
        &self,
        input: &StateUpdate,
        transaction: &Transaction,
    ) -> StateUpdate;
}
