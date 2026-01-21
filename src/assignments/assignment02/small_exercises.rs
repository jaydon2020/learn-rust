//! Small problems.

use std::iter;

use itertools::Itertools;
use rayon::range;

use crate::assignments::assignment07::small_exercises::range;

const FAHRENHEIT_OFFSET: f64 = 32.0;
const FAHRENHEIT_SCALE: f64 = 5.0 / 9.0;

/// Converts Fahrenheit to Celsius temperature degree.
pub fn fahrenheit_to_celsius(degree: f64) -> f64 {
    FAHRENHEIT_SCALE * (degree - FAHRENHEIT_OFFSET)
}

/// Capitalizes English alphabets (leaving the other characters intact).
pub fn capitalize(input: String) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                c.to_ascii_uppercase()
            } else {
                c
            }
        })
        .collect()
}

/// Returns the sum of the given array. (We assume the absence of integer overflow.)
pub fn sum_array(input: &[u64]) -> u64 {
    let mut sum: u64 = 0;
    input.iter().for_each(|x| sum += x);
    sum
}

/// Given a non-negative integer, say `n`, return the smallest integer of the form `3^m` that's
/// greater than or equal to `n`.
///
/// For instance, up3(6) = 9, up3(9) = 9, up3(10) = 27. (We assume the absence of integer overflow.)
pub fn up3(n: u64) -> u64 {
    if n <= 1 {
        return 1;
    }
    let mut p = 1u64;
    while p < n {
        match p.checked_mul(3) {
            Some(next) => p = next,
            None => return u64::MAX, // or handle as you prefer
        }
    }
    p
}

/// Returns the greatest common divisor (GCD) of two non-negative integers. (We assume the absence
/// of integer overflow.)
pub fn gcd(lhs: u64, rhs: u64) -> u64 {
    if lhs % rhs == 0 {
        rhs
    } else {
        let temp = lhs % rhs;
        gcd(rhs, temp)
    }
}

/// Returns the array of nC0, nC1, nC2, ..., nCn, where nCk = n! / (k! * (n-k)!). (We assume the
/// absence of integer overflow.)
///
/// Consult <https://en.wikipedia.org/wiki/Pascal%27s_triangle> for computation of binomial
/// coefficients without integer overflow.
pub fn chooses(n: u64) -> Vec<u64> {
    let mut row = Vec::with_capacity((n as usize) + 1);
    let mut c = 1u128;
    row.push(c as u64);
    for k in 0..n {
        c = c * u128::from(n - k) / u128::from(k + 1);
        // If you want to detect overflow instead of truncating:
        // let val = u64::try_from(c).expect("binomial coefficient overflowed u64");
        row.push(c as u64);
    }
    row
}

/// Returns the "zip" of two vectors.
///
/// For instance, `zip(vec![1, 2, 3], vec![4, 5])` equals to `vec![(1, 4), (2, 5)]`. Here, `3` is
/// ignored because it doesn't have a partner.
pub fn zip(lhs: Vec<u64>, rhs: Vec<u64>) -> Vec<(u64, u64)> {
    lhs.into_iter().zip(rhs).collect()
}
