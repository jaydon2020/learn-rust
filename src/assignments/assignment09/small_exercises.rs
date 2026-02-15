//! Small exercises.

use std::{collections::HashMap, f64};

use itertools::Itertools;

/// Returns whether the given sequence is a fibonacci sequence starts from the given sequence's
/// first two terms.
///
/// Returns `true` if the length of sequence is less or equal than 2.
///
/// # Example
///
/// ```
/// use cs220::assignments::assignment09::small_exercises::*;
///
/// assert_eq!(is_fibonacci([1, 1, 2, 3, 5, 8, 13].into_iter()), true);
/// assert_eq!(is_fibonacci([1, 1, 2, 3, 5, 8, 14].into_iter()), false);
/// ```
pub fn is_fibonacci(inner: impl Iterator<Item = i64>) -> bool {
    let mut first = 0;
    let mut second = 0;
    for (i, x) in inner.enumerate() {
        if i == 0 {
            first = x;
        } else if i == 1 {
            second = x;
        } else {
            let next = first + second;
            if next != x {
                return false;
            }
            first = second;
            second = next
        }
    }
    true
}

/// Returns the sum of `f(v)` for all element `v` the given array.
///
/// # Example
///
/// ```
/// use cs220::assignments::assignment09::small_exercises::*;
///
/// assert_eq!(sigma([1, 2].into_iter(), |x| x + 2), 7);
/// assert_eq!(sigma([1, 2].into_iter(), |x| x * 4), 12);
/// ```
pub fn sigma<T, F: Fn(T) -> i64>(inner: impl Iterator<Item = T>, f: F) -> i64 {
    let mut sum = 0;
    for x in inner {
        sum += f(x);
    }
    sum
}

/// Alternate elements from three iterators until they have run out.
///
/// You can assume that the number of elements of three iterators are same.
///
/// # Example
///
/// ```
/// use cs220::assignments::assignment09::small_exercises::*;
///
/// assert_eq!(
///     interleave3([1, 2].into_iter(), [3, 4].into_iter(), [5, 6].into_iter()),
///     vec![1, 3, 5, 2, 4, 6]
/// );
/// ```
pub fn interleave3<T>(
    mut list1: impl Iterator<Item = T>,
    mut list2: impl Iterator<Item = T>,
    mut list3: impl Iterator<Item = T>,
) -> Vec<T> {
    let mut merge: Vec<T> = Vec::new();
    let mut err = 0;
    loop {
        if let Some(x) = list1.next() {
            merge.push(x);
        } else {
            err += 1;
        }
        if let Some(x) = list2.next() {
            merge.push(x);
        } else {
            err += 1;
        }
        if let Some(x) = list3.next() {
            merge.push(x);
        } else {
            err += 1;
        }

        if err >= 3 {
            break;
        }
    }
    merge
}

/// Alternate elements from array of n iterators until they have run out.
///
/// You can assume that the number of elements of iterators are same.
///
/// # Example
///
/// ```
/// use cs220::assignments::assignment09::small_exercises::*;
///
/// assert_eq!(
///     interleave_n(&mut [[1, 2].into_iter(), [3, 4].into_iter(), [5, 6].into_iter()]),
///     vec![1, 3, 5, 2, 4, 6]
/// );
/// ```
pub fn interleave_n<T, const N: usize>(
    mut iters: [impl Iterator<Item = T>; N],
) -> impl Iterator<Item = T> {
    let mut merge: Vec<T> = Vec::new();
    let mut err = 0;
    loop {
        for iter in iters.iter_mut() {
            if let Some(x) = iter.next() {
                merge.push(x);
            } else {
                err += 1;
            }
        }

        if err >= N {
            break;
        }
    }
    merge.into_iter()
}

/// Returns mean of k smallest value's mean.
///
/// # Example
///
/// ```
/// use cs220::assignments::assignment09::small_exercises::*;
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
    let mut sort = inner.sorted();
    let sum = sort.by_ref().take(k).sum::<i64>();
    sum as f64 / k as f64
}

