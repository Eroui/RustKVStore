//! Module 1: Rust Basics - Variables, Mutability, and Functions
//!
//! Complete these 3 micro-exercises to get comfortable with basic Rust syntax!

/// Exercise 1: Return the sum of two integers.
///
/// Key concept: Function syntax `fn name(param: type) -> ReturnType`
/// Hint: In Rust, the last expression in a function without a semicolon `;` is returned automatically!
pub fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

/// Exercise 2: Mutate a variable and modify strings.
///
/// Key concept: Variables are immutable by default (`let x = 5;`).
/// To change a variable's value, use `let mut x = ...;`.
pub fn make_greeting(name: &str) -> String {
    // Hint:
    // 1. Declare a mutable string: `let mut greeting = String::from("Hello, ");`
    // 2. Push the name slice onto it: `greeting.push_str(name);`
    // 3. Return `greeting` (without a trailing semicolon).
    let mut greeting = String::from("Hello, ");
    greeting.push_str(name);
    greeting
}

/// Exercise 3: Create an owned empty String.
///
/// Key concept: `String::new()` creates a brand-new, empty String on the heap.
pub fn create_empty_string() -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_numbers() {
        assert_eq!(add_numbers(3, 4), 7);
        assert_eq!(add_numbers(-1, 1), 0);
    }

    #[test]
    fn test_make_greeting() {
        assert_eq!(make_greeting("Alice"), "Hello, Alice");
        assert_eq!(make_greeting("Bob"), "Hello, Bob");
    }

    #[test]
    fn test_create_empty_string() {
        let s = create_empty_string();
        assert!(s.is_empty());
        assert_eq!(s.len(), 0);
    }
}
