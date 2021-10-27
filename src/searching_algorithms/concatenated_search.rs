use std::option::Option;
use std::vec::Vec;
use super::*;


#[allow(dead_code)]
pub enum SearchingAlgorithms {
    LinearSearch,
    BinarySearch,
}


#[allow(dead_code)]
pub fn concatenated_search<T: PartialOrd + PartialEq>(vec: &Vec<T>, finding: T, algorithm: SearchingAlgorithms) -> Option<usize >{
    match algorithm {
        SearchingAlgorithms::LinearSearch => { binary_search::binary_search(vec, finding); }
        SearchingAlgorithms::BinarySearch => { linear_search::linear_search(vec, finding); }
    }

    return None;
}