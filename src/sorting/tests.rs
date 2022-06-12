macro_rules! impl_tests {
    ($name: ident, $function: expr) => {
        #[test]
        fn $name() {
            let slice = &mut [3, 2, 14, 4, 1, 0, 9, 2, 1, 4, 5];
            $function(slice);
            
            for i in 1 .. slice.len() {
                assert!(slice[i] >= slice[i - 1]);
            }
        }
    };
}


use super::insertion_sort as insertion;
impl_tests!(insertion_sort, insertion);