//! Small exercises.

use std::collections::{HashMap, HashSet};

use itertools::*;

/// Returns the pairs of `(i, j)` where `i < j` and `inner[i] > inner[j]` in increasing order.
///
/// For example, the inversions of `[3, 5, 1, 2, 4]` is `[(0, 2), (0, 3), (1, 2), (1, 3), (1, 4)]` because as follows:
///
/// - `0 < 2`, `inner[0] = 3 > 1 = inner[2]`
/// - `0 < 3`, `inner[0] = 3 > 2 = inner[3]`
/// - `1 < 2`, `inner[1] = 5 > 1 = inner[2]`
/// - `1 < 3`, `inner[1] = 5 > 2 = inner[3]`
/// - `1 < 4`, `inner[1] = 5 > 4 = inner[4]`
///
/// Consult <https://en.wikipedia.org/wiki/Inversion_(discrete_mathematics)> for more details of inversion.
pub fn inversion<T: Ord>(inner: Vec<T>) -> Vec<(usize, usize)> {
    inner
        .iter()
        .enumerate()
        .flat_map(|(i, x)| {
            inner
                .iter()
                .enumerate()
                .skip(i + 1)
                .filter(move |(j, y)| x > y)
                .map(move |(j, _)| (i, j))
        })
        .collect()
}

/// Represents a node of tree data structure.
///
/// Consult <https://en.wikipedia.org/wiki/Tree_(data_structure)> for more details on tree data structure.
#[derive(Debug)]
pub enum Node<T> {
    /// Non-leaf node
    ///
    /// It contains `(the name of node, list of child nodes)`.
    NonLeaf((T, Vec<Node<T>>)),
    /// Leaf node
    ///
    /// It contains the name of node.
    Leaf(T),
}

/// Traverses the tree in preorder.
///
/// The algorithm for preorder traversal is as follows:
///
/// 1. Visit the root.
/// 2. If the root is a leaf node, end the traverse.
/// 3. If the root is a non-leaf node, traverse each subtree from the child nodes.
///
/// For example, the result of preorder traversal for the following tree
///
/// ```text
///     1
///    /|\
///   2 3 4
///  /|  /|\
/// 5 6 7 8 9
/// ```
///
/// which can be represented as
///
/// ```ignore
/// Node::NonLeaf((
///     1,
///     vec![
///         Node::NonLeaf((2, vec![Node::Leaf(5), Node::Leaf(6)])),
///         Node::Leaf(3),
///         Node::NonLeaf((4, vec![Node::Leaf(7), Node::Leaf(8), Node::Leaf(9)])),
///     ]
/// ))
/// ```
///
/// is `1 -> 2 -> 5 -> 6 -> 3 -> 4 -> 7 -> 8 -> 9`.
pub fn traverse_preorder<T>(root: Node<T>) -> Vec<T> {
    match root {
        Node::Leaf(leaf) => vec![leaf],
        Node::NonLeaf((root, children)) => {
            let mut v = vec![root];
            for child in children {
                v.append(&mut traverse_preorder(child));
            }
            v
        }
    }
}

/// File
#[derive(Debug)]
pub enum File {
    /// Directory
    ///
    /// It contains `(name of directory, list of files under the directory)`
    ///
    /// The size of a directory is the sum of the sizes of its sub-files.
    Directory(String, Vec<File>),

    /// Data
    ///
    /// It contains `(name of data, size of data)`
    Data(String, usize),
}

/// Given a file, summarize all subfiles and sizes in ascending order of size.
///
/// - Its behaviour is the same as the `du | sort -h` command on Linux.
/// - If the file size is the same, sort it by name.
/// - Assume that there are no duplicate file names.
///
/// # Example
///
/// Input:
///
/// ```txt
/// root (Directory)
/// |
/// |__a (Directory)
/// |  |__a1 (Data, size: 1)
/// |  |__a2 (Data, size: 3)
/// |
/// |__b (Directory)
/// |  |__b1 (Data, size: 3)
/// |  |__b2 (Data, size: 15)
/// |
/// |__c (Data, size: 8)
/// ```
///
/// Output: `[("a1", 1), ("a2", 3), ("b1", 3), ("a", 4), ("c", 8), ("b2", 15), ("b", 18), ("root", 30)]`
pub fn du_sort(root: &File) -> Vec<(&str, usize)> {
    fn compute_size(root: &File) -> usize {
        match root {
            File::Data(data, size) => *size,
            File::Directory(dir, subfile) => subfile.iter().map(compute_size).sum(),
        }
    }
    let mut result = match root {
        File::Directory(dirname, subfiles) => {
            let mut result = vec![(dirname.as_str(), compute_size(root))];
            for file in subfiles {
                match file {
                    File::Directory(dir, files) => {
                        result.append(&mut du_sort(file));
                    }
                    File::Data(name, size) => {
                        result.push((name, *size));
                    }
                }
            }
            result
        }
        File::Data(name, size) => {
            vec![(name.as_str(), *size)]
        }
    };
    result.sort_by(|a, b| {
        if a.1 == b.1 {
            a.0.cmp(b.0)
        } else {
            a.1.cmp(&b.1)
        }
    });
    result
}

