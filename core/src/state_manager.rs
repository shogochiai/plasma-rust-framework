use crate::data_structure::error::Error;
use crate::data_structure::{StateUpdate, Transaction};
use plasma_db::impls::rangestore::memory::RangeDbMemoryImpl;
use plasma_db::range::Range;
use plasma_db::traits::db::DatabaseTrait;
use plasma_db::traits::rangestore::RangeStore;

struct MockPredicatePlugin {}
impl MockPredicatePlugin {
    pub fn execute_state_transition(
        input: &StateUpdate,
        _transaction: &Transaction,
    ) -> StateUpdate {
        input.clone()
    }
}

pub struct ResultOfExecuteTransaction {
    state_update: Box<StateUpdate>,
    ranges: Box<[Range]>,
}

impl ResultOfExecuteTransaction {
    pub fn new(state_update: &StateUpdate, ranges: &[Range]) -> Self {
        ResultOfExecuteTransaction {
            state_update: Box::new(state_update.clone()),
            ranges: ranges.to_vec().into_boxed_slice(),
        }
    }
    pub fn get_state_update(&self) -> &StateUpdate {
        &self.state_update
    }
    pub fn get_ranges(&self) -> &[Range] {
        &self.ranges
    }
}

pub struct StateManager {
    db: Box<RangeStore>,
}

impl Default for StateManager {
    fn default() -> Self {
        Self {
            db: Box::new(RangeDbMemoryImpl::open("test")),
        }
    }
}

impl StateManager {
    fn ranges_to_state_update(ranges: &[Range]) -> Result<Vec<StateUpdate>, Error> {
        ranges
            .iter()
            .map(|range| StateUpdate::from_abi(range.get_value()))
            .collect()
    }

    /// force to put state update
    pub fn deposit(&self, start: u64, end: u64, value: &[u8]) -> Result<(), Error> {
        self.db
            .put(start, end, value)
            .map_err::<Error, _>(Into::into)
    }

    /// Execute a transaction
    pub fn execute_transaction(
        &self,
        transaction: &Transaction,
    ) -> Result<ResultOfExecuteTransaction, Error> {
        let _ranges = self
            .db
            .get(transaction.get_start(), transaction.get_end())
            .map_err::<Error, _>(Into::into)?;
        let state_updates = Self::ranges_to_state_update(&_ranges)?;
        let new_state_updates: Vec<StateUpdate> = state_updates
            .iter()
            .map(|state_update| {
                MockPredicatePlugin::execute_state_transition(state_update, transaction)
            })
            .collect();
        // new_state_updates should has same state_update
        let new_state_update: &StateUpdate = &new_state_updates[0];
        self.db
            .put(
                transaction.get_start(),
                transaction.get_end(),
                &new_state_update.to_abi(),
            )
            .map_err::<Error, _>(Into::into)?;
        Ok(ResultOfExecuteTransaction::new(new_state_update, &_ranges))
    }
}

#[cfg(test)]
mod tests {
    use super::StateManager;
    use crate::data_structure::{StateObject, StateUpdate, Transaction, Witness};
    use ethereum_types::{Address, H256};

    fn create_state_update(start: u64, end: u64, block_number: u64) -> StateUpdate {
        StateUpdate::new(
            &StateObject::new(Address::zero(), &b"data"[..]),
            start,
            end,
            block_number,
            Address::zero(),
        )
    }

    #[test]
    fn test_execute_transaction() {
        // make state update
        let state_update = create_state_update(0, 100, 1);
        // make transaction
        let transaction = Transaction::new(
            Address::zero(),
            0,
            100,
            Transaction::create_method_id(&b"send(address)"[..]),
            &b"new state update"[..],
            &Witness::new(H256::zero(), H256::zero(), 0),
        );

        let state_manager: StateManager = Default::default();
        let deposit_result = state_manager.deposit(0, 100, &state_update.to_abi());
        assert!(deposit_result.is_ok());
        let result = state_manager.execute_transaction(&transaction);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_transaction_for_partial_range() {
        // make state update
        let state_update = create_state_update(0, 100, 1);
        // make transaction
        let transaction = Transaction::new(
            Address::zero(),
            0,
            20,
            Transaction::create_method_id(&b"send(address)"[..]),
            &b"new state update"[..],
            &Witness::new(H256::zero(), H256::zero(), 0),
        );

        let state_manager: StateManager = Default::default();
        let deposit_result = state_manager.deposit(0, 100, &state_update.to_abi());
        assert!(deposit_result.is_ok());
        let result = state_manager.execute_transaction(&transaction);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_transaction_for_multiple_ranges() {
        // make state update
        let state_update1 = create_state_update(0, 100, 1);
        let state_update2 = create_state_update(100, 200, 2);
        // make transaction
        let transaction = Transaction::new(
            Address::zero(),
            50,
            150,
            Transaction::create_method_id(&b"send(address)"[..]),
            &b"new state update"[..],
            &Witness::new(H256::zero(), H256::zero(), 0),
        );

        let state_manager: StateManager = Default::default();
        assert!(state_manager
            .deposit(0, 100, &state_update1.to_abi())
            .is_ok());
        assert!(state_manager
            .deposit(100, 200, &state_update2.to_abi())
            .is_ok());
        let result = state_manager.execute_transaction(&transaction);
        assert!(result.is_ok());
    }

}
