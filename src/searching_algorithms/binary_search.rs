use std::result::Result;
use std::vec::Vec;

/// # Binary Search
/// Searches a sorted vector for `finding`, returning its index if found or an error if not.
/// ### Parameters:
/// ```rust
/// vec: &Vec<i32> // Sorted vector to search through `finding`.
/// finding: i32 // What we're trying to find within `vec`.
/// ```
/// ### Complexities:
/// ```py
/// Worst Case Time Complexity == O(log n)
/// Average Case Time Complexity == O(log n)
/// Best Case Time Complexity == O(1)
/// Space Complexity == O(1)
/// ```
pub fn binary_search(vec: &Vec<i32>, finding: i32) -> Result<usize, &'static str> {
    let mut left: usize = 0;
    let mut right: usize = vec.len() - 1;

    while left <= right {
        let middle: usize = (left + right) / 2;
        if finding == vec[middle] { return Ok(middle); }
        else if finding < vec[middle] { right = middle - 1; }
        else { left = middle + 1; }
    }

    return Err("Could not find `finding`.");
}