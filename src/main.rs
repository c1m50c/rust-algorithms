mod searching_algorithms;
mod sorting_algorithms;
mod tests;


use searching_algorithms::linear_search::linear_search;


fn main() {
    tests::searching_tests::run_tests(&linear_search as &dyn Fn(&Vec<i32>, i32) -> Option<usize>, "LinearSearch");
}