mod error;

pub use crate::error::{AppError, Result};

#[must_use]
pub const fn add(left: u64, right: u64) -> u64 {
    left.saturating_add(right)
}

#[cfg(test)]
mod tests {
    use super::add;

    #[test]
    fn adds() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn saturates() {
        assert_eq!(add(u64::MAX, 1), u64::MAX);
    }
}
