use std::vec::Vec;


/// ## Complexities:
/// ```py
/// Worst Case Time Complexity ==  O(n * n)
/// Average Case Time Complexity ==  O(n * n)
/// Best Case Time Complexity ==  O(n)
/// Space Complexity ==  O(n) Total, O(1) Auxiliary
/// ```
#[allow(dead_code)]
pub fn bubble_sort<T: PartialOrd>(vec: &mut Vec<T>) {
    let vec_len: usize = vec.len();
    for i in 0 .. vec_len {
        let mut swapped: bool = false;
        for j in 0 .. vec_len - i - 1 {
            if vec[j] > vec[j + 1] {
                vec.swap(j, j + 1);
                swapped = true;
            }
        }
        if swapped == false { break; }
    }
}


#[cfg(test)]
mod tests {
    use super::bubble_sort;

    #[test]
    fn sort_integer_vector() {
        let mut vec: Vec<i32> = vec![0, 3, 1, 5, 6, 8, 7];
        bubble_sort(&mut vec);
        assert_eq!(vec, vec![0, 1, 3, 5, 6, 7, 8]);
    }

    #[test]
    fn sort_unsigned_vector() {
        let mut vec: Vec<u32> = vec![4, 3, 2, 6, 3, 1, 9];
        bubble_sort(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 3, 4, 6, 9]);
    }

    #[test]
    fn sort_floating_vector() {
        let mut vec: Vec<f32> = vec![0.5, 1.32, 1.11, 5.72, 4.20, 1.337, 8.04];
        bubble_sort(&mut vec);
        assert_eq!(vec, vec![0.5, 1.11, 1.32, 1.337, 4.20, 5.72, 8.04]);
    }
}