/// Returns mean for each class.
///
/// # Example
///
/// ```
/// use cs220::assignments::assignment09::small_exercises::*;
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
    let mut process: HashMap<String, (i64, usize)> = HashMap::new();
    let mut result: HashMap<String, f64> = HashMap::new();
    for (courses, marks) in inner {
        let _unused = process
            .entry(courses)
            .and_modify(|x| {
                x.0 += marks;
                x.1 += 1;
            })
            .or_insert((marks, 1));
    }
    for tmp in process {
        let _unused = result.insert(tmp.0, tmp.1 .0 as f64 / tmp.1 .1 as f64);
    }
    result
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
/// use cs220::assignments::assignment09::small_exercises::*;
///
/// assert_eq!(sum_is_n(vec![vec![1, 2, 3], vec![2, 3]], 3), 1);
/// assert_eq!(sum_is_n(vec![vec![1, 2, 3], vec![2, 3]], 4), 2);
/// assert_eq!(sum_is_n(vec![vec![1, 2, 3], vec![2, 3]], 5), 2);
/// assert_eq!(sum_is_n(vec![vec![1, 2, 3], vec![2, 3]], 6), 1);
/// assert_eq!(sum_is_n(vec![vec![1, 2, 3], vec![2, 3]], 2), 0);
/// ```
pub fn sum_is_n(inner: Vec<Vec<i64>>, n: i64) -> usize {
    let mut sum_vec: Vec<i64> = Vec::new();
    for (i, vec) in inner.iter().enumerate() {
        if i == 0 {
            sum_vec = vec.to_vec()
        } else {
            let mut tmp: Vec<i64> = Vec::new();
            for item in &sum_vec {
                for jtem in vec {
                    tmp.push(item + jtem);
                }
            }
            sum_vec = tmp
        }
    }
    sum_vec.iter().filter(|&&x| x == n).count()
}

/// Returns a new vector that contains the item that appears `n` times in the input vector in
/// increasing order.
///
/// # Example
///
/// ```
/// use cs220::assignments::assignment09::small_exercises::*;
///
/// assert_eq!(find_count_n(vec![1, 2], 1), vec![1, 2]);
/// assert_eq!(find_count_n(vec![1, 3, 3], 1), vec![1]);
/// assert_eq!(find_count_n(vec![1, 3, 3], 2), vec![3]);
/// assert_eq!(find_count_n(vec![1, 2, 3, 4, 4], 1), vec![1, 2, 3]);
/// ```
pub fn find_count_n(inner: Vec<usize>, n: usize) -> Vec<usize> {
    let mut proc: HashMap<usize, usize> = HashMap::new();
    for i in inner {
        let _unused = proc.entry(i).and_modify(|count| *count += 1).or_insert(1);
    }
    let mut result = Vec::new();
    for hm in proc {
        if hm.1 == n {
            result.push(hm.0);
        }
    }
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
/// # Example
///
/// ```
/// use cs220::assignments::assignment09::small_exercises::*;
///
/// assert_eq!(position_median(vec![1, 3, 3, 6, 7, 8, 9]), Some(3));
/// assert_eq!(position_median(vec![1, 3, 3, 3]), Some(1));
/// ```
pub fn position_median<T: Ord + Clone>(inner: Vec<T>) -> Option<usize> {
    if inner.is_empty() {
        return None;
    }

    let n = inner.len();
    // Clone to sort and find the median value
    let mut sorted = inner.clone();
    sorted.sort();

    // For both odd and even, the required median is at 0-based index n/2
    // (since for even you want the upper median).
    let median_value = &sorted[n / 2];

    // Find the first position of that median value in the original vector
    inner.iter().position(|x| x == median_value)
}

/// Returns the sum of all elements in a two-dimensional array.
///
/// # Example
/// ```
/// use cs220::assignments::assignment09::small_exercises::*;
///
/// assert_eq!(
///     two_dimensional_sum([[1, 2, 3].into_iter(), [4, 5, 6].into_iter()].into_iter()),
///     21
/// );
/// ```
pub fn two_dimensional_sum(inner: impl Iterator<Item = impl Iterator<Item = i64>>) -> i64 {
    inner.flatten().sum::<i64>()
}

/// Returns whether the given string is palindrome or not.
///
/// A palindrome is a word, number, phrase, or other sequence of characters which reads the same
/// backward as forward.
///
/// We consider the empty string as a palindrome.
///
/// Consult <https://en.wikipedia.org/wiki/Palindrome>.
pub fn is_palindrome(s: String) -> bool {
    if s.len() <= 1 {
        true
    } else {
        let mut tmp = s.clone();
        while tmp.len() >= 2 {
            let a = tmp.remove(0);
            let b = tmp.pop().unwrap_or_default();
            if a != b {
                return false;
            }
        }
        true
    }
}
