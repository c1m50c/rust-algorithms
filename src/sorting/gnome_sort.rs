use std::vec::Vec;


/// ## Complexities:
/// ```py
/// Worst Case Time Complexity == O(n * n)
/// Average Case Time Complexity == O(n * n)
/// Best Case Time Complexity == O(n)
/// Space Complexity == O(1) Auxiliary
/// ```
#[allow(dead_code)]
pub fn gnome_sort<T: PartialOrd>(vec: &mut Vec<T>) {
    for p in 1 .. vec.len() {
        let mut position: usize = p;
        
        while position > 0 && vec[position - 1] > vec[position] {
            vec.swap(position - 1, position);
            position -= 1;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::gnome_sort;

    #[test]
    fn sort_integer_vector() {
        let mut vec: Vec<i32> = vec![0, 3, 1, 5, 6, 8, 7];
        gnome_sort(&mut vec);
        assert_eq!(vec, vec![0, 1, 3, 5, 6, 7, 8]);
    }

    #[test]
    fn sort_unsigned_vector() {
        let mut vec: Vec<u32> = vec![4, 3, 2, 6, 3, 1, 9];
        gnome_sort(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 3, 4, 6, 9]);
    }

    #[test]
    fn sort_floating_vector() {
        let mut vec: Vec<f32> = vec![0.5, 1.32, 1.11, 5.72, 4.20, 1.337, 8.04];
        gnome_sort(&mut vec);
        assert_eq!(vec, vec![0.5, 1.11, 1.32, 1.337, 4.20, 5.72, 8.04]);
    }
}