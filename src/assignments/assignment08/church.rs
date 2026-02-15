//! Church Numerals
//!
//! This exercise involves the use of "Church numerals", a
//! representation of natural numbers using lambda calculus, named after
//! Alonzo Church. Each Church numeral corresponds to a natural number `n`
//! and is represented as a higher-order function that applies a given function `f` `n` times.
//!
//! For more information, see:
//! - <https://en.wikipedia.org/wiki/Church_encoding>
//! - <https://opendsa-server.cs.vt.edu/OpenDSA/Books/PL/html/ChurchNumerals.html>

use std::cell::RefCell;
use std::rc::Rc;

/// Church numerals are represented as higher-order functions that take a function `f`
pub type Church<T> = Rc<dyn Fn(Rc<dyn Fn(T) -> T>) -> Rc<dyn Fn(T) -> T>>;

/// This function returns a Church numeral equivalent of the natural number 1.
/// It takes a function `f` and applies it exactly once.
pub fn one<T: 'static>() -> Church<T> {
    Rc::new(move |f| Rc::new(move |x| f(x)))
}

/// This function returns a Church numeral equivalent of the natural number 2.
/// It takes a function `f` and applies it twice.
pub fn two<T: 'static>() -> Church<T> {
    Rc::new(move |f| Rc::new(move |x| f(f(x))))
}

/// This function represents the Church numeral for zero. As zero applications
/// of `f` should leave the argument unchanged, the function simply returns the input.
pub fn zero<T: 'static>() -> Church<T> {
    Rc::new(|_| Rc::new(|x| x))
}

/// Implement a function to add 1 to a given Church numeral.
pub fn succ<T: 'static>(n: Church<T>) -> Church<T> {
    Rc::new(move |f| {
        let f_n = n(Rc::clone(&f));
        Rc::new(move |x| f(f_n(x)))
    })
}

/// Implement a function to add two Church numerals.
pub fn add<T: 'static>(n: Church<T>, m: Church<T>) -> Church<T> {
    Rc::new(move |f| {
        let f_n = n(Rc::clone(&f));
        let f_m = n(Rc::clone(&f));
        Rc::new(move |x| f_m(f_n(x)))
    })
}

/// Implement a function to multiply (mult) two Church numerals.
pub fn mult<T: 'static>(n: Church<T>, m: Church<T>) -> Church<T> {
    Rc::new(move |f| {
        let f_n = n(Rc::clone(&f));
        let f_m_n = m(Rc::clone(&f_n));
        Rc::new(move |x| f_m_n(x))
    })
}

/// Implement a function to raise one Church numeral to the power of another.
/// This is the Church numeral equivalent of the natural number operation of exponentiation.
/// Given two natural numbers `n` and `m`, the function should return a Church numeral
/// that represents `n` to the power of `m`. The key is to convert `n` and `m` to Church numerals,
/// and then apply the Church numeral for `m` (the exponent) to the Church numeral for `n` (the
/// base). Note: This function should be implemented *WITHOUT* using the `to_usize` or any
/// `pow`-like method.
pub fn exp<T: 'static>(n: usize, m: usize) -> Church<T> {
    // ACTION ITEM: Uncomment the following lines and replace `todo!()` with your code.
    let church_n: Church<T> = from_usize(n);
    let church_m: Church<T> = from_usize(m);
    Rc::new(move |f| {
        let f_m = Rc::clone(&church_m);
        let f_n = Rc::clone(&church_n);
        let mut result = f.clone();
        for _ in 0..n {
            result = f_n(result.clone());
        }
        result
    })
}

/// Implement a function to convert a Church numeral to a usize type.
pub fn to_usize<T: 'static + Default>(n: Church<T>) -> usize {
    let count = Rc::new(RefCell::new(0));

    let c = Rc::clone(&count);
    let default_function: Rc<dyn Fn(T) -> T> = Rc::new(move |x| {
        let mut count_mut = c.borrow_mut();
        *count_mut += 1;
        x
    });

    let result_function = n(default_function);

    let _unused = result_function(Default::default());

    let result = *count.borrow();
    result
}

/// Implement a function to convert a usize type to a Church numeral.
pub fn from_usize<T: 'static>(n: usize) -> Church<T> {
    let mut result = zero();
    for _ in 0..n {
        result = succ(result);
    }
    result
}
