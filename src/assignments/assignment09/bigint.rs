//! Big integer with infinite precision.

use std::fmt;
use std::iter::zip;
use std::ops::*;

/// An signed integer with infinite precision implemented with an "carrier" vector of `u32`s.
///
/// The vector is interpreted as a base 2^(32 * (len(carrier) - 1)) integer, where negative
/// integers are represented in their [2's complement form](https://en.wikipedia.org/wiki/Two%27s_complement).
///
/// For example, the vector `vec![44,345,3]` represents the integer
/// `44 * (2^32)^2 + 345 * (2^32) + 3`,
/// and the vector `vec![u32::MAX - 5, u32::MAX - 7]` represents the integer
/// `- (5 * 2^32 + 8)`
///
/// You will implement the `Add` and `Sub` trait for this type.
///
/// Unlike standard fix-sized intergers in Rust where overflow will panic, the carrier is extended
/// to save the overflowed bit. On the contrary, if the precision is too much (e.g, vec![0,0] is
/// used to represent 0, where `vec![0]` is sufficent), the carrier is truncated.
///
/// See [this section](https://en.wikipedia.org/wiki/Two%27s_complement#Arithmetic_operations) for a rouge guide on implementation,
/// while keeping in mind that the carrier should be extended to deal with overflow.
///
/// The `sign_extension()`, `two_complement()`, and `truncate()` are non-mandatory helper methods.
///
/// For testing and debugging purposes, the `Display` trait is implemented for you, which shows the
/// integer in hexadecimal form.
#[derive(Debug, Clone)]
pub struct BigInt {
    /// The carrier for `BigInt`.
    ///
    /// Note that the carrier should always be non-empty.
    pub carrier: Vec<u32>,
}

impl BigInt {
    /// Create a new `BigInt` from a `usize`.
    pub fn new(n: u32) -> Self {
        BigInt { carrier: vec![n] }
    }

    /// Creates a new `BigInt` from a `Vec<u32>`.
    ///
    /// # Panic
    ///
    /// Panics if `carrier` is empty.
    pub fn new_large(carrier: Vec<u32>) -> Self {
        assert!(!carrier.is_empty(), "BigInt::carrier must be non-empty");
        BigInt { carrier }
    }
}

const SIGN_MASK: u32 = 1 << 31;

impl BigInt {
    /// Determine the sign extension word for this value (0 for non-negative, 0xFFFF_FFFF for negative).
    #[inline]
    fn sign_word(&self) -> u32 {
        match self.carrier.first() {
            Some(&w) if (w & SIGN_MASK) != 0 => u32::MAX,
            _ => 0,
        }
    }

    /// Extend `self` to `len` words (big-endian) by sign extension.
    ///
    /// Note: despite the docstring saying "bits", we extend to `len` *words* (32-bit limbs),
    /// which is what is needed for arithmetic on the carrier.
    fn sign_extension(&self, len: usize) -> Self {
        let cur_len = self.carrier.len();
        if cur_len >= len {
            return self.clone();
        }
        let sign = self.sign_word();
        let mut out = Vec::with_capacity(len);
        out.extend(std::iter::repeat(sign).take(len - cur_len));
        out.extend(self.carrier.iter().copied());
        BigInt { carrier: out }
    }

    /// Compute the two's complement of `self`.
    ///
    /// This treats the number as having just enough width to represent itself, then inverts all
    /// bits and adds one. The result is then canonicalized via `truncate()`.
    fn two_complement(&self) -> Self {
        // Invert all words, then add 1 from the least significant word (the end).
        let mut inv: Vec<u32> = self.carrier.iter().map(|w| !w).collect();
        // Add one (small unsigned addition from LSW).
        let mut carry: u64 = 1;
        for w in inv.iter_mut().rev() {
            let sum = (*w as u64) + carry;
            *w = sum as u32;
            carry = sum >> 32;
            if carry == 0 {
                break;
            }
        }
        // If carry is still 1, we need to grow by a new MSW of 0 (since ~x + 1 can overflow),
        // but in two's complement infinite precision, that extra word would be zero, which is
        // redundant and will be removed by truncate() anyway. We'll just truncate next.
        BigInt { carrier: inv }.truncate()
    }

    /// Truncate a `BigInt` to the minimum length by removing redundant sign-extension words.
    ///
    /// Rules:
    /// - Keep at least one word.
    /// - Drop leading 0 words while the next word's sign bit is 0.
    /// - Drop leading 0xFFFF_FFFF words while the next word's sign bit is 1.
    fn truncate(&self) -> Self {
        if self.carrier.is_empty() {
            return BigInt { carrier: vec![0] };
        }

        let mut drop_count = 0usize;
        // We'll walk pairs (msw, next) from the start and stop at the first non-droppable pair.
        while drop_count + 1 < self.carrier.len() {
            let msw = self.carrier[drop_count];
            let next = self.carrier[drop_count + 1];
            let next_sign = (next & SIGN_MASK) != 0;

            let can_drop = (msw == 0 && !next_sign) || (msw == u32::MAX && next_sign);
            if can_drop {
                drop_count += 1;
            } else {
                break;
            }
        }

        let mut out = if drop_count > 0 {
            self.carrier[drop_count..].to_vec()
        } else {
            self.carrier.clone()
        };

        if out.is_empty() {
            out.push(0);
        }

        BigInt { carrier: out }
    }

    /// Internal: big-endian addition with sign extension.
    /// Returns a canonicalized result.
    fn add_raw(a: &BigInt, b: &BigInt) -> BigInt {
        let max_len = a.carrier.len().max(b.carrier.len()) + 1; // +1 to capture carry/sign overflow
        let ae = a.sign_extension(max_len);
        let be = b.sign_extension(max_len);

        let mut res_rev: Vec<u32> = Vec::with_capacity(max_len);
        let mut carry: u64 = 0;

        // Add from least significant word to most (iterate in reverse)
        for (aw, bw) in zip(ae.carrier.iter().rev(), be.carrier.iter().rev()) {
            let sum = (*aw as u64) + (*bw as u64) + carry;
            res_rev.push(sum as u32);
            carry = sum >> 32;
        }

        // We extended by +1 word using sign extension. That guarantees any residual `carry`
        // gets properly represented in the most significant accumulated word. No need to push.
        res_rev.reverse();
        BigInt { carrier: res_rev }.truncate()
    }

    /// Internal: big-endian subtraction with sign extension (a - b).
    /// Returns a canonicalized result.
    fn sub_raw(a: &BigInt, b: &BigInt) -> BigInt {
        let max_len = a.carrier.len().max(b.carrier.len()) + 1;
        let ae = a.sign_extension(max_len);
        let be = b.sign_extension(max_len);

        let mut res_rev: Vec<u32> = Vec::with_capacity(max_len);
        let mut borrow: i64 = 0;

        for (aw, bw) in zip(ae.carrier.iter().rev(), be.carrier.iter().rev()) {
            let ai = *aw as i64;
            let bi = *bw as i64;
            let mut diff = ai - bi - borrow;
            if diff < 0 {
                diff += 1i64 << 32;
                borrow = 1;
            } else {
                borrow = 0;
            }
            res_rev.push(diff as u32);
        }

        res_rev.reverse();
        BigInt { carrier: res_rev }.truncate()
    }
}

impl Add for BigInt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        BigInt::add_raw(&self, &rhs)
    }
}

impl Sub for BigInt {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        BigInt::sub_raw(&self, &rhs)
    }
}

impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Hex formatting so that each u32 can be formatted independently.
        for i in self.carrier.iter() {
            write!(f, "{:08x}", i)?;
        }
        Ok(())
    }
}
