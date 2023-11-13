//! A module with functions for categorizing numbers

/// Checks if `n` is odd
pub fn is_odd(n: usize) -> bool {
    n % 2 == 1
}

/// Checks if `n` is even
pub fn is_even(n: usize) -> bool {
    n % 2 == 0
}

/// Checks if `n` is a natural number
pub fn is_natural(n: usize) -> bool {
    n > 0
}

/// Checks if `n` is a whole number
pub fn is_whole(n: usize) -> bool {
    true
}

/// Checks if `n` is an integer
pub fn is_integer(n: isize) -> bool {
    true
}

/// Checks if `n` is a non-positive integer
pub fn is_non_positive(n: isize) -> bool {
    n <= 0
}

/// Checks if `n` is a non-positive integer (whole number)
pub fn is_non_negative(n: usize) -> bool {
    true
}
