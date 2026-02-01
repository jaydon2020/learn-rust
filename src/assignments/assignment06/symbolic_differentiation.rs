//! Symbolic differentiation with rational coefficents.

use std::cmp::min;
use std::fmt;
use std::ops::*;

use ndarray_rand::rand_distr::num_traits::ConstZero;
use ndarray_rand::rand_distr::num_traits::Pow;

/// Rational number represented by two isize, numerator and denominator.
///
/// Each Rational number should be normalized so that `demoninator` is nonnegative and `numerator`
/// and `demoninator` are coprime. See `normalize` for examples. As a corner case, 0 is represented
/// by `Rational { numerator: 0, demoninator: 0 }`.
///
/// For "natural use", it also overloads standard arithmetic operations, i.e, `+`, `-`, `*`, and
/// `/`.
///
/// See [here](https://doc.rust-lang.org/core/ops/index.html) for details.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rational {
    numerator: isize,
    denominator: isize,
}

// Some useful constants.

/// Returns the greatest common divisor (GCD) of two non-negative integers. (We assume the absence
/// of integer overflow.)
fn gcd(a: isize, b: isize) -> isize {
    // Initialize ans, or answer and limit
    let mut ans: isize = 1;
    let limit = min(a, b);

    // Loop from 2 to limit, both inclusive
    for i in 2..(limit + 1) {
        // Check if both a and b are divisible
        if a % i == 0 && b % i == 0 {
            ans = i;
        }
    }
    ans
}

/// Returns the greatest common divisor (GCD) of two non-negative integers. (We assume the absence
/// of integer overflow.)
pub fn lcm(a: isize, b: isize) -> isize {
    // LCM = a*b / gcd
    a * (b / gcd(a, b))
}

/// Zero
pub const ZERO: Rational = Rational::new(0, 0);
/// One
pub const ONE: Rational = Rational::new(1, 1);
/// Minus one
pub const MINUS_ONE: Rational = Rational::new(-1, 1);

impl Rational {
    /// Creates a new rational number.
    pub const fn new(numerator: isize, denominator: isize) -> Self {
        Self {
            numerator,
            denominator,
        }
    }

    fn normalized(&self) -> Self {
        // Handle zero numerator: represent as 0/1
        if self.numerator == 0 {
            return Self {
                numerator: 0,
                denominator: 1,
            };
        }

        // Compute gcd on absolute values
        let g = gcd(self.numerator.abs(), self.denominator.abs());

        let mut n = self.numerator / g;
        let mut d = self.denominator / g;

        // Keep denominator positive
        if d < 0 {
            n = -n;
            d = -d;
        }

        Self {
            numerator: n,
            denominator: d,
        }
    }
}

impl Add for Rational {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.denominator == rhs.denominator {
            Self {
                numerator: self.numerator + rhs.numerator,
                denominator: self.denominator,
            }
            .normalized()
        } else {
            let lcm = lcm(self.denominator, rhs.denominator);
            let lhs_mul = lcm.div(self.denominator);
            let rhs_mul = lcm.div(rhs.denominator);

            Self {
                numerator: self.numerator * lhs_mul + rhs.numerator * rhs_mul,
                denominator: lcm,
            }
            .normalized()
        }
    }
}

impl Mul for Rational {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            numerator: self.numerator * rhs.numerator,
            denominator: self.denominator * rhs.denominator,
        }
        .normalized()
    }
}

impl Sub for Rational {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.denominator == rhs.denominator {
            Self {
                numerator: self.numerator - rhs.numerator,
                denominator: self.denominator,
            }
            .normalized()
        } else {
            let lcm = lcm(self.denominator, rhs.denominator);
            let lhs_mul = lcm / self.denominator;
            let rhs_mul = lcm / rhs.denominator;

            Self {
                numerator: self.numerator * lhs_mul - rhs.numerator * rhs_mul,
                denominator: lcm,
            }
            .normalized()
        }
    }
}

