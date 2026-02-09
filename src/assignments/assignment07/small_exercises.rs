//! Implement functions using `Iterator` trait

struct FindIter<'s, T: Eq> {
    query: &'s [T],
    base: &'s [T],
    curr: usize,
}

impl<T: Eq> Iterator for FindIter<'_, T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.base.len();
        let m = self.query.len();

        // Define empty-pattern semantics: yield every position 0..=n
        if m == 0 {
            if self.curr <= n {
                let i = self.curr;
                self.curr += 1;
                return Some(i);
            }
            return None;
        }

        let mut i = self.curr;
        // Only positions where a full match can start
        while i + m <= n {
            if self.base[i..].starts_with(self.query) {
                let pos = i;
                // Overlapping behavior: slide by 1. For non-overlapping, use i + m.
                self.curr = i + 1;
                return Some(pos);
            }
            i += 1;
        }

        // Exhausted: make subsequent calls cheap
        self.curr = n;
        None
    }
}

/// Returns an iterator over substring query indexes in the base.
pub fn find<'s, T: Eq>(query: &'s [T], base: &'s [T]) -> impl 's + Iterator<Item = usize> {
    FindIter {
        query,
        base,
        curr: 0,
    }
}

/// Implement generic fibonacci iterator
struct FibIter<T> {
    // TODO: remove `_marker` and add necessary fields as you want
    first: T,
    second: T,
    _marker: std::marker::PhantomData<T>,
}

impl<T: std::ops::Add<Output = T> + Copy> FibIter<T> {
    fn new(first: T, second: T) -> Self {
        FibIter {
            first,
            second,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T> Iterator for FibIter<T>
where
    T: std::ops::Add<Output = T> + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.first;
        let next = self.first + self.second;
        self.first = self.second;
        self.second = next;
        Some(out)
    }
}

/// Returns and iterator over the generic fibonacci sequence starting from `first` and `second`.
/// This is a generic version of `fibonacci` function, which works for any types that implements
/// `std::ops::Add` trait.
pub fn fib<T>(first: T, second: T) -> impl Iterator<Item = T>
where
    T: std::ops::Add<Output = T> + Copy,
{
    FibIter::new(first, second)
}

/// Endpoint of range, inclusive or exclusive.
#[derive(Debug)]
pub enum Endpoint {
    /// Inclusive endpoint
    Inclusive(isize),

    /// Exclusive endpoint
    Exclusive(isize),
}

struct RangeIter {
    // TODO: add necessary fields as you want
    left_val: isize,
    right_val: isize,
    step: isize,
}

impl RangeIter {
    fn new(endpoints: (Endpoint, Endpoint), step: isize) -> Self {
        let lhs = match endpoints.0 {
            Endpoint::Inclusive(val) => val,
            Endpoint::Exclusive(val) => {
                if step > 0 {
                    val + 1
                } else {
                    val - 1
                }
            }
        };

        let rhs = match endpoints.1 {
            Endpoint::Inclusive(val) => {
                if step > 0 {
                    val + 1
                } else {
                    val - 1
                }
            }
            Endpoint::Exclusive(val) => val,
        };

        Self {
            left_val: lhs,
            right_val: rhs,
            step,
        }
    }
}

impl Iterator for RangeIter {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.step > 0 {
            if self.left_val >= self.right_val {
                None
            } else {
                let ret = self.left_val;
                self.left_val += self.step;
                Some(ret)
            }
        } else if self.left_val <= self.right_val {
            None
        } else {
            let ret = self.left_val;
            self.left_val += self.step;
            Some(ret)
        }
    }
}

/// Returns an iterator over the range [left, right) with the given step.
pub fn range(left: Endpoint, right: Endpoint, step: isize) -> impl Iterator<Item = isize> {
    RangeIter::new((left, right), step)
}

/// Write an iterator that returns all divisors of n in increasing order.
/// Assume n > 0.
///
/// Hint: trying all candidates from 1 to n will most likely time out!
/// To optimize it, make use of the following fact:
/// if x is a divisor of n that is greater than sqrt(n),
/// then n/x is a divisor of n that is smaller than sqrt(n).
struct Divisors {
    n: u64,
    // TODO: you may define additional fields here
    i: u64,
    large: Vec<u64>,
}

impl Iterator for Divisors {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        // Scan small divisors
        while self.i * self.i <= self.n {
            let i = self.i;
            self.i += 1;

            if self.n % i == 0 {
                let j = self.n / i;
                if j != i {
                    self.large.push(j);
                }
                return Some(i);
            }
        }

        self.large.pop()
    }
}

/// Returns an iterator over the divisors of n.
pub fn divisors(n: u64) -> impl Iterator<Item = u64> {
    Divisors {
        n,
        // TODO: you may define additional fields here
        i: 1,
        large: Vec::new(),
    }
}
