//! A library for categorizing and operating on integers

pub mod adder;
pub mod categories;
pub mod divider;
pub mod multiplier;
pub mod subtracter;

pub use categories::{is_even, is_odd};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_odd() {
        assert!(is_odd(99));
        assert!(!is_odd(98));
    }

    #[test]
    fn test_is_even() {
        assert!(is_even(98));
        assert!(!is_even(99));
    }

    #[test]
    fn test_adder() {
        assert_eq!(adder::add_22_to(22), 44);
    }

    #[test]
    fn test_divider() {
        assert_eq!(divider::floor_divide_by_22(44), 2);
    }

    #[test]
    fn test_subtracter() {
        assert_eq!(subtracter::subtract_22_from(44), 22);
    }

    #[test]
    fn test_multiplier() {
        assert_eq!(multiplier::multiply_22_and(2), 44);
    }
}