impl Div for Rational {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        if rhs.numerator < 0 {
            Self {
                numerator: -self.numerator * rhs.denominator,
                denominator: -self.denominator * rhs.numerator,
            }
            .normalized()
        } else {
            Self {
                numerator: self.numerator * rhs.denominator,
                denominator: self.denominator * rhs.numerator,
            }
            .normalized()
        }
    }
}

/// Differentiable functions.
///
/// For simplicity, we only consider infinitely differentiable functions.
pub trait Differentiable: Clone {
    /// Differentiate.
    ///
    /// Since the return type is `Self`, this trait can only be implemented
    /// for types that are closed under differentiation.
    fn diff(&self) -> Self;
}

impl Differentiable for Rational {
    /// HINT: Consult <https://en.wikipedia.org/wiki/Differentiation_rules#Constant_term_rule>
    fn diff(&self) -> Self {
        ZERO
    }
}

/// Singleton polynomial.
///
/// Unlike regular polynomials, this type only represents a single term.
/// The `Const` variant is included to make `Polynomial` closed under differentiation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SingletonPolynomial {
    /// Constant polynomial.
    Const(Rational),
    /// Non-const polynomial.
    Polynomial {
        /// Coefficent of polynomial. Must be non-zero.
        coeff: Rational,
        /// Power of polynomial. Must be non-zero.
        power: Rational,
    },
}

impl SingletonPolynomial {
    /// Creates a new const polynomial.
    pub fn new_c(r: Rational) -> Self {
        Self::Const(r)
    }

    /// Creates a new polynomial.
    pub fn new_poly(coeff: Rational, power: Rational) -> Self {
        Self::Polynomial { coeff, power }
    }
}

impl Differentiable for SingletonPolynomial {
    /// HINT: Consult <https://en.wikipedia.org/wiki/Power_rule>
    fn diff(&self) -> Self {
        match self {
            SingletonPolynomial::Const(rational) => Self::Const(ZERO),
            SingletonPolynomial::Polynomial { coeff, power } => {
                if *power == ONE {
                    Self::Const(*coeff)
                } else {
                    Self::Polynomial {
                        coeff: *coeff * *power,
                        power: *power - ONE,
                    }
                }
            }
        }
    }
}

/// Expoential function.(`e^x`)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Exp;

impl Exp {
    /// Creates a new exponential function.
    pub fn new() -> Self {
        Exp
    }
}

impl Default for Exp {
    fn default() -> Self {
        Self::new()
    }
}

impl Differentiable for Exp {
    /// HINT: Consult <https://en.wikipedia.org/wiki/Differentiation_rules#Derivatives_of_exponential_and_logarithmic_functions>
    fn diff(&self) -> Self {
        Exp
    }
}

/// Trigonometric functions.
///
/// The trig fucntions carry their coefficents to be closed under differntiation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trignometric {
    /// Sine function.
    Sine {
        /// Coefficent
        coeff: Rational,
    },
    /// Cosine function.
    Cosine {
        /// Coefficent
        coeff: Rational,
    },
}

impl Trignometric {
    /// Creates a new sine function.
    pub fn new_sine(coeff: Rational) -> Self {
        Trignometric::Sine { coeff }
    }

    /// Creates a new cosine function.
    pub fn new_cosine(coeff: Rational) -> Self {
        Trignometric::Cosine { coeff }
    }
}

impl Differentiable for Trignometric {
    /// HINT: Consult <https://en.wikipedia.org/wiki/Differentiation_rules#Derivatives_of_trigonometric_functions>
    fn diff(&self) -> Self {
        match self {
            Trignometric::Sine { coeff } => Trignometric::Cosine { coeff: *coeff },
            Trignometric::Cosine { coeff } => Trignometric::Sine {
                coeff: *coeff * Rational::new(-1, 1),
            },
        }
    }
}

/// Basic functions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseFuncs {
    /// Constant
    Const(Rational),
    /// Polynomial
    Poly(SingletonPolynomial),
    /// Exponential
    Exp(Exp),
    /// Trignometirc
    Trig(Trignometric),
}

