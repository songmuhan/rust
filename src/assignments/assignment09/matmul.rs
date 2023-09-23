//! Simple matrix multiplication

use itertools::*;

/// elementwise vector addition
///
/// # Exmaple
///
/// ```
/// use cs220::assignments::assignment09::vec_add;
///
/// let vec1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let vec2 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let res = vec_add(&vec1, &vec2);
/// assert_eq!(res, vec![2.0, 4.0, 6.0, 8.0, 10.0]);
/// ```
pub fn vec_add(lhs: &[f64], rhs: &[f64]) -> Vec<f64> {
    lhs.iter().zip(rhs).map(|(v1, v2)| v1 + v2).collect_vec()
}

/// dot product of two arrays
///
/// You don't know how to calculate dot product?
/// See <https://mathinsight.org/dot_product_examples>
///
/// # Exmaple
///
/// ```
/// use cs220::assignments::assignment09::dot_product;
///
/// let vec1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let vec2 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let res = dot_product(&vec1, &vec2);
///
/// assert_eq!(res, 55.0);
/// ```
pub fn dot_product(lhs: &[f64], rhs: &[f64]) -> f64 {
    lhs.iter().zip(rhs).map(|(v1, v2)| v1 * v2).sum()
}

/// Matrix multiplication
///
/// You don't know how to multiply matrix?
/// Quite simple! See <https://www.mathsisfun.com/algebra/matrix-multiplying.html>
///
/// Assume rhs is transposed
/// - lhs: (m, n)
/// - rhs: (p, n)
/// - output: (m, p)
///
/// # Exmaple
///
/// ```
/// use cs220::assignments::assignment09::matmul;
///
/// let mat1 = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
/// let mat2 = vec![
///     vec![7.0, 8.0, 9.0],
///     vec![10.0, 11.0, 12.0],
///     vec![13.0, 14.0, 15.0],
///     vec![16.0, 17.0, 18.0],
/// ];
/// let ans = vec![
///     vec![50.0, 68.0, 86.0, 104.0],
///     vec![122.0, 167.0, 212.0, 257.0],
/// ];
/// let res = matmul(&mat1, &mat2);
/// assert_eq!(ans, res);
/// ```
pub fn matmul(lhs: &[Vec<f64>], rhs: &[Vec<f64>]) -> Vec<Vec<f64>> {
    lhs.iter()
        .map(|row| {
            rhs.iter()
                .map(|column| dot_product(row, column))
                .collect_vec()
        })
        .collect_vec()
}
