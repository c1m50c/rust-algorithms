/* Based on: https://github.com/TheAlgorithms/Rust/blob/master/src/sorting/merge_sort.rs */
use std::vec::Vec;


/// Deals with merging and comparison aspect of the `merge_sort` algorithm.
#[allow(dead_code)]
fn merge<T: PartialOrd + Copy>(left: usize, middle: usize, right: usize, vec: &mut Vec<T>) {
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


/// ## Complexities:
/// ```py
/// Worst Case Time Complexity == O(n log n)
/// Average Case Time Complexity == O(n log n)
/// Best Case Time Complexity == O(n log n)
/// Space Complexity == O(n)
/// ```
#[allow(dead_code)]
pub fn merge_sort<T: PartialOrd + Copy>(vec: &mut Vec<T>, left: usize, right: usize) {
    if left < right {
        let middle: usize = left + (right - left) / 2;
        merge_sort(vec, left, middle);
        merge_sort(vec, middle + 1, right);
        merge(left, middle, right, vec);
    }
}


#[cfg(test)]
mod tests {
    use super::merge_sort;

    #[test]
    fn sort_integer_vector() {
        let mut vec: Vec<i32> = vec![0, 3, 1, 5, 6, 8, 7];
        merge_sort(&mut vec, 0, 6);
        assert_eq!(vec, vec![0, 1, 3, 5, 6, 7, 8]);
    }

    #[test]
    fn sort_unsigned_vector() {
        let mut vec: Vec<u32> = vec![4, 3, 2, 6, 3, 1, 9];
        merge_sort(&mut vec, 0, 6);
        assert_eq!(vec, vec![1, 2, 3, 3, 4, 6, 9]);
    }

    #[test]
    fn sort_floating_vector() {
        let mut vec: Vec<f32> = vec![0.5, 1.32, 1.11, 5.72, 4.20, 1.337, 8.04];
        merge_sort(&mut vec, 0, 6);
        assert_eq!(vec, vec![0.5, 1.11, 1.32, 1.337, 4.20, 5.72, 8.04]);
    }
}