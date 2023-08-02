mod mq_version;
mod error;

use std::sync::atomic::{AtomicU64, Ordering};
pub use mq_version::*;
pub use error::MQError;

const COMMAND_ID: AtomicU64 = AtomicU64::new(0);

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn get_command_id() -> u64 {
    COMMAND_ID.fetch_add(1, Ordering::AcqRel)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
