use std::option::Option;
use std::vec::Vec;


/// ## Complexities:
/// ```py
/// Worst Case Time Complexity == O(n)
/// Average Case Time Complexity == O(n / 2)
/// Best Case Time Complexity == O(1)
/// Space Complexity == O(1)
/// ```
#[allow(dead_code)]
pub fn linear_search<T: PartialEq>(vec: &Vec<T>, finding: T) -> Option<usize> {
    for index in 0 .. vec.len() {
        if vec[index] == finding {
            return Some(index);
        }
    }

    return None;
}


#[cfg(test)]
mod tests {
    use super::linear_search;

    #[test]
    fn find_integer() {
        let vec: Vec<i32> = vec![-1, 2, 3, 4, 5, 6, 7];
        let found: Option<usize> = linear_search(&vec, 4);
        assert!(found.is_some());
        assert_eq!(3, found.unwrap());
    }

    #[test]
    fn find_unsigned() {
        let vec: Vec<u32> = vec![0, 2, 4, 6, 8, 10, 12];
        let found: Option<usize> = linear_search(&vec, 10);
        assert!(found.is_some());
        assert_eq!(5, found.unwrap());
    }

    #[test]
    fn find_floating() {
        let vec: Vec<f32> = vec![1.28, 1.337, 3.33, 4.2, 7.331, 9.6, 9.745];
        let found: Option<usize> = linear_search(&vec, 1.337);
        assert!(found.is_some());
        assert_eq!(1, found.unwrap());
    }
}