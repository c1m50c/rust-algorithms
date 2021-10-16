use std::{ops::AddAssign, vec::Vec};


/// # Merge
/// Deals with the merging and comparison aspect of `merge_sort`.
/// ### Parameters:
/// ```rust
/// left: &[i32] // Left most slice.
/// right: &[i32] // Right most slice.
/// vec: &mut Vec<i32> // Vector to use for merging.
/// ```
#[allow(dead_code)]
fn merge<T: Ord + AddAssign + Copy>(left: &[T], right: &[T], vec: &mut Vec<T>) {
    let (mut i, mut j, mut m) = (0, 0, 0);

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            vec[m] = left[i];
            i += 1;
        } else {
            vec[m] = right[j];
            j += 1;
        }
        m += 1;
    }

    if i < left.len() { vec[m..].copy_from_slice(&left[i..]); }
    if j < right.len() { vec[m..].copy_from_slice(&right[j..]); }
}


/// # Merge Sort
/// ### Parameters:
/// ```rust
/// vec: &mut Vec<i32> // Vector to sort.
/// ```
/// ### Complexities:
/// ```py
/// Worst Case Time Complexity == O(n log n)
/// Average Case Time Complexity == O(n log n)
/// Best Case Time Complexity == O(n log n)
/// Space Complexity == O(n)
/// ```
#[allow(dead_code)]
pub fn merge_sort<T: Ord + AddAssign + Copy>(vec: &mut Vec<T>) {
    /* Adapted from https://www.hackertouch.com/merge-sort-in-rust.html */
    if vec.len() <= 1 { return; }

    merge_sort(&mut vec[0 .. vec.len() / 2].to_vec());
    merge_sort(&mut vec[vec.len() / 2 .. vec.len()].to_vec());

    let mut temp: Vec<T> = vec.clone();
    merge(&vec[0 .. vec.len() / 2], &vec[vec.len() / 2 .. vec.len()], &mut temp);
    vec.copy_from_slice(&temp);
}