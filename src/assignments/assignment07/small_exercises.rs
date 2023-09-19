//! Implement functions usint `Iterator` trait

use rayon::vec;

struct FindIter<'s, T: Eq> {
    query: &'s [T],
    base: &'s [T],
    curr: usize,
}

impl<T: Eq> Iterator for FindIter<'_, T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.curr + self.query.len() <= self.base.len() {
            if self.base[self.curr..self.curr + self.query.len()] == *self.query {
                let index = self.curr;
                self.curr += 1;
                return Some(index);
            }
            self.curr += 1;
        }
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
}

impl<T: std::ops::Add<Output = T> + Copy> FibIter<T> {
    fn new(first: T, second: T) -> Self {
        FibIter { first, second }
    }
}

impl<T> Iterator for FibIter<T>
where
    T: std::ops::Add<Output = T> + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let rc = Some(self.first);
        (self.first, self.second) = (self.second, self.first + self.second);
        rc
    }
}

/// Returns and iterator over the generic fibonacci sequence starting from `first` and `second`.
/// This is a generic version of `fibonacci` function, which works for any types that implements `std::ops::Add` trait.
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
    current: isize,
    end: isize,
    step: isize,
}

impl RangeIter {
    fn new(endpoints: (Endpoint, Endpoint), step: isize) -> Self {
        RangeIter {
            current: match endpoints.0 {
                Endpoint::Inclusive(v) => v,
                Endpoint::Exclusive(v) => {
                    if step > 0 {
                        v + 1
                    } else {
                        v - 1
                    }
                }
            },
            end: match endpoints.1 {
                Endpoint::Inclusive(v) => v,
                Endpoint::Exclusive(v) => {
                    if step > 0 {
                        v - 1
                    } else {
                        v + 1
                    }
                }
            },
            step,
        }
    }
}

impl Iterator for RangeIter {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.step > 0 && self.current > self.end) || (self.step < 0 && self.current < self.end)
        {
            return None;
        }

        let value = self.current;
        self.current += self.step;
        Some(value)
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
    divisor: u64,
    saved: Vec<u64>,
}

impl Iterator for Divisors {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        while self.divisor * self.divisor < self.n {
            if self.n % self.divisor == 0 {
                let divisor = self.divisor;
                self.divisor += 1;
                self.saved.push(self.n / divisor);
                return Some(divisor);
            }
            self.divisor += 1;
        }

        if self.divisor * self.divisor == self.n {
            let divisor = self.divisor;
            self.divisor += 1;
            return Some(divisor);
        }

        self.saved.pop()
    }
}

/// Returns an iterator over the divisors of n.
pub fn divisors(n: u64) -> impl Iterator<Item = u64> {
    Divisors {
        n,
        divisor: 1,
        saved: vec![],
    }
}
