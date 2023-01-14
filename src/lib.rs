//! # My Crate
//!
//! `woofytest` is a personal demo project to
//! test carge crate registry functionality.

/// Adds three to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = woofytest::add_three(arg);
///
/// assert_eq!(8, answer);
/// ```
pub fn add_three(x: i32) -> i32 {
    x + 3
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_three(100);
        assert_eq!(result, 103);
    }
}
