/* Based around: https://github.com/TheAlgorithms/Rust/blob/master/src/sorting/quick_sort.rs */
use std::vec::Vec;


fn partition<T: Ord>(vec: &mut Vec<T>, left: isize, right: isize) -> isize {
    let pivot: usize = right as usize;
    let (mut i, mut j) = (left - 1, right);

    loop {
        i += 1;
        while vec[i as usize] < vec[pivot] { i += 1; }

        j -= 1;
        while j >= 0 && vec[j as usize] > vec[pivot] { j -= 1; }

        if i >= j { break; }
        else { vec.swap(i as usize, j as usize); }
    }
    
    vec.swap(i as usize, pivot as usize);
    return i;
}


fn quick_sort_lr<T: Ord>(vec: &mut Vec<T>, left: isize, right: isize) {
    if left < right {
        let part: isize = partition(vec, left, right);
        quick_sort_lr(vec, left, part - 1);
        quick_sort_lr(vec, part + 1, right);
    }
}


pub fn quick_sort<T: Ord>(vec: &mut Vec<T>) {
    let vec_len: usize = vec.len();
    quick_sort_lr(vec, 0, (vec_len - 1) as isize);
}