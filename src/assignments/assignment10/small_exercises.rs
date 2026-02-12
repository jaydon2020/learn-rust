//! Small exercises.

use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashSet},
};

use itertools::*;

/// Returns the pairs of `(i, j)` where `i < j` and `inner[i] > inner[j]` in increasing order.
///
/// For example, the inversions of `[3, 5, 1, 2, 4]` is `[(0, 2), (0, 3), (1, 2), (1, 3), (1, 4)]`
/// because as follows:
///
/// - `0 < 2`, `inner[0] = 3 > 1 = inner[2]`
/// - `0 < 3`, `inner[0] = 3 > 2 = inner[3]`
/// - `1 < 2`, `inner[1] = 5 > 1 = inner[2]`
/// - `1 < 3`, `inner[1] = 5 > 2 = inner[3]`
/// - `1 < 4`, `inner[1] = 5 > 4 = inner[4]`
///
/// Consult <https://en.wikipedia.org/wiki/Inversion_(discrete_mathematics)> for more details of inversion.
pub fn inversion<T: Ord>(inner: Vec<T>) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();
    for (i, val_i) in inner.iter().enumerate() {
        for (j, val_j) in inner.iter().enumerate().skip(i) {
            if val_i > val_j {
                result.push((i, j));
            }
        }
    }
    result
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
    let mut result: Vec<T> = Vec::new();
    match root {
        Node::NonLeaf((t, vec)) => {
            result.push(t);
            for v in vec {
                let ret = traverse_preorder(v);
                result.extend(ret);
            }
        }
        Node::Leaf(t) => {
            result.push(t);
        }
    }
    result
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
/// Output: `[("a1", 1), ("a2", 3), ("b1", 3), ("a", 4), ("c", 8), ("b2", 15), ("b", 18), ("root",
/// 30)]`
pub fn du_sort(root: &File) -> Vec<(&str, usize)> {
    let mut result: Vec<(&str, usize)> = Vec::new();
    match root {
        File::Directory(dir_name, files) => {
            let mut size: usize = 0;
            for fl in files {
                let ret = du_sort(fl);

                // Fix 1: Only add the size of the specific child, not the sum of its whole subtree
                let child_size = match fl {
                    File::Data(_, s) => *s,
                    File::Directory(name, _) => {
                        // Find the directory entry in the returned vector to get its total size
                        ret.iter()
                            .find(|(n, _)| n == name)
                            .map(|(_, s)| *s)
                            .unwrap_or(0)
                    }
                };

                size += child_size;
                result.extend(ret);
            }
            result.push((dir_name, size));
        }
        File::Data(data_name, size) => {
            result.push((data_name, *size));
        }
    }

    // Fix 2: Sort by size, then by name if sizes are equal
    result.sort_by(
        |(a_name, a_size), (b_name, b_size)| match a_size.cmp(b_size) {
            Ordering::Equal => a_name.cmp(b_name),
            other => other,
        },
    );

    result
}

/// Remove all even numbers inside a vector using the given mutable reference.
/// That is, you must modify the vector using the given mutable reference instead
/// of returning a new vector.
///
/// # Example
/// ```ignore
/// let mut vec = vec![1, 2, 3, 4, 5];
/// remove_even(&mut vec);
/// assert_eq!(*vec, vec![1, 3, 5]);
/// ```
#[allow(clippy::ptr_arg)]
pub fn remove_even(inner: &mut Vec<i64>) {
    inner.retain(|x| x % 2 != 0);
}

/// Remove all duplicate occurences of a number inside the array.
/// That is, if an integer appears more than once, remove some occurences
/// of it so that it only appears once. Note that you must modify the vector
/// using the given mutable reference instead of returning a new vector.
/// Also, note that the order does not matter.
///
/// # Example
/// ```ignore
/// let mut vec = vec![1, 2, 1, 1, 3, 7, 5, 7];
/// remove_duplicate(&mut vec);
/// assert_eq!(*vec, vec![1, 2, 3, 7, 5]);
/// ```
#[allow(clippy::ptr_arg)]
pub fn remove_duplicate(inner: &mut Vec<i64>) {
    inner.dedup();
}

