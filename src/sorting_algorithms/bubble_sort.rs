use std::vec::Vec;


#[allow(dead_code)]
pub fn bubble_sort(vec: &mut Vec<i32>) {
    for i in 0 .. vec.len() {
        let mut swapped: bool = false;
        for j in 0 .. vec.len() - i - 1 {
            if vec[j] > vec[j + 1] {
                vec.swap(j, j + 1);
                swapped = true;
            }
        }
        if swapped == false { break; }
    }
}