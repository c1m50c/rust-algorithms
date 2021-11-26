use std::vec::Vec;
use super::*;


#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum SortingAlgorithms {
    SelectionSort,
    InsertionSort,
    BubbleSort,
    GnomeSort,
    QuickSort,
    MergeSort,
    ShellSort,
    HeapSort,
    CombSort,
}


#[allow(dead_code)]
impl SortingAlgorithms {
    const ALGORITHMS: [Self; 9] = [
        Self::SelectionSort,
        Self::InsertionSort,
        Self::BubbleSort,
        Self::GnomeSort,
        Self::QuickSort,
        Self::MergeSort,
        Self::ShellSort,
        Self::HeapSort,
        Self::CombSort,
    ];

    pub fn iterator() -> impl Iterator<Item = SortingAlgorithms> {
        return Self::ALGORITHMS.iter().copied();
    }
}


#[allow(dead_code)]
pub fn concatenated_sort<T: PartialOrd + Copy>(vec: &mut Vec<T>, algorithm: SortingAlgorithms) {
    let high: usize = vec.len() - 1;
    match algorithm {
        SortingAlgorithms::SelectionSort => { selection_sort::selection_sort(vec); }
        SortingAlgorithms::InsertionSort => { insertion_sort::insertion_sort(vec); }
        SortingAlgorithms::BubbleSort => { bubble_sort::bubble_sort(vec); }
        SortingAlgorithms::GnomeSort => { gnome_sort::gnome_sort(vec); }
        SortingAlgorithms::QuickSort => { quick_sort::quick_sort(vec, 0, high as isize); }
        SortingAlgorithms::MergeSort => { merge_sort::merge_sort(vec, 0, high); }
        SortingAlgorithms::ShellSort => { shell_sort::shell_sort(vec); }
        SortingAlgorithms::HeapSort => { heap_sort::heap_sort(vec); }
        SortingAlgorithms::CombSort => { comb_sort::comb_sort(vec); }
    }
}


#[cfg(test)]
mod tests {
    use super::{SortingAlgorithms, concatenated_sort};

    #[test]
    pub fn sort_integer_vector() {
        for alg in SortingAlgorithms::iterator() {
            let mut vec: Vec<i32> = vec![0, 3, 1, 5, 6, 8, 7];
            concatenated_sort(&mut vec, alg);
            assert_eq!(vec, vec![0, 1, 3, 5, 6, 7, 8]);
        }
    }

    #[test]
    pub fn sort_unsigned_vector() {
        for alg in SortingAlgorithms::iterator() {
            let mut vec: Vec<u32> = vec![4, 3, 2, 6, 3, 1, 9];
            concatenated_sort(&mut vec, alg);
            assert_eq!(vec, vec![1, 2, 3, 3, 4, 6, 9]);
        }
    }

    #[test]
    pub fn sort_floating_vector() {
        for alg in SortingAlgorithms::iterator() {
            let mut vec: Vec<f32> = vec![0.5, 1.32, 1.11, 5.72, 4.20, 1.337, 8.04];
            concatenated_sort(&mut vec, alg);
            assert_eq!(vec, vec![0.5, 1.11, 1.32, 1.337, 4.20, 5.72, 8.04]);
        }
    }
}