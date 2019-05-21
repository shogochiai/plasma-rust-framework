use super::error::Error;
use plasma_utils_range::Range;

pub trait RangeDatabase {
    fn get(&self, start: u64, end: u64) -> Result<Option<Vec<Range>>, Error>;
    fn del(&self, start: u64, end: u64) -> Result<(), Error>;
    fn batch_put(&mut self, start: u64, end: u64, ranges: &[Range]) -> Result<(), Error>;
    fn put(&mut self, start: u64, end: u64, range: &Range) -> Result<(), Error>;
}
