//! Big integer with infinite precision.

use std::fmt;
use std::{iter::zip, ops::*};

use ntest::assert_false;

/// An signed integer with infinite precision implemented with an "carrier" vector of `u32`s.
///
/// The vector is interpreted as a base 2^(32 * (len(carrier) - 1)) integer, where negative
/// integers are represented in their [2's complement form](https://en.wikipedia.org/wiki/Two%27s_complement).
///
/// For example, the vector `vec![44,345,3]` represents the integer
/// `44 * (2^32)^2 + 345 * (2^32) + 3`,
/// and the vector `vec![u32::MAX - 5, u32::MAX - 7]` represents the integer
/// `- (5 * 2^32 + 8)`
///
/// You will implement the `Add` and `Sub` trait for this type.
///
/// Unlike standard fix-sized intergers in Rust where overflow will panic, the carrier is extended to save the overflowed bit.
/// On the contrary, if the precision is too much (e.g, vec![0,0] is used to represent 0, where `vec![0]` is sufficent), the carrier is truncated.
///
/// See [this section](https://en.wikipedia.org/wiki/Two%27s_complement#Arithmetic_operations) for a rouge guide on implementation,
/// while keeping in mind that the carrier should be extended to deal with overflow.
///
/// The `sign_extension()`, `two_complement()`, and `truncate()` are non-mandatory helper methods.
///
/// For testing and debugging pruposes, the `Display` trait is implemented for you, which shows the integer in hexadecimal form.
#[derive(Debug, Clone)]
pub struct BigInt {
    /// The carrier for `BigInt`.
    ///
    /// Note that the carrier should always be non-empty.
    pub carrier: Vec<u32>,
}

impl BigInt {
    /// Create a new `BigInt` from a `usize`.
    pub fn new(n: u32) -> Self {
        BigInt { carrier: vec![n] }
    }

    /// Creates a new `BigInt` from a `Vec<u32>`.
    pub fn new_large(carrier: Vec<u32>) -> Self {
        assert_false!(carrier.is_empty());
        BigInt { carrier }
    }
}

const SIGN_MASK: u32 = 1 << 31;

impl BigInt {
    /// Extend `self` to `len` bits.
    pub fn sign_extension(&self, len: usize) -> Self {
        let sign_extended = if self.carrier.first().unwrap() & SIGN_MASK == 0 {
            vec![0; len - self.carrier.len()]
        } else {
            vec![u32::MAX; len - self.carrier.len()]
        };
        BigInt {
            carrier: [sign_extended, self.carrier.clone()].concat(),
        }
    }

    fn two_complement(&self) -> Self {
        let complement = self.carrier.iter().map(|&x| !x).collect();
        let one = BigInt::new(1);
        BigInt::new_large(complement) + one
    }

    /// Truncate a `BigInt` to the minimum length.
    fn truncate(&self) -> Self {
        let mut carrier = self.carrier.clone();
        if carrier.len() <= 1 {
            Self { carrier }
        } else {
            let mut index = 0;
            let sign = *carrier.first().unwrap();
            if sign != 0 && sign != u32::MAX {
                Self { carrier }
            } else {
                for (i, value) in carrier.iter().enumerate().skip(1) {
                    if *value == sign {
                        index = i;
                    } else {
                        if *value & SIGN_MASK == sign & SIGN_MASK {
                            index = i;
                        }
                        break;
                    }
                }
                Self {
                    carrier: carrier[index..].to_vec(),
                }
            }
        }
    }
}

impl Add for BigInt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let len = self.carrier.len().max(rhs.carrier.len());
        let self_extended = self.sign_extension(len + 1);
        let rhs_extended = rhs.sign_extension(len + 1);

        let mut result = Vec::new();
        let mut carry = 0;
        for (a, b) in self_extended
            .carrier
            .iter()
            .rev()
            .zip(rhs_extended.carrier.iter().rev())
        {
            let sum = a.wrapping_add(*b).wrapping_add(carry);
            if sum < *a || sum < *b {
                carry = 1;
            } else {
                carry = 0;
            }
            result.push(sum);
        }
        if carry > 0 {
            _ = result.pop();
        }
        result.reverse();

        BigInt::new_large(result).truncate()
    }
}

impl Sub for BigInt {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.add(rhs.two_complement())
    }
}

impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Hex formatting so that each u32 can be formatted independently.
        for i in self.carrier.iter() {
            write!(f, "{:08x}", i)?;
        }
        Ok(())
    }
}
