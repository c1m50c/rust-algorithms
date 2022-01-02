/* Based around: https://github.com/TheAlgorithms/Rust/blob/master/src/sorting/heap_sort.rs */
use std::vec::Vec;


/// Move the element at `root` down until `slice` is a max heap again.
#[allow(dead_code)]
fn move_down<T: PartialOrd>(slice: &mut [T], mut root: usize) {
    let last: usize = slice.len() - 1;

    loop {
        let left: usize = 2 * root + 1;
        if left > last {
            break;
        }

        let right: usize = left + 1;
        let max: usize;

        if right <= last && slice[right] > slice[left] {
            max = right;
        } else {
            max = left;
        }

        if slice[max] > slice[root] {
            slice.swap(root, max);
        }

        root = max;
    }
}


/// Creates a heap within the given vector.
#[allow(dead_code)]
fn create_heap<T: PartialOrd>(vec: &mut Vec<T>) {
    let last_parent: usize  = (vec.len() - 2) / 2;

    for i in (0 ..=last_parent).rev() {
        move_down(vec.as_mut_slice(), i);
    }
}


/// ## Complexities:
/// ```py
/// Worst Case Time Complexity == O(n log n)
/// Average Case Time Complexity == O(n log n)
/// Best Case Time Complexity == O(n)
/// Space Complexity == O(n) Total, O(1) Auxiliary
/// ```
#[allow(dead_code)]
pub fn heap_sort<T: PartialOrd>(vec: &mut Vec<T>) {
    let vec_len: usize = vec.len();

    if vec_len <= 1 {
        return;
    }

    create_heap(vec);

    for end in (1 .. vec_len).rev() {
        vec.swap(0, end);
        move_down(&mut vec[..end], 0);
    }
}


#[cfg(test)]
mod tests {
    use super::heap_sort;

    #[test]
    fn sort_integer_vector() {
        let mut vec: Vec<i32> = vec![0, 3, 1, 5, 6, 8, 7];
        heap_sort(&mut vec);
        assert_eq!(vec, vec![0, 1, 3, 5, 6, 7, 8]);
    }

    #[test]
    fn sort_unsigned_vector() {
        let mut vec: Vec<u32> = vec![4, 3, 2, 6, 3, 1, 9];
        heap_sort(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 3, 4, 6, 9]);
    }

    #[test]
    fn sort_floating_vector() {
        let mut vec: Vec<f32> = vec![0.5, 1.32, 1.11, 5.72, 4.20, 1.337, 8.04];
        heap_sort(&mut vec);
        assert_eq!(vec, vec![0.5, 1.11, 1.32, 1.337, 4.20, 5.72, 8.04]);
    }
}