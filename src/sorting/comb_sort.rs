use std::vec::Vec;


/// ## Complexities:
/// ```py
/// Worst Case Time Complexity == O(n * n)
/// Average Case Time Complexity == O(n * n / 2^p) # p = number of increments
/// Best Case Time Complexity == O(n log n)
/// Space Complexity == O(1)
/// ```
#[allow(dead_code)]
pub fn comb_sort<T: PartialOrd>(vec: &mut Vec<T>) {
    const GAP_SHRINK_FACTOR: f64 = 1.3;
    let mut gap: f64 = vec.len() as f64;
    let mut sorted: bool = false;

    while sorted == false {
        gap = f64::from(gap / GAP_SHRINK_FACTOR).floor();

        if gap <= 1.0 {
            sorted = true;
            gap = 1.0;
        }

        let mut i = 0;
        while (i as f64 + gap) < vec.len() as f64 {
            if vec[i] > vec[i + gap as usize] {
                vec.swap(i, i + gap as usize);
                sorted = false;
            }

            i += 1;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::comb_sort;

    #[test]
    fn sort_integer_vector() {
        let mut vec: Vec<i32> = vec![0, 3, 1, 5, 6, 8, 7];
        comb_sort(&mut vec);
        assert_eq!(vec, vec![0, 1, 3, 5, 6, 7, 8]);
    }

    #[test]
    fn sort_unsigned_vector() {
        let mut vec: Vec<u32> = vec![4, 3, 2, 6, 3, 1, 9];
        comb_sort(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 3, 4, 6, 9]);
    }

    #[test]
    fn sort_floating_vector() {
        let mut vec: Vec<f32> = vec![0.5, 1.32, 1.11, 5.72, 4.20, 1.337, 8.04];
        comb_sort(&mut vec);
        assert_eq!(vec, vec![0.5, 1.11, 1.32, 1.337, 4.20, 5.72, 8.04]);
    }
}