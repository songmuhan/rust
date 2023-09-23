//! Small exercises.

use std::collections::{BinaryHeap, HashMap};

use itertools::Itertools;

/// Returns whether the given sequence is a fibonacci sequence starts from the given sequence's first two terms.
///
/// Returns `true` if the length of sequence is less or equal than 2.
///
/// # Exmample
///
/// ```
/// use cs220::assignments::assignment09::is_fibonacci;
///
/// assert_eq!(is_fibonacci([1, 1, 2, 3, 5, 8, 13].into_iter()), true);
/// assert_eq!(is_fibonacci([1, 1, 2, 3, 5, 8, 14].into_iter()), false);
/// ```
pub fn is_fibonacci(inner: impl Iterator<Item = i64>) -> bool {
    let v = inner.collect::<Vec<_>>();
    if v.len() <= 2 {
        return true;
    } else {
        for i in 2..v.len() {
            if v[i] != v[i - 1] + v[i - 2] {
                return false;
            }
        }
    }
    true
}

/// Returns the sum of `f(v)` for all element `v` the given array.
///
/// # Exmaple
///
/// ```
/// use cs220::assignments::assignment09::sigma;
///
/// assert_eq!(sigma([1, 2].into_iter(), |x| x + 2), 7);
/// assert_eq!(sigma([1, 2].into_iter(), |x| x * 4), 12);
/// ```
pub fn sigma<T, F: Fn(T) -> i64>(inner: impl Iterator<Item = T>, f: F) -> i64 {
    inner.map(f).sum::<i64>()
}
/// Alternate elements from three iterators until they have run out.
///
/// You can assume that the number of elements of three iterators are same.
///
/// # Example
///
/// ```
/// use cs220::assignments::assignment09::interleave3;
///
/// assert_eq!(
///     interleave3([1, 2].into_iter(), [3, 4].into_iter(), [5, 6].into_iter()),
///     vec![1, 3, 5, 2, 4, 6]
/// );
/// ```
pub fn interleave3<T>(
    list1: impl Iterator<Item = T>,
    list2: impl Iterator<Item = T>,
    list3: impl Iterator<Item = T>,
) -> Vec<T> {
    list1
        .zip(list2)
        .zip(list3)
        .flat_map(|((x, y), z)| vec![x, y, z])
        .collect()
}

/// Alternate elements from array of n iterators until they have run out.
///
/// You can assume that the number of elements of iterators are same.
///
/// # Example
///
/// ```
/// use cs220::assignments::assignment09::interleave_n;
///
/// assert_eq!(
///     interleave_n(&mut [[1, 2].into_iter(), [3, 4].into_iter(), [5, 6].into_iter()]),
///     vec![1, 3, 5, 2, 4, 6]
/// );
/// ```
pub fn interleave_n<T, const N: usize>(
    mut iters: [impl Iterator<Item = T>; N],
) -> impl Iterator<Item = T> {
    std::iter::from_fn(move || {
        let mut result = Vec::with_capacity(N);
        for iter in iters.iter_mut() {
            if let Some(item) = iter.next() {
                result.push(item);
            }
        }
        if result.is_empty() {
            None
        } else {
            Some(result.into_iter())
        }
    })
    .flatten()
}
/// Returns mean of k smallest value's mean.
///
/// # Example
///
/// ```
/// use cs220::assignments::assignment09::k_smallest_mean;
///
/// assert_eq!(
///     k_smallest_mean(vec![1, 3, 2].into_iter(), 2),
///     ((1 + 2) as f64 / 2.0)
/// );
/// assert_eq!(
///     k_smallest_mean(vec![7, 5, 3, 6].into_iter(), 3),
///     ((3 + 5 + 6) as f64 / 3.0)
/// );
/// ```
pub fn k_smallest_mean(inner: impl Iterator<Item = i64>, k: usize) -> f64 {
    let mut numbers = inner.collect_vec();
    numbers.sort();

    let sum: i64 = numbers.iter().take(k).sum();
    sum as f64 / k as f64
}

/// Returns mean for each class.
///
/// # Exmaple
///
/// ```
/// use cs220::assignments::assignment09::calculate_mean;
///
/// assert_eq!(
///     calculate_mean(
///         [
///             ("CS100".to_string(), 60),
///             ("CS200".to_string(), 60),
///             ("CS200".to_string(), 80),
///             ("CS300".to_string(), 100),
///         ]
///         .into_iter()
///     ),
///     [
///         ("CS100".to_string(), 60.0),
///         ("CS200".to_string(), 70.0),
///         ("CS300".to_string(), 100.0)
///     ]
///     .into_iter()
///     .collect()
/// );
/// ```
pub fn calculate_mean(inner: impl Iterator<Item = (String, i64)>) -> HashMap<String, f64> {
    let mut map = HashMap::new();

    for (key, value) in inner {
        _ = map
            .entry(key)
            .and_modify(|(origin, count)| {
                *origin += value;
                *count += 1;
            })
            .or_insert((value, 1));
    }

    map.into_iter()
        .map(|(key, (sum, count))| (key, sum as f64 / count as f64))
        .collect::<HashMap<String, f64>>()
}

