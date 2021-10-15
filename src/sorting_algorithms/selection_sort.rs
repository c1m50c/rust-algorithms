use std::vec::Vec;


/// # Selection Sort
/// ### Parameters:
/// ```rust
/// vec: &mut Vec<i32> // Vector to sort
/// ```
/// ### Complexities
/// ```py
/// Worst Case Time Complexity == O(n^2)
/// Average Case Time Complexity == O(n^2)
/// Best Case Time Complexity == O(n^2)
/// Space Complexity == O(1)
/// ```
#[allow(dead_code)]
pub fn selection_sort(vec: &mut Vec<i32>) {
    for i in 0 .. vec.len() - 1 {
        let mut cur_min_idx: usize = i;
        for j in i + 1 .. vec.len() {
            if vec[j] < vec[cur_min_idx] {
                cur_min_idx = j;
            }
        }
        vec.swap(i, cur_min_idx);
    }
}