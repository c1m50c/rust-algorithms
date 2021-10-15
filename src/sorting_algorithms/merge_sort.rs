use std::vec::Vec;


fn merge(left: &[i32], right: &[i32], vec: &mut Vec<i32>) {
    let (mut i, mut j, mut m) = (0, 0, 0);

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            vec[m] = left[i];
            i += 1;
        } else {
            vec[m] = right[j];
            j += 1;
        }
        m += 1;
    }

    if i < left.len() { vec[m..].copy_from_slice(&left[i..]); }
    if j < right.len() { vec[m..].copy_from_slice(&right[j..]); }
}


pub fn merge_sort(vec: &mut Vec<i32>) {
    /* Adapted from https://www.hackertouch.com/merge-sort-in-rust.html */
    if vec.len() <= 1 { return; }

    merge_sort(&mut vec[0 .. vec.len() / 2].to_vec());
    merge_sort(&mut vec[vec.len() / 2 .. vec.len()].to_vec());

    let mut temp: Vec<i32> = vec.clone();
    merge(&vec[0 .. vec.len() / 2], &vec[vec.len() / 2 .. vec.len()], &mut temp);
    vec.copy_from_slice(&temp);
}