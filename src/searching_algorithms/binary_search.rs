use std::vec::Vec;


/// # Binary Search
/// Searches a sorted vector for `finding`, returning its index if found or `None` if not.
/// ### Parameters:
/// ```rust
/// where T: Ord + Eq
/// vec: &Vec<T> // Sorted vector to search through `finding`.
/// finding: T // What we're trying to find within `vec`.
/// ```
/// ### Complexities:
/// ```py
/// Worst Case Time Complexity == O(log n)
/// Average Case Time Complexity == O(log n)
/// Best Case Time Complexity == O(1)
/// Space Complexity == O(1)
/// ```
#[allow(dead_code)]
pub fn binary_search<T: Ord + Eq>(vec: &Vec<T>, finding: T) -> Option<usize>{
    let mut left: usize = 0;
    let mut right: usize = vec.len() - 1;

    while left <= right {
        let middle: usize = (left + right) / 2;
        if finding == vec[middle] { return Some(middle); }
        else if finding < vec[middle] { right = middle - 1; }
        else { left = middle + 1; }
    }

    return None;
}