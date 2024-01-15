//! Small problems.

use itertools::Itertools;

const FAHRENHEIT_OFFSET: f64 = 32.0;
const FAHRENHEIT_SCALE: f64 = 5.0 / 9.0;

/// Converts Fahrenheit to Celsius temperature degree.
pub fn fahrenheit_to_celsius(degree: f64) -> f64 {
    (degree - 32.0) * 5.0 / 9.0
}

/// Capitalizes English alphabets (leaving the other characters intact).
pub fn capitalize(input: String) -> String {
    input.to_ascii_uppercase()
}

/// Returns the sum of the given array. (We assume the absence of integer overflow.)
pub fn sum_array(input: &[u64]) -> u64 {
    input.iter().sum()
}

/// Given a non-negative integer, say `n`, return the smallest integer of the form `3^m` that's greater than or equal to `n`.
///
/// For instance, up3(6) = 9, up3(9) = 9, up3(10) = 27. (We assume the absence of integer overflow.)
pub fn up3(n: u64) -> u64 {
    (0..)
        .map(|i| u64::pow(3, i))
        .find(|&value| value >= n)
        .unwrap_or(0)
}

/// Returns the greatest common divisor (GCD) of two non-negative integers. (We assume the absence of integer overflow.)
pub fn gcd(lhs: u64, rhs: u64) -> u64 {
    let (mut m, mut n) = (lhs, rhs);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

/// Returns the array of nC0, nC1, nC2, ..., nCn, where nCk = n! / (k! * (n-k)!). (We assume the absence of integer overflow.)
///
/// Consult <https://en.wikipedia.org/wiki/Pascal%27s_triangle> for computation of binomial coefficients without integer overflow.
pub fn chooses(n: u64) -> Vec<u64> {
    let mut v: Vec<Vec<u64>> = Vec::new();
    for i in 0..=n as usize {
        v.push(vec![1; i + 1]);
        for j in 1..i {
            v[i][j] = v[i - 1][j - 1] + v[i - 1][j];
        }
    }
    v[n as usize].clone()
}

/// Returns the "zip" of two vectors.
///
/// For instance, `zip(vec![1, 2, 3], vec![4, 5])` equals to `vec![(1, 4), (2, 5)]`.
/// Here, `3` is ignored because it doesn't have a partner.
pub fn zip(lhs: Vec<u64>, rhs: Vec<u64>) -> Vec<(u64, u64)> {
    lhs.into_iter().zip(rhs).collect()
}
