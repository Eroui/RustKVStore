//! Foundations Part 2: Ownership, Borrowing, and References
//! (Covering Chapter 4 of The Rust Book)

/// 1. Ownership & Move Semantics
///
/// In Rust, heap-allocated values (like `String`) have a single owner.
/// Passing an owned `String` into a function transfers (moves) ownership.
///
/// Task:
/// Take an owned `String` parameter `s`, append `"!"` to it, and return the modified `String`.
/// Key concept: Declare parameter `mut s: String` so you can modify it after taking ownership!
pub fn add_exclamation(mut s: String) -> String {
    s.push('!');
    s
}

/// 2. Immutable Borrowing (`&T`)
///
/// Borrowing allows a function to inspect a value WITHOUT taking ownership.
/// `&str` or `&String` provides read-only access.
///
/// Task:
/// Take a borrowed string slice `s: &str` and return `true` if it starts with the letter 'A' or 'a'.
/// Hint: `s.starts_with('A') || s.starts_with('a')`
pub fn starts_with_a(s: &str) -> bool {
    s.starts_with("A") || s.starts_with("a")
}

/// 3. Mutable Borrowing (`&mut T`)
///
/// To modify data in-place without taking ownership, pass a mutable reference (`&mut String`).
///
/// Task:
/// Modify the borrowed string `s` in-place by appending `", world!"` to it.
/// Hint: Use `s.push_str(", world!");`
pub fn append_world(s: &mut String) {
    s.push_str(", world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_exclamation() {
        let name = String::from("Rust");
        let result = add_exclamation(name);
        assert_eq!(result, "Rust!");
    }

    #[test]
    fn test_starts_with_a() {
        let greeting = String::from("Apple");
        assert!(starts_with_a(&greeting));
        assert!(!starts_with_a("banana"));
    }

    #[test]
    fn test_append_world() {
        let mut msg = String::from("Hello");
        append_world(&mut msg);
        assert_eq!(msg, "Hello, world!");
    }
}
