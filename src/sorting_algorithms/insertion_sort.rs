use std::vec::Vec;

/// # Insertion Sort
/// ### Parameters:
/// ```rust
/// vec: &mut Vec<i32> // Vector to sort
/// ```
/// ### Complexities:
/// ```py
/// Worst Case Time Complexity == O(n^2)
/// Average Case Time Complexity == O(n^2)
/// Best Case Time Complexity == O(n)
/// Space Complexity == O(n) total, O(1) auxiliary
/// ```
#[allow(dead_code)]
pub fn insertion_sort(vec: &mut Vec<i32>) {
    for i in 1 .. vec.len() {
        let mut j: usize = i;
        while j > 0 && vec[j] < vec[j - 1] {
            vec.swap(j - 1, j);
            j = j - 1;
        }
    }
}