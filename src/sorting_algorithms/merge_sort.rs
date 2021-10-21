use std::vec::Vec;


/// # Merge
/// Deals with the merging and comparison aspect of `merge_sort`.
/// ### Parameters:
/// ```rust
/// where T: Ord + Copy
/// left: usize // Left Index.
/// middle: usize // Middle Index.
/// right: usize // Right Index.
/// vec: &mut Vec<T> // Vector to merge into.
/// ```
#[allow(dead_code)]
fn merge<T: Ord + Copy>(left: usize, middle: usize, right: usize, vec: &mut Vec<T>) {
    /* Based on: https://github.com/TheAlgorithms/Rust/blob/master/src/sorting/merge_sort.rs */

    /* Temporay vectors for each half in `vec` */
    let (mut left_vec, mut right_vec) = (Vec::new(), Vec::new());
    for v in vec.iter().take(middle + 1).skip(left) { left_vec.push(*v); }
    for v in vec.iter().take(right + 1).skip(middle + 1) { right_vec.push(*v); }

    let (left_len, right_len) = (left_vec.len(), right_vec.len());

    /* Track positions while merging */
    let (mut l, mut r, mut v) = (0, 0, left);

    /* Pick smaller elements one by one from both halves */
    while l < left_len && r < right_len {
        if left_vec[l] < right_vec[r] {
            vec[v] = left_vec[l];
            l += 1;
        }
        else {
            vec[v] = right_vec[r];
            r += 1
        }

        v += 1;
    }

    /* Finish up putting away all elements in left half */
    while l < left_len {
        vec[v] = left_vec[l];
        l += 1;
        v += 1;
    }

    /* Finish up putting away all elements in right half */
    while r < right_len {
        vec[v] = right_vec[r];
        r += 1;
        v += 1;
    }
}


/// # Merge Sort *LeftRight*
/// Similar to `merge_sort` but with control over starting and end position of the sort.
/// ### Parameters:
/// ```rust
/// where T: Ord + Copy
/// vec: &mut Vec<T> // Vector to sort.
/// left: usize // Starting index position.
/// right: usize // Ending index position.
/// ```
/// ### Complexities:
/// ```py
/// Worst Case Time Complexity == O(n log n)
/// Average Case Time Complexity == O(n log n)
/// Best Case Time Complexity == O(n log n)
/// Space Complexity == O(n)
/// ```
fn merge_sort_lr<T: Ord + Copy>(vec: &mut Vec<T>, left: usize, right: usize) {
    if left < right {
        let middle: usize = left + (right - left) / 2;
        merge_sort_lr(vec, left, middle);
        merge_sort_lr(vec, middle + 1, right);
        merge(left, middle, right, vec);
    }
}


/// # Merge Sort
/// ### Parameters:
/// ```rust
/// where T: Ord + Copy
/// vec: &mut Vec<T> // Vector to sort.
/// ```
/// ### Complexities:
/// ```py
/// Worst Case Time Complexity == O(n log n)
/// Average Case Time Complexity == O(n log n)
/// Best Case Time Complexity == O(n log n)
/// Space Complexity == O(n)
/// ```
#[allow(dead_code)]
pub fn merge_sort<T: Ord + Copy>(vec: &mut Vec<T>) {
    let vec_len: usize = vec.len();
    if vec_len > 1 { merge_sort_lr(vec, 0, vec_len - 1); }
}