/// Remove all even numbers inside a vector using the given mutable reference.
/// That is, you must modify the vector using the given mutable reference instead
/// of returning a new vector.
///
/// # Example
/// ```
/// let mut vec = vec![1, 2, 3, 4, 5];
/// remove_even(&mut vec);
/// assert_eq!(*vec, vec![1, 3, 5]);
/// ```
#[allow(clippy::ptr_arg)]
pub fn remove_even(inner: &mut Vec<i64>) {
    inner.retain(|item| item % 2 != 0)
}

/// Remove all duplicate occurences of a number inside the array.
/// That is, if an integer appears more than once, remove some occurences
/// of it so that it only appears once. Note that you must modify the vector
/// using the given mutable reference instead of returning a new vector.
/// Also, note that the order does not matter.
///
/// # Example
/// ```
/// let mut vec = vec![1, 2, 1, 1, 3, 7, 5, 7];
/// remove_duplicate(&mut vec);
/// assert_eq!(*vec, vec![1, 2, 3, 7, 5]);
/// ```
#[allow(clippy::ptr_arg)]
pub fn remove_duplicate(inner: &mut Vec<i64>) {
    let mut seen = HashSet::new();
    for i in &mut *inner {
        _ = seen.insert(*i);
    }

    inner.retain(|&item| seen.contains(&item))
}

/// Returns the natural join of two tables using the first column as the join argument.
/// That is, for each pair of a row(`Vec<String>`) from table1 and a row(`Vec<String>`) from table2,
/// if the first element of them are equal, then add all elements of the row from table2
/// except its first element to the row from table1 and add it to the results.
/// Note that the order of results does not matter.
///
/// # Example
///
///        table1                     table2
/// ----------------------     ----------------------
///  20230001 |    Jack         20230001 |    CS
///  20231234 |    Mike         20230001 |    EE
///                             20231234 |    ME
///
///
///               result
/// -----------------------------------
///  20230001 |    Jack   |     CS
///  20230001 |    Jack   |     EE
///  20231234 |    Mike   |     ME
///
pub fn natural_join(table1: Vec<Vec<String>>, table2: Vec<Vec<String>>) -> Vec<Vec<String>> {
    table1
        .iter()
        .flat_map(|row1| {
            table2.iter().filter_map(|row2| {
                if row1[0] == row2[0] {
                    Some(row1.iter().chain(row2.iter().skip(1)).cloned().collect())
                } else {
                    None
                }
            })
        })
        .collect()
}

struct Pythagorean {
    triples: Vec<(u64, u64, u64)>,
    index: usize,
}

impl Pythagorean {
    fn new() -> Self {
        let mut triples = vec![];
        for m in 2..=50 {
            // 50 is an arbitrary limit; adjust as needed.
            for n in 1..m {
                if gcd(m, n) == 1 && (m - n) % 2 == 1 {
                    let a = m * m - n * n;
                    let b = 2 * m * n;
                    let c = m * m + n * n;
                    if a < b {
                        triples.push((a, b, c));
                    } else {
                        triples.push((b, a, c));
                    }
                }
            }
        }
        triples.sort_by(
            |&(a1, _, c1), &(a2, _, c2)| {
                if c1 == c2 {
                    a1.cmp(&a2)
                } else {
                    c1.cmp(&c2)
                }
            },
        );
        Pythagorean { triples, index: 0 }
    }
}

impl Iterator for Pythagorean {
    type Item = (u64, u64, u64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.triples.len() {
            self.index += 1;
            Some(self.triples[self.index - 1])
        } else {
            None
        }
    }
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let tmp = a % b;
        a = b;
        b = tmp;
    }
    a
}

/// Generates sequence of unique [primitive Pythagorean triples](https://en.wikipedia.org/wiki/Pythagorean_triple),
/// i.e. (a,b,c) such that a² + b² = c², a and b are coprimes, and a < b. Generate in the increasing order of c.
pub fn pythagorean() -> impl Iterator<Item = (u64, u64, u64)> {
    Pythagorean::new()
}
