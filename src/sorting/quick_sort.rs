use std::vec::Vec;


/// Handles the sorting aspect of `quick_sort`.
#[allow(dead_code)]
fn partition<T: PartialOrd>(vec: &mut Vec<T>, left: isize, right: isize) -> isize {
    let (mut i, mut j) = (left - 1, right);
    let pivot: usize = right as usize;

    loop {
        i += 1;
        while vec[i as usize] < vec[pivot] {
            i += 1;
        }

        j -= 1;
        while j >= 0 && vec[j as usize] > vec[pivot] {
            j -= 1;
        }

        if i >= j {
            break;
        } else {
            vec.swap(i as usize, j as usize);
        }
    }

    vec.swap(i as usize, pivot);
    return i;
}


/// ## Complexities:
/// ```py
/// Worst Case Time Complexity == O(n * n)
/// Average Case Time Complexity == O(n log n)
/// Best Case Time Complexity == O(n log n)
/// Space Complexity == O(n)
/// ```
#[allow(dead_code)]
pub fn quick_sort<T: PartialOrd>(vec: &mut Vec<T>, left: isize, right: isize) {
    if left < right {
        let part: isize = partition(vec, left, right);
        quick_sort(vec, left, part - 1);
        quick_sort(vec, part + 1, right);
    }
}


#[cfg(test)]
mod tests {
    use super::quick_sort;

    #[test]
    fn sort_integer_vector() {
        let mut vec: Vec<i32> = vec![0, 3, 1, 5, 6, 8, 7];
        quick_sort(&mut vec, 0, 6);
        assert_eq!(vec, vec![0, 1, 3, 5, 6, 7, 8]);
    }

    #[test]
    fn sort_unsigned_vector() {
        let mut vec: Vec<u32> = vec![4, 3, 2, 6, 3, 1, 9];
        quick_sort(&mut vec, 0, 6);
        assert_eq!(vec, vec![1, 2, 3, 3, 4, 6, 9]);
    }

    #[test]
    fn sort_floating_vector() {
        let mut vec: Vec<f32> = vec![0.5, 1.32, 1.11, 5.72, 4.20, 1.337, 8.04];
        quick_sort(&mut vec, 0, 6);
        assert_eq!(vec, vec![0.5, 1.11, 1.32, 1.337, 4.20, 5.72, 8.04]);
    }
}