impl Differentiable for BaseFuncs {
    fn diff(&self) -> Self {
        match self {
            BaseFuncs::Const(rational) => BaseFuncs::Const(ZERO),
            BaseFuncs::Poly(singleton_polynomial) => BaseFuncs::Poly(singleton_polynomial.diff()),
            BaseFuncs::Exp(exp) => BaseFuncs::Exp(*exp),
            BaseFuncs::Trig(trignometric) => BaseFuncs::Trig(trignometric.diff()),
        }
    }
}

/// Complex functions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComplexFuncs<F> {
    /// Basic functions
    Func(F),
    /// Addition
    Add(Box<ComplexFuncs<F>>, Box<ComplexFuncs<F>>),
    /// Subtraction
    Sub(Box<ComplexFuncs<F>>, Box<ComplexFuncs<F>>),
    /// Multipliciation
    Mul(Box<ComplexFuncs<F>>, Box<ComplexFuncs<F>>),
    /// Division
    Div(Box<ComplexFuncs<F>>, Box<ComplexFuncs<F>>),
    /// Composition
    Comp(Box<ComplexFuncs<F>>, Box<ComplexFuncs<F>>),
}

impl<F: Differentiable> Differentiable for Box<F> {
    fn diff(&self) -> Self {
        Box::new((**self).diff())
    }
}

impl<F: Differentiable> Differentiable for ComplexFuncs<F> {
    fn diff(&self) -> Self {
        match self {
            ComplexFuncs::Func(f) => ComplexFuncs::Func(f.diff()),

            ComplexFuncs::Add(l, r) => ComplexFuncs::Add(l.diff(), r.diff()),

            ComplexFuncs::Sub(l, r) => ComplexFuncs::Sub(l.diff(), r.diff()),

            // Product rule: (u*v)' = u'*v + u*v'
            ComplexFuncs::Mul(l, r) => {
                let u = l.clone();
                let v = r.clone();

                ComplexFuncs::Add(
                    Box::new(ComplexFuncs::Mul(l.diff(), v)),
                    Box::new(ComplexFuncs::Mul(u, r.diff())),
                )
            }

            // Quotient rule: (u/v)' = (u'*v - u*v') / (v*v)
            ComplexFuncs::Div(l, r) => {
                let u = l.clone();
                let v = r.clone();

                let numerator = ComplexFuncs::Sub(
                    Box::new(ComplexFuncs::Mul(l.diff(), v.clone())),
                    Box::new(ComplexFuncs::Mul(u, r.diff())),
                );

                let denominator = ComplexFuncs::Mul(v.clone(), v);

                ComplexFuncs::Div(Box::new(numerator), Box::new(denominator))
            }

            // Chain rule: (f ∘ g)' = (f' ∘ g) * g'
            ComplexFuncs::Comp(f, g) => {
                let g_clone = g.clone();

                ComplexFuncs::Mul(
                    Box::new(ComplexFuncs::Comp(
                        Box::new((**f).diff()), // derivative of outer function
                        g_clone,                // inner unchanged
                    )),
                    g.diff(), // times derivative of inner
                )
            }
        }
    }
}

/// Evaluate functions.
pub trait Evaluate {
    ///  Evaluate `self` at `x`.
    fn evaluate(&self, x: f64) -> f64;
}

impl Evaluate for Rational {
    fn evaluate(&self, _x: f64) -> f64 {
        if self.denominator == 0 {
            // This covers your ZERO = 0/0 representation
            0.0
        } else {
            self.numerator as f64 / self.denominator as f64
        }
    }
}

impl Evaluate for SingletonPolynomial {
    fn evaluate(&self, x: f64) -> f64 {
        match self {
            SingletonPolynomial::Const(rational) => rational.evaluate(x),
            SingletonPolynomial::Polynomial { coeff, power } => {
                coeff.evaluate(x).mul(x.pow(power.evaluate(x)))
            }
        }
    }
}

