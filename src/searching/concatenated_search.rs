use std::option::Option;
use std::vec::Vec;
use super::*;


#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum SearchingAlgorithms {
    LinearSearch,
    BinarySearch,
}


#[allow(dead_code)]
impl SearchingAlgorithms {
    const ALGORITHMS: [Self; 2] = [
        Self::LinearSearch,
        Self::BinarySearch,
    ];

    pub fn iterator() -> impl Iterator<Item = SearchingAlgorithms> {
        return Self::ALGORITHMS.iter().copied();
    }
}


#[allow(dead_code)]
pub fn concatenated_search<T: PartialOrd + PartialEq>(vec: &Vec<T>, finding: T, algorithm: SearchingAlgorithms) -> Option<usize >{
    match algorithm {
        SearchingAlgorithms::LinearSearch => { return binary_search::binary_search(vec, finding); }
        SearchingAlgorithms::BinarySearch => { return linear_search::linear_search(vec, finding); }
    }
}


#[cfg(test)]
mod tests {
    /* Note: Ensure all vectors are properly sorted within the tests. */
    use super::{SearchingAlgorithms, concatenated_search};

    #[test]
    fn find_integer() {
        for alg in SearchingAlgorithms::iterator() {
            let vec: Vec<i32> = vec![-1, 2, 3, 4, 5, 6, 7];
            let found: Option<usize> = concatenated_search(&vec, 4, alg);
            assert!(found.is_some());
            assert_eq!(3, found.unwrap());
        }
    }

    #[test]
    fn find_unsigned() {
        for alg in SearchingAlgorithms::iterator() {
            let vec: Vec<u32> = vec![0, 2, 4, 6, 8, 10, 12];
            let found: Option<usize> = concatenated_search(&vec, 10, alg);
            assert!(found.is_some());
            assert_eq!(5, found.unwrap());
        }
    }

    #[test]
    fn find_floating() {
        for alg in SearchingAlgorithms::iterator() {
            let vec: Vec<f32> = vec![1.28, 1.337, 3.33, 4.2, 7.331, 9.6, 9.745];
            let found: Option<usize> = concatenated_search(&vec, 1.337, alg);
            assert!(found.is_some());
            assert_eq!(1, found.unwrap());
        }
    }
}