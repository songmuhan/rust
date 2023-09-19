//! Implement your own minimal `itertools` crate.

use std::collections::HashSet;
use std::hash::Hash;

/// Iterator that iterates over the given iterator and returns only unique elements.
#[allow(missing_debug_implementations)]
pub struct Unique<I: Iterator> {
    // TODO: remove `_marker` and add necessary fields as you want
    iter: I,
    seen: HashSet<I::Item>,
}

impl<I: Iterator> Iterator for Unique<I>
where
    I::Item: Eq + Hash + Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .by_ref()
            .find(|item| self.seen.insert(item.clone()))
    }
}

/// Iterator that chains two iterators together.
#[allow(missing_debug_implementations)]
pub struct Chain<I1: Iterator, I2: Iterator> {
    // TODO: remove `_marker` and add necessary fields as you want
    first: I1,
    second: I2,
}

impl<T: Eq + Hash + Clone, I1: Iterator<Item = T>, I2: Iterator<Item = T>> Iterator
    for Chain<I1, I2>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let rc = self.first.next();
        if rc.is_none() {
            self.second.next()
        } else {
            rc
        }
    }
}

/// Iterator that iterates over given iterator and enumerates each element.
#[allow(missing_debug_implementations)]
pub struct Enumerate<I: Iterator> {
    // TODO: remove `_marker` and add necessary fields as you want
    cur: usize,
    iter: I,
}

impl<I: Iterator> Iterator for Enumerate<I> {
    type Item = (usize, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let iter_rc = self.iter.next();
        if let Some(item) = self.iter.next() {
            let rc = Some((self.cur, item));
            self.cur += 1;
            rc
        } else {
            None
        }
    }
}

/// Iterator that zips two iterators together.
///
/// If one iterator is longer than the other one, the remaining elements for the longer element
/// should be ignored.
#[allow(missing_debug_implementations)]
pub struct Zip<I1: Iterator, I2: Iterator> {
    // TODO: remove `_marker` and add necessary fields as you want
    first: I1,
    second: I2,
}

impl<I1: Iterator, I2: Iterator> Iterator for Zip<I1, I2> {
    type Item = (I1::Item, I2::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let first_iter = self.first.next();
        let second_iter = self.second.next();
        match (first_iter, second_iter) {
            (Some(first), Some(second)) => Some((first, second)),
            _ => None,
        }
    }
}

/// My Itertools trait.
pub trait MyIterTools: Iterator {
    /// Returns an iterator that iterates over the `self` and returns only unique elements.
    fn my_unique(self) -> Unique<Self>
    where
        Self: Sized,
    {
        Unique {
            iter: self,
            seen: HashSet::new(),
        }
    }

    /// Returns an iterator that chains `self` and `other` together.
    fn my_chain<I: Iterator>(self, other: I) -> Chain<Self, I>
    where
        Self: Sized,
    {
        Chain {
            first: self,
            second: other,
        }
    }

    /// Returns an iterator that iterates over `self` and enumerates each element.
    fn my_enumerate(self) -> Enumerate<Self>
    where
        Self: Sized,
    {
        Enumerate { cur: 0, iter: self }
    }

    /// Returns an iterator that zips `self` and `other` together.
    fn my_zip<I: Iterator>(self, other: I) -> Zip<Self, I>
    where
        Self: Sized,
    {
        Zip {
            first: self,
            second: other,
        }
    }

    /// Foldleft for `MyIterTools`
    fn my_fold<T, F>(mut self, init: T, mut f: F) -> T
    where
        Self: Sized,
        F: FnMut(Self::Item, T) -> T,
    {
        let mut sum = init;
        for value in self {
            sum = f(value, sum);
        }
        sum
    }
}

impl<T: ?Sized> MyIterTools for T where T: Iterator {}
