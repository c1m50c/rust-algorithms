use core::cmp::{PartialEq, PartialOrd};
use core::option::Option;


/// Searchs through the `slice` to retrieve the index of `finding`.
/// 
/// # Example
/// ```rust
/// use rust_algorithms::search::binary_search;
/// 
/// let slice = &[1, 2, 3, 4, 5];
/// let found = binary_search(slice, 3).unwrap();
/// assert_eq!(found, 2);
/// ```
pub fn binary_search<T: PartialEq + PartialOrd>(slice: &[T], finding: T) -> Option<usize> {
    let (mut left, mut right) = (0, slice.len());
    
    while left < right {
        let middle = left + (right - left) / 2;

        if finding == slice[middle] {
            return Some(middle);
        } else if finding < slice[middle] {
            right = middle;
        } else {
            left = middle + 1;
        }
    }

    return None;
}