/// Returns the natural join of two tables using the first column as the join argument.
/// That is, for each pair of a row(`Vec<String>`) from table1 and a row(`Vec<String>`) from table2,
/// if the first element of them are equal, then add all elements of the row from table2
/// except its first element to the row from table1 and add it to the results.
/// Note that the order of results does not matter.
///
/// # Example
///
/// ```text
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
/// ```
pub fn natural_join(table1: Vec<Vec<String>>, table2: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut result = Vec::new();
    for row2 in &table2 {
        let [id2, course]: &[String; 2] = row2
            .as_slice()
            .try_into()
            .expect("each row must have 2 columns");
        for row1 in &table1 {
            let [id1, name]: &[String; 2] = row1
                .as_slice()
                .try_into()
                .expect("each row must have 2 columns");
            if id1 == id2 {
                let tmp: Vec<String> = [id2, name, course].iter().map(|s| s.to_string()).collect();
                result.push(tmp);
                break;
            }
        }
    }
    result
}

/// Helper struct to store state in the Priority Queue.
/// We store `real_a` to handle tie-breaking for equal `c`.
#[derive(Debug, Eq, PartialEq)]
struct TripleState {
    c: u64,
    real_a: u64, // The smaller leg (min(a, b))
    m: u64,
    n: u64,
}

impl Ord for TripleState {
    fn cmp(&self, other: &Self) -> Ordering {
        // Min-Heap logic:
        // 1. Prefer smaller 'c'.
        // 2. If 'c' is equal, prefer smaller 'real_a'.
        other
            .c
            .cmp(&self.c)
            .then_with(|| other.real_a.cmp(&self.real_a))
    }
}

impl PartialOrd for TripleState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Helper to create a state safely handling overflows and calculating 'real_a'.
fn build_state(m: u64, n: u64) -> Option<TripleState> {
    let m2 = m.checked_mul(m)?;
    let n2 = n.checked_mul(n)?;
    let c = m2.checked_add(n2)?;

    // a = m^2 - n^2, b = 2mn
    // Since m > n, m^2 > n^2.
    let leg1 = m2 - n2;
    // We know 2mn <= m^2 + n^2 (c), so it fits in u64 if c does.
    let leg2 = 2 * m * n;

    let real_a = std::cmp::min(leg1, leg2);

    Some(TripleState { c, real_a, m, n })
}

#[derive(Debug)]
/// Feel free
pub struct Pythagorean {
    queue: BinaryHeap<TripleState>,
}

impl Pythagorean {
    fn new() -> Self {
        let mut queue = BinaryHeap::new();
        // Initialize with the smallest primitive triple inputs: m=2, n=1.
        if let Some(state) = build_state(2, 1) {
            queue.push(state);
        }
        Pythagorean { queue }
    }
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

impl Iterator for Pythagorean {
    type Item = (u64, u64, u64);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let state = self.queue.pop()?;
            let m = state.m;
            let n = state.n;
            let c = state.c;
            let a = state.real_a;

            // --- 1. Queue Management (Generate Successors) ---

            // Successor A: Increment m by 2 to maintain parity with n.
            if let Some(next_state) = build_state(m + 2, n) {
                self.queue.push(next_state);
            }

            // Successor B: Start a new stream for n + 1.
            // We only do this once per 'n' stream, specifically when we are at the
            // first valid m for the current n (which is m = n + 1).
            if m == n + 1 {
                let next_n = n + 1;
                // m must start at n + 2 to maintain m > n and opposite parity
                if let Some(next_stream_start) = build_state(next_n + 1, next_n) {
                    self.queue.push(next_stream_start);
                }
            }

            // --- 2. Validation ---

            // Euclid's formula with coprime m, n (and opposite parity) generates primitive triples.
            // Our generation logic guarantees opposite parity, so we only check coprimality.
            if gcd(m, n) == 1 {
                // Determine b based on a and c
                // c^2 = a^2 + b^2 => b = sqrt(c^2 - a^2)
                // But we already know the legs are state.real_a and the other one.
                // We just need to return (a, b, c) where a < b.
                // state.real_a is already min(leg1, leg2), so it is 'a'.

                // We need to calculate 'b'.
                // Since c^2 = a^2 + b^2, and we have accurate integer arithmetic:
                // We can derive b from the legs calculated in build_state, but we didn't store the other leg.
                // Let's recalculate simply.
                let leg1 = m * m - n * n;
                let leg2 = 2 * m * n;
                let b = if leg1 == a { leg2 } else { leg1 };

                return Some((a, b, c));
            }
        }
    }
}

/// Generates sequence of unique [primitive Pythagorean triples](https://en.wikipedia.org/wiki/Pythagorean_triple),
/// i.e. (a,b,c) such that a² + b² = c², a and b are coprimes, and a < b. Generate in the increasing
/// order of c.
pub fn pythagorean() -> impl Iterator<Item = (u64, u64, u64)> {
    Pythagorean::new()
}
