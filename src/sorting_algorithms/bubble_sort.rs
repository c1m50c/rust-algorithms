use std::vec::Vec;


/// # Bubble Sort
/// ### Parameters:
/// ```rust
/// where T: Ord
/// vec: &mut Vec<T> // Vector to sort.
/// ```
/// ### Complexities:
/// ```py
/// Worst Case Time Complexity == O(n^2)
/// Average Case Time Complexity == O(n^2)
/// Best Case Time Complexity == O(n)
/// Space Complexity == O(n) total, O(1) auxiliary
/// ```
#[allow(dead_code)]
pub fn bubble_sort<T: Ord>(vec: &mut Vec<T>) {
    for i in 0 .. vec.len() {
        let mut swapped: bool = false;
        for j in 0 .. vec.len() - i - 1 {
            if vec[j] > vec[j + 1] {
                vec.swap(j, j + 1);
                swapped = true;
            }
        }
        if swapped == false { break; }
    }
}