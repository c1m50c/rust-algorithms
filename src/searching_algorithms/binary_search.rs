use std::option::Option;
use std::vec::Vec;


/// ## Complexities
/// ```py
/// Worst Case Time Complexity == O(log n)
/// Average Case Time Complexity == O(log n)
/// Best Case Time Complexity == O(1)
/// Space Complexity == O(1)
/// ```
#[allow(dead_code)]
pub fn binary_search<T: PartialOrd + PartialEq>(vec: &Vec<T>, finding: T) -> Option<usize> {
    let (mut left, mut right) = (0, vec.len());

    while left < right {
        let middle: usize = left + (right - left) / 2;
        if finding == vec[middle] { return Some(middle); }
        else if finding < vec[middle] { right = middle; }
        else { left = middle + 1; }
    }

    return None;
}


#[cfg(test)]
mod tests {
    /* Note: Ensure all vectors are properly sorted within the tests. */
    use super::binary_search;

    #[test]
    fn find_integer() {
        let vec: Vec<i32> = vec![-1, 2, 3, 4, 5, 6, 7];
        let found: Option<usize> = binary_search(&vec, 4);
        assert!(found.is_some());
        assert_eq!(3, found.unwrap());
    }

    #[test]
    fn find_unsigned() {
        let vec: Vec<u32> = vec![0, 2, 4, 6, 8, 10, 12];
        let found: Option<usize> = binary_search(&vec, 10);
        assert!(found.is_some());
        assert_eq!(5, found.unwrap());
    }

    #[test]
    fn find_floating() {
        let vec: Vec<f32> = vec![1.28, 1.337, 3.33, 4.2, 7.331, 9.6, 9.745];
        let found: Option<usize> = binary_search(&vec, 1.337);
        assert!(found.is_some());
        assert_eq!(1, found.unwrap());
    }
}