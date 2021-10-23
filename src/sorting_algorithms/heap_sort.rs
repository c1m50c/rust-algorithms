/* Based around: https://github.com/TheAlgorithms/Rust/blob/master/src/sorting/heap_sort.rs */
use std::vec::Vec;


/// # Move Down
/// Move the element at `root` down until `slice` is a max heap again.
/// ### Parameters:
/// ```rust
/// where T: Ord
/// slice: &mut [T] // Slice to make a max heap again.
/// mut root: usize // Element root.
/// ```
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


/// # Create Heap
/// Creates a heap within the given vector.
/// ### Parameters:
/// ```rust
/// where T: Ord
/// vec: &mut Vec<T> // Vector to make a heap out of.
/// ```
fn create_heap<T: Ord>(vec: &mut Vec<T>) {
    let last_parent: usize  = (vec.len() - 2) / 2;
    for i in (0 ..=last_parent).rev() {
        move_down(vec.as_mut_slice(), i);
    }
}


/// # Heap Sort
/// ### Parameters:
/// ```rust
/// where T: Ord
/// vec: &mut Vec<T> // Vector to sort.
/// ```
/// ### Complexities:
/// ```py
/// Worst Case Time Complexity: O(n log n)
/// Average Case Time Complexity: O(n log n)
/// Best Case Time Complexity: O(n)
/// Space Complexity: O(n) total, O(1) auxiliary
/// ```
pub fn heap_sort<T: Ord>(vec: &mut Vec<T>) {
    let vec_len: usize = vec.len();
    if vec_len <= 1 { return; }
    create_heap(vec);

    for end in (1 .. vec_len).rev() {
        vec.swap(0, end);
        move_down(&mut vec[..end], 0);
    }
}