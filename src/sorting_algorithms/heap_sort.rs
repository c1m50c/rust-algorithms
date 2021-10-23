/* Based around: https://github.com/TheAlgorithms/Rust/blob/master/src/sorting/heap_sort.rs */
use std::vec::Vec;


fn move_down<T: Ord>(slice: &mut [T], mut root: usize) {
    let last: usize = slice.len() - 1;

    loop {
        let left: usize = 2 * root + 1;
        if left > last { break; }

        let right: usize = left + 1;
        let max: usize;

        if right <= last && slice[right] > slice[left] { max = right; }
        else { max = left; }

        if slice[max] > slice[root] { slice.swap(root, max); }
        root = max;
    }
}


fn create_heap<T: Ord>(vec: &mut Vec<T>) {
    let last_parent: usize  = (vec.len() - 2) / 2;
    for i in (0 ..=last_parent).rev() {
        move_down(vec.as_mut_slice(), i);
    }
}


pub fn heap_sort<T: Ord>(vec: &mut Vec<T>) {
    let vec_len: usize = vec.len();
    if vec_len <= 1 { return; }
    create_heap(vec);

    for end in (1 .. vec_len).rev() {
        vec.swap(0, end);
        move_down(&mut vec[..end], 0);
    }
}