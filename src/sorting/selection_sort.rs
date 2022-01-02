use std::vec::Vec;


/// ## Complexities:
/// ```py
/// Worst Case Time Complexity == O(n * n)
/// Average Case Time Complexity == O(n * n)
/// Best Case Time Complexity == O(n * n)
/// Space Complexity == O(1)
/// ```
#[allow(dead_code)]
pub fn selection_sort<T: PartialOrd>(vec: &mut Vec<T>) {
    let vec_len: usize = vec.len();

    for i in 0 .. vec_len {
        let mut current_minimum_index: usize = i;

        for j in i + 1 .. vec_len {
            if vec[j] < vec[current_minimum_index] {
                current_minimum_index = j;
            }
        }
        
        vec.swap(i, current_minimum_index);
    }
}


#[cfg(test)]
mod tests {
    use super::selection_sort;

    #[test]
    fn sort_integer_vector() {
        let mut vec: Vec<i32> = vec![0, 3, 1, 5, 6, 8, 7];
        selection_sort(&mut vec);
        assert_eq!(vec, vec![0, 1, 3, 5, 6, 7, 8]);
    }

    #[test]
    fn sort_unsigned_vector() {
        let mut vec: Vec<u32> = vec![4, 3, 2, 6, 3, 1, 9];
        selection_sort(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 3, 4, 6, 9]);
    }

    #[test]
    fn sort_floating_vector() {
        let mut vec: Vec<f32> = vec![0.5, 1.32, 1.11, 5.72, 4.20, 1.337, 8.04];
        selection_sort(&mut vec);
        assert_eq!(vec, vec![0.5, 1.11, 1.32, 1.337, 4.20, 5.72, 8.04]);
    }
}