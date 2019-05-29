use super::error::Error;
use super::rangedb::RangeDatabase;
use plasma_utils_range::Range;

#[cfg(not(target_os = "android"))]
use leveldb::database::Database;
#[cfg(not(target_os = "android"))]
use leveldb::options::Options;
#[cfg(not(target_os = "android"))]
use tempdir::TempDir;

#[cfg(target_os = "android")]
pub struct MockRangeDatabase {
    ranges: Vec<Range>,
}

#[cfg(target_os = "android")]
impl Default for MockRangeDatabase {
    fn default() -> Self {
        Self { ranges: vec![] }
    }
}

#[cfg(target_os = "android")]
impl RangeDatabase for MockRangeDatabase {
    fn open(&mut self) {}
    fn get(&self, start: u64, end: u64) -> Result<Option<Vec<Range>>, Error> {
        let mut result = vec![];
        for range in &self.ranges {
            if start < range.get_end() {
                result.push(range.clone());
                if !range.intersect(start, end) {
                    break;
                }
            }
        }
        Ok(Some(result))
    }
    fn del(&self, _start: u64, _end: u64) -> Result<(), Error> {
        Ok(())
    }
    fn batch_put(&mut self, _start: u64, _end: u64, ranges: &[Range]) -> Result<(), Error> {
        for range in ranges {
            self.ranges.push(range.clone());
        }
        self.ranges.sort();
        Ok(())
    }
    fn put(&mut self, _start: u64, _end: u64, range: &Range) -> Result<(), Error> {
        self.ranges.push(range.clone());
        Ok(())
    }
}

#[cfg(not(target_os = "android"))]
pub struct MockRangeDatabase {
    ranges: Vec<Range>,
    db: Option<Database<i32>>,
}

#[cfg(not(target_os = "android"))]
impl Default for MockRangeDatabase {
    fn default() -> Self {
        Self {
            ranges: vec![],
            db: None,
        }
    }
}

#[cfg(not(target_os = "android"))]
impl RangeDatabase for MockRangeDatabase {
    fn open(&mut self) {
        let tempdir = TempDir::new("demo").unwrap();
        let path = tempdir.path();

        let mut options = Options::new();
        options.create_if_missing = true;
        self.db = Database::open(path, options).ok();
    }
    fn get(&self, start: u64, end: u64) -> Result<Option<Vec<Range>>, Error> {
        let mut result = vec![];
        for range in &self.ranges {
            if start < range.get_end() {
                result.push(range.clone());
                if !range.intersect(start, end) {
                    break;
                }
            }
        }
        Ok(Some(result))
    }
    fn del(&self, _start: u64, _end: u64) -> Result<(), Error> {
        Ok(())
    }
    fn batch_put(&mut self, _start: u64, _end: u64, ranges: &[Range]) -> Result<(), Error> {
        for range in ranges {
            self.ranges.push(range.clone());
        }
        self.ranges.sort();
        Ok(())
    }
    fn put(&mut self, _start: u64, _end: u64, range: &Range) -> Result<(), Error> {
        self.ranges.push(range.clone());
        Ok(())
    }
}
