//! Generators
//!
//! HINT: Look at the `generator_grade.rs` file to see how the generator is used.

/// Yielded value. It can be either a value or a stop signal.
enum Yielded<T> {
    Value(T),
    Stop,
}

/// Generator
/// - You can call `next()` method to get the next value.
/// - The generator should stop when it yields `Yielded::Stop`.
///
/// Reference:
/// - [Python generator](https://python-reference.readthedocs.io/en/latest/docs/generator/)
#[derive(Debug)]
pub struct Generator<T, S> {
    state: S,
    f: fn(&mut S) -> Yielded<T>,
}

impl<T, S> Iterator for Generator<T, S> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.f)(&mut self.state) {
            Yielded::Value(t) => Some(t),
            Yielded::Stop => None,
        }
    }
}

/// Returns a generator that yields fibonacci numbers.
///
/// HINT: Consult <https://en.wikipedia.org/wiki/Fibonacci_sequence>
pub fn fib_generator(first: usize, second: usize) -> Generator<usize, (usize, usize)> {
    fn fib_step(st: &mut (usize, usize)) -> Yielded<usize> {
        let out = st.0;
        let next = st.0 + st.1;
        st.0 = st.1;
        st.1 = next;
        Yielded::Value(out)
    }

    Generator {
        state: (first, second),
        f: fib_step,
    }
}

/// Returns a generator that yields collatz numbers.
///
/// HINT: Consult <https://en.wikipedia.org/wiki/Collatz_conjecture>

pub fn collatz_conjecture(start: usize) -> Generator<usize, usize> {
    fn collatz_step(st: &mut usize) -> Yielded<usize> {
        // `0` means "already completed" (sentinel)
        if *st == 0 {
            return Yielded::Stop;
        }

        let current = *st;

        // Yield 1 once, then mark as done (set sentinel) so next call stops.
        if current == 1 {
            *st = 0; // mark as done
            return Yielded::Value(1);
        }

        // Compute the next value in the Collatz sequence.
        let next = if current % 2 == 0 {
            current / 2
        } else {
            // Avoid panic on overflow; prefer checked math with a safe fallback.
            current
                .checked_mul(3)
                .and_then(|v| v.checked_add(1))
                .unwrap_or_else(|| current.wrapping_mul(3).wrapping_add(1))
        };

        // Advance the state for the next step.
        *st = next;

        // Yield the current value.
        Yielded::Value(current)
    }

    // If you want to reject start == 0 instead, you could set state to 0 to stop immediately.
    Generator {
        state: start,
        f: collatz_step,
    }
}
