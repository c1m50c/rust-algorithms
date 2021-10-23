/* Based around: https://github.com/TheAlgorithms/Rust/blob/master/src/sorting/quick_sort.rs */
use std::vec::Vec;


/// # Partition
/// Handles the sorting aspect of `quick_sort` & `quick_sort_lr`.
/// ### Parameters:
/// ```rust
/// where T: Ord
/// vec: &mut Vec<T> // Vector to create partition from.
/// left: isize // Start position of portion to create partition from.
/// right: isize // End position of portion to create partition from.
/// ```
fn partition<T: Ord>(vec: &mut Vec<T>, left: isize, right: isize) -> isize {
    let pivot: usize = right as usize;
    let (mut i, mut j) = (left - 1, right);

    loop {
        i += 1;
        while vec[i as usize] < vec[pivot] { i += 1; }

        j -= 1;
        while j >= 0 && vec[j as usize] > vec[pivot] { j -= 1; }

        if i >= j { break; }
        else { vec.swap(i as usize, j as usize); }
    }
    
    vec.swap(i as usize, pivot as usize);
    return i;
}


/// # Quick Sort *LeftRight*
/// Similar to `quick_sort` with added control of the portion of the vector to sort.
/// ### Parameters:
/// ```rust
/// where T: Ord
/// vec: &mut Vec<T> // Vector to sort.
/// left: isize // Start of sorting portion.
/// right: isize // End of sorting portion.
/// ```
/// ### Complexities:
/// ```py
/// Worst Case Time Complexity == O(n^2)
/// Average Case Time Complexity == O(n log n)
/// Best Case Time Complexity == O(n log n)
/// Space Complexity == O(n)
/// ```
fn quick_sort_lr<T: Ord>(vec: &mut Vec<T>, left: isize, right: isize) {
    if left < right {
        let part: isize = partition(vec, left, right);
        quick_sort_lr(vec, left, part - 1);
        quick_sort_lr(vec, part + 1, right);
    }
}


/// # Quick Sort
/// ### Parameters:
/// ```rust
/// where T: Ord
/// vec: &mut Vec<T> // Vector to sort.
/// ```
/// ### Complexities:
/// ```py
/// Worst Case Time Complexity == O(n^2)
/// Average Case Time Complexity == O(n log n)
/// Best Case Time Complexity == O(n log n)
/// Space Complexity == O(n)
/// ```
pub fn quick_sort<T: Ord>(vec: &mut Vec<T>) {
    quick_sort_lr(vec, 0, (vec.len() - 1) as isize);
}