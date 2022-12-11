// iterators4.rs
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a hint.

use std::ops::Mul;

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Execute `rustlings hint iterators4` for hints.

    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables

    // if num > 1 { num * factorial(num - 1) } else { 1 }

    // For an extra challenge, don't use:
    // - recursion

    (1..=num).reduce(u64::mul).unwrap_or(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
