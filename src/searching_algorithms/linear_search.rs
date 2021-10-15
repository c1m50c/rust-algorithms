use std::option::Option;
use std::vec::Vec;


/// # Linear Search
/// Searches through `vec` to find `finding`, returning its index if found, or `None` if not.
/// ### Parameters:
/// ```rust
/// vec: &Vec<i32> // Vector to search through.
/// finding: i32 // What we're trying to find within the Vector.
/// ```
/// ### Complexities:
/// ```py
/// Worst Case Time Complexity == O(n)
/// Average Case Time Complexity == O(n / 2)
/// Best Case Time Complexity == O(1)
/// Space Complexity == O(1)
/// ```
#[allow(dead_code)]
pub fn linear_search(vec: &Vec<i32>, finding: i32) -> Option<usize> {
    for index in 0 .. vec.len() {
        if vec[index] == finding {
            return Some(index);
        }
    }

    return None;
}