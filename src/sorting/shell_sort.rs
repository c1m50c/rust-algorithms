use std::vec::Vec;


/// ## Complexities:
/// ```py
/// # Time Complexities strongly depend on the gap sequence.
/// Worst Case Time Complexity == O(?)
/// Average Case Time Complexity == O(?)
/// Best Case Time Complexity == O(?)
/// Space Complexity == O(n) Total, O(1) Auxiliary
/// ```
#[allow(dead_code)]
pub fn shell_sort<T: PartialOrd + Copy>(vec: &mut Vec<T>) {
    const GAP_SEQUENCE: [usize; 8] = [ 701, 301, 132, 57, 23, 10, 4, 1 ];
    for gap in GAP_SEQUENCE {
        for i in gap .. vec.len() {
            let temp: T = vec[i];
            let mut j = i;

            while j >= gap && vec[j - gap] > temp {
                vec[j] = vec[j - gap];
                j -= gap;
            }

            vec[j] = temp;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::shell_sort;

    #[test]
    fn sort_integer_vector() {
        let mut vec: Vec<i32> = vec![0, 3, 1, 5, 6, 8, 7];
        shell_sort(&mut vec);
        assert_eq!(vec, vec![0, 1, 3, 5, 6, 7, 8]);
    }

    #[test]
    fn sort_unsigned_vector() {
        let mut vec: Vec<u32> = vec![4, 3, 2, 6, 3, 1, 9];
        shell_sort(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 3, 4, 6, 9]);
    }

    #[test]
    fn sort_floating_vector() {
        let mut vec: Vec<f32> = vec![0.5, 1.32, 1.11, 5.72, 4.20, 1.337, 8.04];
        shell_sort(&mut vec);
        assert_eq!(vec, vec![0.5, 1.11, 1.32, 1.337, 4.20, 5.72, 8.04]);
    }
}