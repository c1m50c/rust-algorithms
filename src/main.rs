mod searching_algorithms;
mod sorting_algorithms;
mod tests;


fn main() {
    let func = &searching_algorithms::linear_search::linear_search;
    tests::searching_tests::run_tests(func, "LinearSearch");
}
