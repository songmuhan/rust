//! You will implement a number of custom operators.

/// Custom option type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MyOption<T> {
    /// Some value of type `T`.
    MySome(T),
    /// No value.
    MyNone,
}

/// Maps an `MyOption<T>` to `MyOption<U>` by applying a function to a contained value.
///
/// # Examples
///
/// Converts an `MyOption<String>` into an `MyOption<usize>`, consuming the original:
///
/// ```
/// use cs220::assignments::assignment03::{my_map, MyOption};
///
/// fn len(s: String) -> usize {
///     s.len()
/// }
///
/// assert_eq!(my_map(MyOption::MySome(String::from("Hello, World!")), len), MyOption::MySome(13));
/// assert_eq!(my_map(MyOption::MyNone, len), MyOption::MyNone);
/// ```
pub fn my_map<T, U, F: FnOnce(T) -> U>(v: MyOption<T>, f: F) -> MyOption<U> {
    match v {
        MyOption::MyNone => MyOption::MyNone,
        MyOption::MySome(t) => MyOption::MySome(f(t)),
    }
}

/// Returns `MyNone` if the option is `MyNone`, otherwise calls `f` with the wrapped value and returns the result.
///
/// Some languages call this operation flatmap.
///
/// # Examples
///
/// ```
/// use cs220::assignments::assignment03::{MyOption, my_and_then};
///
/// fn pos_then_to_string(x: isize) -> MyOption<String> {
///     if x > 0 {
///         MyOption::MySome(x.to_string())
///     } else {
///         MyOption::MyNone
///     }
/// }
///
/// assert_eq!(my_and_then(MyOption::MySome(2), pos_then_to_string), MyOption::MySome(2.to_string()));
/// assert_eq!(my_and_then(MyOption::MySome(-3), pos_then_to_string), MyOption::MyNone);
/// assert_eq!(my_and_then(MyOption::MyNone, pos_then_to_string), MyOption::MyNone);
/// ```
pub fn my_and_then<T, U, F: FnOnce(T) -> MyOption<U>>(v: MyOption<T>, f: F) -> MyOption<U> {
    match v {
        MyOption::MyNone => MyOption::MyNone,
        MyOption::MySome(t) => f(t),
    }
}

/// Custom operator: `option_op_or(v1, v2, f)`
/// If neither `v1` nor `v2` is `Some`, returns `None`.
/// If exactly one is `Some`, returns the same `Some` value.
/// If both are `Some`, apply the values inside `Some` to `f` and wrap the resulting value inside `Some`.
///
/// # Examples
///
/// ```
/// fn product(a: i32, b: i32) -> i32 {
///     a * b
/// }
///
/// assert_eq!(option_op_or(None, None, product), None);
/// assert_eq!(option_op_or(Some(3), None, product), Some(3));
/// assert_eq!(option_op_or(Some(3), Some(5), product), Some(15));
/// ```
pub fn my_option_op_or<T, F: FnOnce(T, T) -> T>(
    v1: MyOption<T>,
    v2: MyOption<T>,
    f: F,
) -> MyOption<T> {
    match v1 {
        MyOption::MyNone => match v2 {
            MyOption::MyNone => MyOption::MyNone,
            MyOption::MySome(t) => MyOption::MySome(t),
        },
        MyOption::MySome(a) => match v2 {
            MyOption::MyNone => MyOption::MySome(a),
            MyOption::MySome(b) => MyOption::MySome(f(a, b)),
        },
    }
}
