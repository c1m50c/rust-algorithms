use core::cmp::PartialOrd;


/// Sorts the `slice` from least to greatest.
/// 
/// # Example
/// ```rust
/// use rust_algorithms::sorting::insertion_sort;
/// 
/// let slice = &mut [4, 1, 2, 4, 0, 9, 8];
/// insertion_sort(slice);
/// 
/// // Ensure the slice is properly sorted
/// for i in 1 .. slice.len() {
///     assert!(slice[i] >= slice[i - 1]);
/// }
/// ```
pub fn insertion_sort<T: PartialOrd>(slice: &mut [T]) {
    for i in 1 .. slice.len() {
        let mut j = i;

        while j > 0 && slice[j] < slice[j - 1] {
            slice.swap(j - 1, j);
            j -= 1;
        }
    }
}