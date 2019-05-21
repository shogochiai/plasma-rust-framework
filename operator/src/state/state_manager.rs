use plasma_core::process::state_processor::StateProcessor;

pub struct StateManager {}

impl StateManager {
    pub fn apply(transaction: &Transaction) {
        let input_ranges: &[Range] = &[];
        // get ranges from database
        let affected = StateProcessor::apply(input_ranges, transaction)
        // store affected to database
    }
}
