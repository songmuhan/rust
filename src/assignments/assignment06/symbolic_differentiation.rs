//! Symbolic differentiation with rational coefficents.

use core::num;
use std::fmt;
use std::ops::*;

/// Rational number represented by two isize, numerator and denominator.
///
/// Each Rational number should be normalized so that `demoninator` is nonnegative and `numerator` and `demoninator` are coprime.
/// See `normalize` for examples. As a corner case, 0 is represented by Rational { numerator: 0, demoninator: 0 }.
///
/// For "natural use", Rational also overloads standard arithmetic operations, i.e, `+`, `-`, `*`, `/`.
///
/// See [here](https://doc.rust-lang.org/core/ops/index.html) for details.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rational {
    numerator: isize,
    denominator: isize,
}

// Some useful constants.

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
    fn gcd(self, mut a: isize, mut b: isize) -> isize {
        while b != 0 {
            let remainder = a % b;
            a = b;
            b = remainder;
        }
        a
    }
    /// simplify origin result
    pub fn simplify(self) -> Rational {
        let mut negative = false;
        if (self.numerator < 0) != (self.denominator < 0) {
            negative = true;
        }
        let mut numerator = self.numerator.abs();
        let denominator = self.denominator.abs();

        let factor = self.gcd(numerator, denominator);
        if negative {
            numerator = -numerator;
        }
        Self {
            numerator: numerator / factor,
            denominator: denominator / factor,
        }
    }
}

impl Add for Rational {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let numerator = self.numerator * rhs.denominator + self.denominator * rhs.numerator;
        let denominator = self.denominator * rhs.denominator;
        Self {
            numerator,
            denominator,
        }
        .simplify()
    }
}

impl Mul for Rational {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let numerator = self.numerator * rhs.numerator;
        let denominator = self.denominator * rhs.denominator;
        Self {
            numerator,
            denominator,
        }
        .simplify()
    }
}

impl Sub for Rational {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let numerator = self.numerator * rhs.denominator - self.denominator * rhs.numerator;
        let denominator = self.denominator * rhs.denominator;
        Self {
            numerator,
            denominator,
        }
        .simplify()
    }
}

impl Div for Rational {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let numerator = self.numerator * rhs.denominator;
        let denominator = self.denominator * rhs.numerator;
        Self {
            numerator,
            denominator,
        }
        .simplify()
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
        /// coefficent of polynomial. Must be non-zero.
        coeff: Rational,
        /// power of polynomial. Must be non-zero.
        power: Rational,
    },
}

impl SingletonPolynomial {
    /// Creates a new const polynomial.
    pub fn new_c(r: Rational) -> Self {
        SingletonPolynomial::Const(r)
    }

    /// Creates a new polynomial.
    pub fn new_poly(coeff: Rational, power: Rational) -> Self {
        SingletonPolynomial::Polynomial { coeff, power }
    }
}

