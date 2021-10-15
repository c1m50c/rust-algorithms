use std::vec::Vec;


pub fn insertion_sort(vec: &mut Vec<i32>) {
    for i in 1 .. vec.len() {
        let mut j: usize = i;
        while j > 0 && vec[j] < vec[j - 1] {
            vec.swap(j - 1, j);
            j = j - 1;
        }
    }
}