/// Among the cartesian product of input vectors, return the number of sets whose sum equals `n`.
///
/// # Example
///
/// The cartesian product of [1, 2, 3] and [2, 3] are:
///     [1, 2], [1, 3], [2, 2], [2, 3], [3, 2], [3, 3].
///
/// Among these sets, the number of sets whose sum is 4 is 2, which is [1, 3] and [2, 2].
///
/// ```
/// use cs220::assignments::assignment09::sum_is_n;
///
/// assert_eq!(sum_is_n(vec![vec![1, 2, 3], vec![2, 3]], 3), 1);
/// assert_eq!(sum_is_n(vec![vec![1, 2, 3], vec![2, 3]], 4), 2);
/// assert_eq!(sum_is_n(vec![vec![1, 2, 3], vec![2, 3]], 5), 2);
/// assert_eq!(sum_is_n(vec![vec![1, 2, 3], vec![2, 3]], 6), 1);
/// assert_eq!(sum_is_n(vec![vec![1, 2, 3], vec![2, 3]], 2), 0);
/// ```
pub fn sum_is_n(inner: Vec<Vec<i64>>, n: i64) -> usize {
    if inner.is_empty() {
        return 0;
    }
    // Use a helper function for the recursive approach.
    fn helper(remaining: &[Vec<i64>], target: i64) -> usize {
        // If no vectors are left, check if we've reached the target sum.
        if remaining.is_empty() {
            return if target == 0 { 1 } else { 0 };
        }

        let (first, rest) = remaining.split_at(1);
        let mut count = 0;

        for &value in &first[0] {
            count += helper(rest, target - value);
        }

        count
    }

    helper(&inner, n)
}

/// Returns a new vector that contains the item that appears `n` times in the input vector in increasing order.
///
/// # Example
///
/// ```
/// use cs220::assignments::assignment09::find_count_n;
///
/// assert_eq!(find_count_n(vec![1, 2], 1), vec![1, 2]);
/// assert_eq!(find_count_n(vec![1, 3, 3], 1), vec![1]);
/// assert_eq!(find_count_n(vec![1, 3, 3], 2), vec![3]);
/// assert_eq!(find_count_n(vec![1, 2, 3, 4, 4], 1), vec![1, 2, 3]);
/// ```
pub fn find_count_n(inner: Vec<usize>, n: usize) -> Vec<usize> {
    let mut count_map = HashMap::new();

    // Populate the count map.
    for &num in &inner {
        *count_map.entry(num).or_insert(0) += 1;
    }

    // Filter numbers that appear n times, collect into a vector and sort.
    let mut result: Vec<usize> = count_map
        .into_iter()
        .filter(|&(_, count)| count == n)
        .map(|(num, _)| num)
        .collect();
    result.sort();

    result
}

/// Return the position of the median element in the vector.
///
/// For a data set `x` of `n` elements, the median can be defined as follows:
///
/// - If `n` is odd, the median is `(n+1)/2`-th smallest element of `x`.
/// - If `n` is even, the median is `(n/2)+1`-th smallest element of `x`.
///
/// Please following these rules:
///
/// - If the list is empty, returns `None`.
/// - If several elements are equally median, the position of the first of them is returned.
///
/// # Exmaple
///
/// ```
/// use cs220::assignments::assignment09::position_median;
///
/// assert_eq!(position_median(vec![1, 3, 3, 6, 7, 8, 9]), Some(3));
/// assert_eq!(position_median(vec![1, 3, 3, 3]), Some(1));
/// ```
pub fn position_median<T: Ord>(inner: Vec<T>) -> Option<usize> {
    let n = inner.len();
    if n == 0 {
        return None;
    }

    // Get indices of sorted version of inner
    let mut indices: Vec<usize> = (0..n).collect();
    indices.sort_by(|&a, &b| inner[a].cmp(&inner[b]));

    // Determine the median's index in the sorted version
    let median_index_in_sorted = if n % 2 == 0 {
        indices[n / 2]
    } else {
        indices[(n + 1) / 2 - 1]
    };
    let value = &inner[median_index_in_sorted];
    let mut counter = 0;
    for (i, v) in inner.as_slice().iter().enumerate() {
        if *value == *v {
            return Some(i);
        }
    }
    None
}

/// Returns the sum of all elements in a two-dimensional array.
///
/// # Example
/// ```
/// assert_eq!(
///     two_dimensional_sum([[1, 2, 3].into_iter(), [4, 5, 6].into_iter()].into_iter()),
///     21
/// );
/// ```
pub fn two_dimensional_sum(inner: impl Iterator<Item = impl Iterator<Item = i64>>) -> i64 {
    inner.map(|item| item.sum::<i64>()).sum::<i64>()
}

/// Returns whether the given string is palindrome or not.
///
/// A palindrome is a word, number, phrase, or other sequence of characters which reads the same backward as forward.
/// We consider the empty string is palindrome.
///
/// Consult <https://en.wikipedia.org/wiki/Palindrome>.
pub fn is_palindrome(s: String) -> bool {
    s == s.chars().rev().collect::<String>()
}
