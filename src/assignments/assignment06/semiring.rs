//! Semiring

use std::{collections::HashMap, fmt::Debug};

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
        0
    }

    fn one() -> Self {
        1
    }

    fn add(&self, rhs: &Self) -> Self {
        self + rhs
    }

    fn mul(&self, rhs: &Self) -> Self {
        self * rhs
    }
}

impl Semiring for i64 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }

    fn add(&self, rhs: &Self) -> Self {
        self + rhs
    }

    fn mul(&self, rhs: &Self) -> Self {
        self * rhs
    }
}

impl Semiring for f64 {
    fn zero() -> Self {
        0.0
    }

    fn one() -> Self {
        1.0
    }

    fn add(&self, rhs: &Self) -> Self {
        self + rhs
    }

    fn mul(&self, rhs: &Self) -> Self {
        self * rhs
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
        Polynomial {
            coefficients: HashMap::new(),
        }
    }

    fn one() -> Self {
        let mut map = HashMap::new();
        let _ = map
            .entry(0)
            .and_modify(|origin| *origin = C::one())
            .or_insert(C::one());
        Polynomial { coefficients: map }
    }

    fn add(&self, rhs: &Self) -> Self {
        let mut map = self.coefficients.clone();
        for (key, value) in &rhs.coefficients {
            let _ = map
                .entry(*key)
                .and_modify(|origin| *origin = value.add(origin))
                .or_insert(value.clone());
        }
        map.retain(|_, v| *v != C::zero());
        Polynomial { coefficients: map }
    }

    fn mul(&self, rhs: &Self) -> Self {
        let mut map: HashMap<u64, C> = HashMap::new();
        for (outer_key, outer_value) in &self.coefficients {
            for (inner_key, inner_value) in &rhs.coefficients {
                let value = outer_value.mul(inner_value);
                let key = outer_key.add(inner_key);
                let _ = map
                    .entry(key)
                    .and_modify(|origin| *origin = value.add(origin))
                    .or_insert(value);
            }
        }
        map.retain(|_, v| *v != C::zero());
        Polynomial { coefficients: map }
    }
}

impl<C: Semiring> Polynomial<C> {
    /// Constructs polynomial `x`.
    pub fn x() -> Self {
        let mut map = HashMap::new();
        let _ = map
            .entry(1)
            .and_modify(|origin| *origin = C::one())
            .or_insert(C::one());
        Polynomial { coefficients: map }
    }

    /// Evaluates the polynomial with the given value.
    pub fn eval(&self, value: C) -> C {
        fn pow<C: Semiring>(value: &C, power: u64) -> C {
            let mut rc = C::one();
            if power != 0 {
                for i in 1..=power {
                    rc = rc.mul(value);
                }
            }
            rc
        }
        let mut rc = C::zero();
        for (n, a) in &self.coefficients {
            let mut result = pow(&value, *n).mul(a);
            rc = rc.add(&result);
        }
        rc
    }

    /// Constructs polynomial `ax^n`.
    pub fn term(a: C, n: u64) -> Self {
        let mut map = HashMap::new();
        let _ = map.insert(n, a);
        Polynomial { coefficients: map }
    }
}

impl<C: Semiring> From<C> for Polynomial<C> {
    fn from(value: C) -> Self {
        let mut map = HashMap::new();
        let _ = map.insert(0, value);
        Polynomial { coefficients: map }
    }
}

/// Given a string `s`, parse it into a `Polynomial<C>`.
/// You may assume that `s` follows the criteria below.
/// Therefore, you do not have to return `Err`.
///
/// Assumptions:
/// - Each term is separated by ` + `.
/// - Each term is one of the following form:
///   `a`, `x`, `ax`, `x^n`, and `ax^n`,
///   where `a` is a `usize` number and `n` is a `u64` number.
///   This `a` should then be converted to a `C` type.
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
        let terms = s.split(" + ");
        let mut result = Polynomial::zero();

        for term in terms {
            let (mut a, mut n) = (1, 0);
            if term.contains('x') {
                /* find a  */
                if term.starts_with(|ch: char| ch.is_ascii_digit()) {
                    let pos = term.find('x').unwrap();
                    a = term[0..pos].parse::<usize>().unwrap();
                }

                /* find n */
                if let Some(pos) = term.find('^') {
                    n = term[pos + 1..].parse::<u64>().unwrap();
                } else {
                    n = 1;
                }
            } else {
                a = term.parse::<usize>().unwrap();
            }
            let term = Polynomial::term(from_usize(a), n);
            result = Polynomial::add(&result, &term.clone());
        }
        Ok(result)
    }
}
