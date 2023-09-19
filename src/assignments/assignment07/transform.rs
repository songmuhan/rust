//! Tranformer
use std::marker::PhantomData;
use std::ops::Add;

/// Represents transformation of type `T`.
pub trait Transform<T> {
    /// Transforms value.
    fn transform(&self, value: T) -> T;
}

impl<T1, T2, Tr1: Transform<T1>, Tr2: Transform<T2>> Transform<(T1, T2)> for (Tr1, Tr2) {
    fn transform(&self, value: (T1, T2)) -> (T1, T2) {
        (
            Tr1::transform(&self.0, value.0),
            Tr2::transform(&self.1, value.1),
        )
    }
}

/// Identity transformation.
#[derive(Debug, Clone, Copy)]
pub struct Identity;

impl<T> Transform<T> for Identity {
    fn transform(&self, value: T) -> T {
        value
    }
}

/// Custom transformation.
#[derive(Debug, Clone, Copy)]
pub struct Custom<T, F: Fn(T) -> T> {
    f: F,
    _marker: PhantomData<T>,
}

impl<T, F: Fn(T) -> T> From<F> for Custom<T, F> {
    fn from(f: F) -> Self {
        Self {
            f,
            _marker: PhantomData,
        }
    }
}

impl<T, F: Fn(T) -> T> Transform<T> for Custom<T, F> {
    fn transform(&self, value: T) -> T {
        (self.f)(value)
    }
}

/// Repeats transformation for `n` times.
#[derive(Debug, Clone, Copy)]
pub struct Repeat<T, Tr: Transform<T>> {
    inner: Tr,
    n: u32,
    _marker: PhantomData<T>,
}

impl<T, Tr: Transform<T>> Repeat<T, Tr> {
    /// Creates a new repeat transformation.
    pub fn new(inner: Tr, n: u32) -> Self {
        Repeat {
            inner,
            n,
            _marker: PhantomData,
        }
    }
}

impl<T, Tr: Transform<T>> Transform<T> for Repeat<T, Tr> {
    fn transform(&self, mut value: T) -> T {
        for i in 0..self.n {
            value = self.inner.transform(value)
        }
        value
    }
}

/// Repeats transformation until converges.
#[derive(Debug, Clone, Copy)]
pub struct RepeatUntilConverge<T: Eq, Tr: Transform<T>> {
    inner: Tr,
    _marker: PhantomData<T>,
}

impl<T: Clone + Eq, Tr: Transform<T>> RepeatUntilConverge<T, Tr> {
    /// Creates a new repeat transformation.
    pub fn new(inner: Tr) -> Self {
        RepeatUntilConverge {
            inner,
            _marker: PhantomData,
        }
    }
}

impl<T: Clone + Eq, Tr: Transform<T>> Transform<T> for RepeatUntilConverge<T, Tr> {
    fn transform(&self, mut value: T) -> T {
        let mut previous = value;
        loop {
            let cur = self.inner.transform(previous.clone());
            if cur == previous {
                break;
            } else {
                previous = cur;
            }
        }
        previous
    }
}
