//! Foundations Part 1: Control Flow, Tuples, & Variable Shadowing
//! (Covering Chapter 3 of The Rust Book)

/// 1. Variable Shadowing
///
/// In Rust, you can re-declare a variable using `let` with the same name.
/// This is called "shadowing". It lets you transform a value (and even change its type)
/// without needing `mut`.
///
/// Example:
/// ```rust
/// let spaces = "   ";          // &str
/// let spaces = spaces.len();   // usize (shadowed!)
/// ```
///
/// Task:
/// 1. Take the input string `spaces`.
/// 2. Shadow `spaces` to store its character/byte length using `spaces.len()`.
/// 3. Return `spaces as i32`.
///
pub fn measure_spaces(spaces: &str) -> i32 {
    let spaces = spaces.len();
    spaces as i32
}

/// 2. Tuples & Destructuring & 'if' Expressions
///
/// A tuple groups multiple values of different types: `let pair = (42, "hello");`
/// In Rust, `if` is an expression that returns a value!
///
/// Example of `if` expression:
///
/// Task:
/// Calculate `a + b`.
/// Return a tuple `(i32, String)` where:
/// - The first element is the calculated sum (`i32`).
/// - The second element is a `String` containing `"Positive"` if sum >= 0, or `"Negative"` if sum < 0.
pub fn analyze_sum(a: i32, b: i32) -> (i32, String) {
    let status = if a + b >= 0 { "Positive" } else { "Negative" };
    (a + b, status.to_string())
}

/// 3. Control Flow & Loops
///
/// Task: Calculate the factorial of `n` (n!) using a `for` loop.
/// Note: factorial(0) = 1, factorial(5) = 5 * 4 * 3 * 2 * 1 = 120.
///
/// Hint:
/// 1. Declare a mutable variable `let mut result: u64 = 1;`
/// 2. Loop over range `1..=n`: `for i in 1..=n { result *= i; }`
/// 3. Return `result`.
pub fn factorial(n: u64) -> u64 {
    let mut result: u64 = 1;
    for i in 1..=n {
        result *= i
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_measure_spaces() {
        assert_eq!(measure_spaces("   "), 3);
        assert_eq!(measure_spaces("hello world"), 11);
    }

    #[test]
    fn test_analyze_sum() {
        assert_eq!(analyze_sum(10, 5), (15, "Positive".to_string()));
        assert_eq!(analyze_sum(-10, 2), (-8, "Negative".to_string()));
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
    }
}
