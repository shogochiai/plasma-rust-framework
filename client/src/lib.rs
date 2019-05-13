extern crate plasma_core;

pub mod error;
pub mod storage;
pub use self::storage::Storage;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
