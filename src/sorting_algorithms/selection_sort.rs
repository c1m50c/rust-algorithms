use std::vec::Vec;


/// # Selection Sort
/// ### Parameters:
/// ```rust
/// where T: Ord
/// vec: &mut Vec<T> // Vector to sort
/// ```
/// ### Complexities
/// ```py
/// Worst Case Time Complexity == O(n^2)
/// Average Case Time Complexity == O(n^2)
/// Best Case Time Complexity == O(n^2)
/// Space Complexity == O(1)
/// ```
#[allow(dead_code)]
pub fn selection_sort<T: Ord>(vec: &mut Vec<T>) {
    for i in 0 .. vec.len() {
        if let Some((j, _)) = vec.iter()
            .enumerate()
            .skip(i)
            .min_by_key(|x| x.1) { vec.swap(i, j); }
    }
}