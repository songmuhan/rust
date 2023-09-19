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
#[allow(missing_debug_implementations)]
pub struct Generator<T, S> {
    state: S,
    f: fn(&mut S) -> Yielded<T>,
}

impl<T, S> Iterator for Generator<T, S> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = (self.f)(&mut self.state);
        match result {
            Yielded::Value(v) => Some(v),
            Yielded::Stop => None,
        }
    }
}

/// Returns a generator that yields fibonacci numbers.
///
/// HINT: Consult <https://en.wikipedia.org/wiki/Fibonacci_sequence>
pub fn fib_generator(first: usize, second: usize) -> Generator<usize, (usize, usize)> {
    fn compute(state: &mut (usize, usize)) -> Yielded<usize> {
        let rc = Yielded::Value(state.0);
        *state = (state.1, state.0 + state.1);
        rc
    }
    Generator {
        state: (first, second),
        f: compute,
    }
}

/// Returns a generator that yields collatz numbers.
///
/// HINT: Consult <https://en.wikipedia.org/wiki/Collatz_conjecture>
pub fn collatz_conjecture(start: usize) -> Generator<usize, usize> {
    fn compute(state: &mut usize) -> Yielded<usize> {
        if *state == 0 {
            return Yielded::Stop;
        }
        let rc = Yielded::Value(*state);
        if *state == 1 {
            *state = 0;
        } else if *state % 2 == 0 {
            *state /= 2;
        } else {
            *state = *state * 3 + 1;
        }
        rc
    }

    Generator {
        state: start,
        f: compute,
    }
}
