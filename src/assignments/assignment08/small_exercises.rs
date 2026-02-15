//! Assignment 08: First-class functions.

use std::{collections::HashSet, thread::current};

/// Repeat
///
/// Returns an anonymous function that applies the given function `f` for `n` times.
/// For instance, `repeat(3, f)(x)` roughly translates to `f(f(f(x)))`.
///
/// Refer `test_repeat` in `assignment08_grade.rs` for detailed examples.
pub fn repeat<T, F: FnMut(T) -> T>(n: usize, mut f: F) -> impl FnMut(T) -> T {
    move |mut x: T| {
        for _ in 0..n {
            x = f(x)
        }
        x
    }
}

/// Funny Map
///
/// Applies the given function `f` for `i` times for the `i`-th element of the given vector.
///
/// For instance, `funny_map(f, [v0, v1, v2, v3])` roughly translates to `[v0, f(v1), f(f(v2)),
/// f(f(f(v3)))]`.
///
/// Refer `test_funny_map` in `assignment08_grade.rs` for detailed examples.
pub fn funny_map<T, F: Fn(T) -> T>(f: F, vs: Vec<T>) -> Vec<T> {
    vs.into_iter()
        .enumerate()
        .map(|(i, v)| {
            // Create a fresh adaptor that forwards to `f`
            let mut apply_i = repeat(i, &f);
            apply_i(v)
        })
        .collect()
}

/// Count Repeat
///
/// Returns the number of the elements in the set {`x`, `f(x)`, `f(f(x))`, `f(f(f(x)))`, ...}. You
/// may assume that the answer is finite and small enough.
///
/// Refer `test_count_repeat` in `assignment08_grade.rs` for detailed examples.
pub fn count_repeat<T, F: Fn(T) -> T>(f: F, x: T) -> usize
where
    T: PartialEq + Copy,
{
    let mut seen = Vec::new();
    let mut current = x; // make it mutable

    loop {
        if seen.contains(&current) {
            return seen.len();
        }

        seen.push(current);

        // Advance to the next value in the sequence:
        current = f(current);
    }
}

/// Either `T1`, or `T2`.
///
/// Fill out `map` method for this type.
#[derive(Debug, PartialEq, Eq)]
pub enum Either2<T1, T2> {
    /// Case 1.
    Case1 {
        /// The inner value.
        inner: T1,
    },
    /// Case 2.
    Case2 {
        /// The inner value.
        inner: T2,
    },
}

impl<T1, T2> Either2<T1, T2> {
    /// Maps the inner value.
    ///
    /// If the inner value is case 1, apply `f1`, and if it is case 2, apply `f2`.
    ///
    /// Refer `test_either2_map` in `assignment08_grade.rs` for detailed examples.
    pub fn map<U1, U2, F1, F2>(self, f1: F1, f2: F2) -> Either2<U1, U2>
    where
        F1: FnOnce(T1) -> U1,
        F2: FnOnce(T2) -> U2,
    {
        match self {
            Either2::Case1 { inner } => Either2::Case1 { inner: f1(inner) },
            Either2::Case2 { inner } => Either2::Case2 { inner: f2(inner) },
        }
    }
}
