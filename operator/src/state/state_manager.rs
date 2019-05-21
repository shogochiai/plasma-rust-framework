use plasma_core::data_structure::transaction::Transaction;
use plasma_core::process::state_processor::StateProcessor;
use plasma_db::mock::MockRangeDatabase;
use plasma_db::rangedb::RangeDatabase;

pub struct StateManager {
    range_db: MockRangeDatabase,
}

impl StateManager {
    pub fn apply(&mut self, transaction: &Transaction) -> bool {
        // get ranges from database
        //let state_update = transaction.get_state_update();
        let input_ranges = self
            .range_db
            .get(transaction.get_start(), transaction.get_end())
            .unwrap()
            .unwrap();
        // call pure function
        let affected = StateProcessor::apply(&input_ranges, transaction);
        // store ranges to database
        self.range_db
            .batch_put(transaction.get_start(), transaction.get_end(), &affected)
            .is_ok()
    }
}
