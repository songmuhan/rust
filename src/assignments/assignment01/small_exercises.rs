//! Assignment 1: Preparing Rust Development Environment.
//! Welcome to the CS220 course!
//!
//! You should fill out `add()` and `sub()` function bodies in such a way that `/scripts/grade.sh 1` works fine.
//! See `small_problems_grade.rs` and `/scripts/grade.sh 1` for the test script.
//!
//! Hint: <https://doc.rust-lang.org/std/primitive.usize.html>

/// Adds two unsigned words. If overflow happens, just wrap around.
pub fn add(lhs: usize, rhs: usize) -> usize {
    lhs.wrapping_add(rhs)
}

/// Subtracts two unsigned words. If overflow happens, just wrap around.
pub fn sub(lhs: usize, rhs: usize) -> usize {
    lhs.wrapping_sub(rhs)
}
