use core::cmp::PartialEq;
use core::option::Option;


/// Searchs through the `slice` to retrieve the index of `finding`.
/// 
/// # Example
/// ```rust
/// use rust_algorithms::search::linear_search;
/// 
/// let slice = &[1, 2, 3, 4, 5];
/// let found = linear_search(slice, 3).unwrap();
/// assert_eq!(found, 2);
/// ```
pub fn linear_search<T: PartialEq>(slice: &[T], finding: T) -> Option<usize> {
    for (idx, ele) in slice.iter().enumerate() {
        if ele == &finding {
            return Some(idx);
        }
    }
    
    return None;
}