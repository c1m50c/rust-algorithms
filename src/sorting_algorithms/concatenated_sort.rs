use std::vec::Vec;
use super::*;


#[allow(dead_code)]
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