impl Differentiable for SingletonPolynomial {
    /// HINT: Consult <https://en.wikipedia.org/wiki/Power_rule>
    fn diff(&self) -> Self {
        match self {
            SingletonPolynomial::Const(_) => SingletonPolynomial::Const(ZERO),
            SingletonPolynomial::Polynomial { coeff, power } => {
                let new_coeff = coeff.mul(*power).simplify();
                let new_power = power.sub(ONE).simplify();
                SingletonPolynomial::Polynomial {
                    coeff: new_coeff,
                    power: new_power,
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
    /// Sine function.
    Cosine {
        /// Coefficent
        coeff: Rational,
    },
}

impl Trignometric {
    /// Creates a new sine function.
    pub fn new_sine(coeff: Rational) -> Self {
        Self::Sine { coeff }
    }

    /// Creates a new cosine function.
    pub fn new_cosine(coeff: Rational) -> Self {
        Self::Cosine { coeff }
    }
}

impl Differentiable for Trignometric {
    /// HINT: Consult <https://en.wikipedia.org/wiki/Differentiation_rules#Derivatives_of_trigonometric_functions>
    fn diff(&self) -> Self {
        match self {
            Trignometric::Sine { coeff } => Trignometric::Cosine { coeff: *coeff },
            Trignometric::Cosine { coeff } => Trignometric::Sine {
                coeff: coeff.mul(MINUS_ONE).simplify(),
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
            BaseFuncs::Const(r) => BaseFuncs::Const(r.diff()),
            BaseFuncs::Poly(r) => BaseFuncs::Poly(r.diff()),
            BaseFuncs::Exp(r) => BaseFuncs::Exp(r.diff()),
            BaseFuncs::Trig(r) => BaseFuncs::Trig(r.diff()),
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
        let f = self.as_ref();
        Box::new(f.diff())
    }
}

impl<F: Differentiable> Differentiable for ComplexFuncs<F> {
    /// HINT: Consult <https://en.wikipedia.org/wiki/Differentiation_rules#Elementary_rules_of_differentiation>
    fn diff(&self) -> Self {
        match self {
            ComplexFuncs::Func(f) => ComplexFuncs::Func(f.diff()),
            ComplexFuncs::Add(lhs, rhs) => ComplexFuncs::Add(lhs.diff(), rhs.diff()),
            ComplexFuncs::Sub(lhs, rhs) => ComplexFuncs::Sub(lhs.diff(), rhs.diff()),
            ComplexFuncs::Mul(lhs, rhs) => {
                let first = Box::new(ComplexFuncs::Mul(lhs.diff(), rhs.clone()));
                let second = Box::new(ComplexFuncs::Mul(lhs.clone(), rhs.diff()));
                ComplexFuncs::Add(first, second)
            }
            ComplexFuncs::Div(lhs, rhs) => {
                let first = Box::new(ComplexFuncs::Mul(lhs.diff(), rhs.clone()));
                let second = Box::new(ComplexFuncs::Mul(lhs.clone(), rhs.diff()));
                let upper = Box::new(ComplexFuncs::Sub(first, second));
                let down = Box::new(ComplexFuncs::Mul(rhs.clone(), rhs.clone()));
                ComplexFuncs::Div(upper, down)
            }
            ComplexFuncs::Comp(outer, inner) => {
                let origin_inner = inner.clone();
                let outer = Box::new(ComplexFuncs::Comp(outer.diff(), origin_inner));
                ComplexFuncs::Mul(inner.diff(), outer)
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
    fn evaluate(&self, x: f64) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }
}

impl Evaluate for SingletonPolynomial {
    fn evaluate(&self, x: f64) -> f64 {
        match self {
            SingletonPolynomial::Const(r) => r.evaluate(x),
            SingletonPolynomial::Polynomial { coeff, power } => {
                coeff.evaluate(x) * x.powf(power.evaluate(x))
            }
        }
    }
}

impl Evaluate for Exp {
    fn evaluate(&self, x: f64) -> f64 {
        x.exp()
    }
}

impl Evaluate for Trignometric {
    fn evaluate(&self, x: f64) -> f64 {
        match self {
            Trignometric::Cosine { coeff } => coeff.evaluate(x) * x.cos(),
            Trignometric::Sine { coeff } => coeff.evaluate(x) * x.sin(),
        }
    }
}

impl Evaluate for BaseFuncs {
    fn evaluate(&self, x: f64) -> f64 {
        match self {
            BaseFuncs::Const(r) => r.evaluate(x),
            BaseFuncs::Poly(r) => r.evaluate(x),
            BaseFuncs::Exp(r) => r.evaluate(x),
            BaseFuncs::Trig(r) => r.evaluate(x),
        }
    }
}

impl<F: Evaluate> Evaluate for ComplexFuncs<F> {
    fn evaluate(&self, x: f64) -> f64 {
        match self {
            ComplexFuncs::Func(f) => f.evaluate(x),
            ComplexFuncs::Add(lhs, rhs) => lhs.evaluate(x) + rhs.evaluate(x),
            ComplexFuncs::Sub(lhs, rhs) => lhs.evaluate(x) - rhs.evaluate(x),
            ComplexFuncs::Mul(lhs, rhs) => lhs.evaluate(x) * rhs.evaluate(x),
            ComplexFuncs::Div(lhs, rhs) => lhs.evaluate(x) / rhs.evaluate(x),
            ComplexFuncs::Comp(outer, inner) => outer.evaluate(inner.evaluate(x)),
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
