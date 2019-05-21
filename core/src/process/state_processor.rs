use super::range_processor::RangeProcessor;
use crate::data_structure::transaction::Transaction;
use plasma_utils_range::Range;

pub struct StateProcessor {}

impl StateProcessor {
    pub fn apply(input_ranges: &[Range], transaction: &Transaction) -> Vec<Range> {
        //let state_update = transaction.get_state_update();
        // check transaction witness
        // transaction.get_transaction_witness()
        // call verify_deprecation
        RangeProcessor::put(
            input_ranges,
            transaction.get_start(),
            transaction.get_end(),
            &transaction.to_abi(),
        )
    }
}
