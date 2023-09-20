//! Church Numerals
//!
//! This exercise involves the use of "Church numerals", a
//! representation of natural numbers using lambda calculus, named after
//! Alonzo Church. Each Church numeral corresponds to a natural number `n`
//! and is represented as a higher-order function that applies a given function `f` `n` times.
//!
//! For more information, see:
//! - <https://en.wikipedia.org/wiki/Church_encoding>
//! - <https://opendsa-server.cs.vt.edu/OpenDSA/Books/PL/html/ChurchNumerals.html>

use std::rc::Rc;

/// Church numerals are represented as higher-order functions that take a function `f`
pub type Church<T> = Rc<dyn Fn(Rc<dyn Fn(T) -> T>) -> Rc<dyn Fn(T) -> T>>;

/// This function returns a Church numeral equivalent of the natural number 1.
/// It takes a function `f` and applies it exactly once.
pub fn one<T: 'static>() -> Church<T> {
    Rc::new(move |f| Rc::new(move |x| f(x)))
}

/// This function returns a Church numeral equivalent of the natural number 2.
/// It takes a function `f` and applies it twice.
pub fn two<T: 'static>() -> Church<T> {
    Rc::new(move |f| Rc::new(move |x| f(f(x))))
}

/// This function represents the Church numeral for zero. As zero applications
/// of `f` should leave the argument unchanged, the function simply returns the input.
pub fn zero<T: 'static>() -> Church<T> {
    Rc::new(|_| Rc::new(|x| x))
}

/// Implement a function to add 1 to a given Church numeral.
pub fn succ<T: 'static>(n: Church<T>) -> Church<T> {
    add(n, one())
}

/// Implement a function to add two Church numerals.
pub fn add<T: 'static>(n: Church<T>, m: Church<T>) -> Church<T> {
    Rc::new(move |f: Rc<dyn Fn(T) -> T>| {
        let n_clone = n.clone();
        let m_clone = m.clone();
        Rc::new(move |x| n_clone(f.clone())(m_clone(f.clone())(x)))
    })
}

/// Implement a function to multiply (mult) two Church numerals.
pub fn mult<T: 'static>(n: Church<T>, m: Church<T>) -> Church<T> {
    Rc::new(move |f: Rc<dyn Fn(T) -> T>| n(m(f)))
}

/// Implement a function to raise one Church numeral to the power of another.
/// This is the Church numeral equivalent of the natural number operation of exponentiation.
/// Given two natural numbers `n` and `m`, the function should return a Church numeral
/// that represents `n` to the power of `m`. The key is to convert `n` and `m` to Church numerals,
/// and then apply the Church numeral for `m` (the exponent) to the Church numeral for `n` (the base).
/// Note: This function should be implemented *WITHOUT* using the `to_usize` or any `pow`-like method.
pub fn exp<T: 'static>(n: usize, m: usize) -> Church<T> {
    let n_church = from_usize(n);
    let m_church = from_usize(m);
    m_church(n_church)
}

/// Implement a function to convert a Church numeral to a usize type.
/* fixme: this implementation is too slow to pass the challenge test.
 *        I have no idea about fast convertion :(.
 */
pub fn to_usize<T: 'static + Default>(n: Church<T>) -> usize {
    use std::cell::RefCell;
    use std::rc::Rc;

    let count = Rc::new(RefCell::new(0));

    // Define a function that increments the counter
    let count_clone = count.clone();
    let inc = Rc::new(move |_| {
        *count_clone.borrow_mut() += 1;
        T::default()
    });

    // Apply the church numeral to our increment function
    let _ = n(inc)(T::default());

    // Return the value of the counter
    let x = *count.borrow();
    x
}

/// Implement a function to convert a usize type to a Church numeral.
pub fn from_usize<T: 'static>(n: usize) -> Church<T> {
    println!("{:?}", n);

    if n == 0 {
        return zero();
    }
    let binary_representation = format!("{:b}", n);
    println!(
        "Binary representation: {}, len:{}",
        binary_representation,
        binary_representation.len()
    );
    let mut current = 1;

    let mut result = zero(); // start with one since we'll double it right away in the loop
    for (i, ch) in binary_representation.chars().enumerate() {
        println!("checking {i}th bit:{ch}");
        result = mult(result.clone(), two());
        if ch == '1' {
            result = succ(result);
        }
    }
    result
}
