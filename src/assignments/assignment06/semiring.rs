//! Semiring

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::Debug;

use itertools::Itertools;
use ndarray_rand::rand_distr::num_traits::zero;

/// Semiring.
///
/// Consult <https://en.wikipedia.org/wiki/Semiring>.
pub trait Semiring: Debug + Clone + PartialEq {
    /// Additive identity.
    fn zero() -> Self;
    /// Multiplicative identity.
    fn one() -> Self;
    /// Addition operation.
    fn add(&self, rhs: &Self) -> Self;
    /// Multiplication operation.
    fn mul(&self, rhs: &Self) -> Self;
}

/// Converts integer to semiring value.
pub fn from_usize<T: Semiring>(value: usize) -> T {
    let mut result = T::zero();
    let one = T::one();

    for _ in 0..value {
        result = T::add(&result, &one);
    }

    result
}

impl Semiring for u64 {
    fn zero() -> Self {
        0_u64
    }

    fn one() -> Self {
        1_u64
    }

    fn add(&self, rhs: &Self) -> Self {
        *self + *rhs
    }

    fn mul(&self, rhs: &Self) -> Self {
        *self * *rhs
    }
}

impl Semiring for i64 {
    fn zero() -> Self {
        0_i64
    }

    fn one() -> Self {
        1_i64
    }

    fn add(&self, rhs: &Self) -> Self {
        *self + *rhs
    }

    fn mul(&self, rhs: &Self) -> Self {
        *self * *rhs
    }
}

impl Semiring for f64 {
    fn zero() -> Self {
        0_f64
    }

    fn one() -> Self {
        1_f64
    }

    fn add(&self, rhs: &Self) -> Self {
        *self + *rhs
    }

    fn mul(&self, rhs: &Self) -> Self {
        *self * *rhs
    }
}

/// Polynomials with coefficient in `C`.
///
/// For example, polynomial `x^2 + 5x + 6` is represented in `Polynomial<u64>` as follows:
///
/// ```ignore
/// Polynomial {
///     coefficients: {
///         2: 1,
///         1: 5,
///         0: 6,
///     },
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Polynomial<C: Semiring> {
    coefficients: HashMap<u64, C>,
}

impl<C: Semiring> Semiring for Polynomial<C> {
    fn zero() -> Self {
        Self {
            coefficients: HashMap::new(),
        }
    }

    fn one() -> Self {
        Self::term(C::one(), 0_u64)
    }

    fn add(&self, rhs: &Self) -> Self {
        let mut out = self.coefficients.clone();

        for (deg, coef_rhs) in rhs.coefficients.iter() {
            match out.entry(*deg) {
                Entry::Vacant(e) => {
                    if *coef_rhs != C::zero() {
                        let _ = e.insert(coef_rhs.clone());
                    }
                }
                Entry::Occupied(mut e) => {
                    let new_val = e.get().add(coef_rhs);
                    if new_val == C::zero() {
                        let _unused = e.remove();
                    } else {
                        *e.get_mut() = new_val;
                    }
                }
            }
        }

        Self { coefficients: out }
    }

    fn mul(&self, rhs: &Self) -> Self {
        // Start from the zero polynomial (prefer empty map)
        let mut out: HashMap<u64, C> = HashMap::new();

        for (deg_l, coef_l) in self.coefficients.iter() {
            for (deg_r, coef_r) in rhs.coefficients.iter() {
                let deg = *deg_l + *deg_r;
                let prod = coef_l.mul(coef_r);

                if prod == C::zero() {
                    continue;
                }

                match out.entry(deg) {
                    Entry::Vacant(e) => {
                        let _ = e.insert(prod);
                    }
                    Entry::Occupied(mut e) => {
                        let new_val = e.get().add(&prod);
                        if new_val == C::zero() {
                            let _unused = e.remove();
                        } else {
                            *e.get_mut() = new_val;
                        }
                    }
                }
            }
        }

        Self { coefficients: out }
    }
}

impl<C: Semiring> Polynomial<C> {
    /// Constructs polynomial `x`.
    pub fn x() -> Self {
        Self::term(C::one(), 1)
    }

    /// Evaluates the polynomial with the given value.
    pub fn eval(&self, value: C) -> C {
        let mut result = C::zero();

        for (deg, coef) in self.coefficients.iter() {
            let mut pow = C::one();
            for _ in 0..*deg {
                pow = pow.mul(&value);
            }
            let term_value = coef.mul(&pow);
            result = result.add(&term_value)
        }

        result
    }

    /// Constructs polynomial `ax^n`.
    pub fn term(a: C, n: u64) -> Self {
        if a == C::zero() {
            return Self {
                coefficients: HashMap::new(),
            };
        }

        let mut coeffs = HashMap::new();
        let _unused = coeffs.insert(n, a);
        Self {
            coefficients: coeffs,
        }
    }
}

impl<C: Semiring> From<C> for Polynomial<C> {
    fn from(value: C) -> Self {
        if value == C::zero() {
            Self {
                coefficients: HashMap::new(),
            }
        } else {
            Self::term(value, 0)
        }
    }
}

/// Given a string `s`, parse it into a `Polynomial<C>`.
/// You may assume that `s` follows the criteria below.
/// Therefore, you do not have to return `Err`.
///
/// Assumptions:
/// - Each term is separated by ` + `.
/// - Each term is one of the following form: `a`, `x`, `ax`, `x^n`, and `ax^n`, where `a` is a
///   `usize` number and `n` is a `u64` number. This `a` should then be converted to a `C` type.
/// - In `a`, it is guaranteed that `a >= 1`.
/// - In `ax` and `ax^n`, it is guaranteed that `a >= 2`.
/// - In `x^n` and `ax^n`, it is guaranteed that `n >= 2`.
/// - All terms have unique degrees.
///
/// Consult `assignment06/grade.rs` for example valid strings.
///
/// Hint: `.split`, `.parse`, and `Polynomial::term`

impl<C: Semiring> std::str::FromStr for Polynomial<C> {
    type Err = (); // Ignore this for now...

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Optional: handle empty string defensively (not required by your assumptions)
        if s.trim().is_empty() {
            return Ok(Self {
                coefficients: HashMap::new(),
            });
        }

        let mut coefficients: HashMap<u64, C> = HashMap::new();

        // Terms are separated by " + " exactly, per assumptions.
        for term in s.split(" + ") {
            let term = term.trim(); // safe even if input is well-formed

            // Parse one term into (degree, coefficient)
            let (deg, coef): (u64, C) = if term == "x" {
                (1, C::one())
            } else if let Some((coef_part, deg_part)) = term.split_once("x^") {
                // "x^n" or "ax^n"
                let deg: u64 = deg_part.parse().unwrap();

                let coef = if coef_part.is_empty() {
                    C::one()
                } else {
                    let a: usize = coef_part.parse().unwrap();
                    from_usize::<C>(a)
                };

                (deg, coef)
            } else if let Some(coef_part) = term.strip_suffix('x') {
                // "ax" (and this also matches "x", but we handled "x" above)
                let coef = if coef_part.is_empty() {
                    C::one()
                } else {
                    let a: usize = coef_part.parse().unwrap();
                    from_usize::<C>(a)
                };

                (1, coef)
            } else {
                // constant term "a"
                let a: usize = term.parse().unwrap();
                (0, from_usize::<C>(a))
            };

            // Assumption: All terms have unique degrees.
            // Insert and (optionally) debug-assert uniqueness.
            let prev = coefficients.insert(deg, coef);
            debug_assert!(prev.is_none(), "duplicate degree {} in input: {}", deg, s);
        }

        Ok(Self { coefficients })
    }
}
