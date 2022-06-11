macro_rules! impl_tests {
    ($name: ident, $function: expr) => {
        #[test]
        fn $name() {
            let slice = &[1, 2, 3, 4, 5];
            let found = $function(slice, 3).unwrap();
            assert_eq!(found, 2);
        }
    };
}


use super::linear_search as linear;
impl_tests!(linear_search, linear);

use super::binary_search as binary;
impl_tests!(binary_search, binary);