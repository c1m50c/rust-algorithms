use std::mem::swap;
use std::vec::Vec;


pub fn selection_sort(vec: &mut Vec<i32>) {
    for i in 0 .. vec.len() - 1 {
        let mut cur_min_idx: usize = i;
        for j in i + 1 .. vec.len() {
            if vec[j] < vec[cur_min_idx] {
                cur_min_idx = j;
            }
        }
        /* Todo: Swap vec[i] w/vec[cur_min_idx] */
    }
}