impl Evaluate for Exp {
    fn evaluate(&self, x: f64) -> f64 {
        x.exp() // e^x
    }
}

impl Evaluate for Trignometric {
    fn evaluate(&self, x: f64) -> f64 {
        match self {
            Trignometric::Sine { coeff } => coeff.evaluate(x) * x.sin(),
            Trignometric::Cosine { coeff } => coeff.evaluate(x) * x.cos(),
        }
    }
}

impl Evaluate for BaseFuncs {
    fn evaluate(&self, x: f64) -> f64 {
        match self {
            BaseFuncs::Const(rational) => rational.evaluate(x),
            BaseFuncs::Poly(singleton_polynomial) => singleton_polynomial.evaluate(x),
            BaseFuncs::Exp(exp) => exp.evaluate(x),
            BaseFuncs::Trig(trignometric) => trignometric.evaluate(x),
        }
    }
}

impl<F: Evaluate> Evaluate for ComplexFuncs<F> {
    fn evaluate(&self, x: f64) -> f64 {
        match self {
            ComplexFuncs::Func(f) => f.evaluate(x),
            ComplexFuncs::Add(l, r) => l.evaluate(x) + r.evaluate(x),
            ComplexFuncs::Sub(l, r) => l.evaluate(x) - r.evaluate(x),
            ComplexFuncs::Mul(l, r) => l.evaluate(x) * r.evaluate(x),
            ComplexFuncs::Div(l, r) => l.evaluate(x) / r.evaluate(x),

            // Composition: f(g(x))
            ComplexFuncs::Comp(f, g) => {
                let gx = g.evaluate(x);
                f.evaluate(gx)
            }
        }
    }
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if *self == ZERO {
            return write!(f, "0");
        } else if self.denominator == 1 {
            return write!(f, "{}", self.numerator);
        }
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

impl fmt::Display for SingletonPolynomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Const(r) => write!(f, "{r}"),
            Self::Polynomial { coeff, power } => {
                // coeff or power is zero
                if *coeff == ZERO {
                    return write!(f, "0");
                } else if *power == ZERO {
                    return write!(f, "{coeff}");
                }

                // Standard form of px^q
                let coeff = if *coeff == ONE {
                    "".to_string()
                } else if *coeff == MINUS_ONE {
                    "-".to_string()
                } else {
                    format!("({coeff})")
                };
                let var = if *power == ONE {
                    "x".to_string()
                } else {
                    format!("x^({power})")
                };
                write!(f, "{coeff}{var}")
            }
        }
    }
}

impl fmt::Display for Exp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "exp(x)")
    }
}

impl fmt::Display for Trignometric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (func, coeff) = match self {
            Trignometric::Sine { coeff } => ("sin(x)", coeff),
            Trignometric::Cosine { coeff } => ("cos(x)", coeff),
        };

        if *coeff == ZERO {
            write!(f, "0")
        } else if *coeff == ONE {
            write!(f, "{func}")
        } else if *coeff == MINUS_ONE {
            write!(f, "-{func}")
        } else {
            write!(f, "({coeff}){func}")
        }
    }
}

impl fmt::Display for BaseFuncs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Const(r) => write!(f, "{r}"),
            Self::Poly(p) => write!(f, "{p}"),
            Self::Exp(e) => write!(f, "{e}"),
            Self::Trig(t) => write!(f, "{t}"),
        }
    }
}

impl<F: Differentiable + fmt::Display> fmt::Display for ComplexFuncs<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ComplexFuncs::Func(func) => write!(f, "{func}"),
            ComplexFuncs::Add(l, r) => write!(f, "({l} + {r})"),
            ComplexFuncs::Sub(l, r) => write!(f, "({l} - {r})"),
            ComplexFuncs::Mul(l, r) => write!(f, "({l} * {r})"),
            ComplexFuncs::Div(l, r) => write!(f, "({l} / {r})"),
            ComplexFuncs::Comp(l, r) => write!(f, "({l} ∘ {r})"),
        }
    }
}
