mod adder;
mod divider;
mod multiplier;
mod subtracter;

pub fn is_odd(n: usize) -> bool {
    n % 2 == 1
}

pub fn is_even(n: usize) -> bool {
    n % 2 == 0
}

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
        assert_eq!(adder::add_2992_to(2992), 5984);
    }

    #[test]
    fn test_divider() {
        assert_eq!(divider::floor_divide_by_2992(5984), 2);
    }

    #[test]
    fn test_subtracter() {
        assert_eq!(subtracter::subtract_2992_from(5984), 2992);
    }

    #[test]
    fn test_multiplier() {
        assert_eq!(multiplier::multiply_2992_and(2), 5984);
    }
}
