use super::helper_functions::create_integer_vector;
use std::time::{Duration, Instant};
use std::option::Option;
use std::vec::Vec;
use rand::Rng;


/// # Run Tests
/// Runs tests to ensure the specified searching algorithm can properly search, while also testing it for speed.
/// ### Parameters:
/// ```rust
/// func: &dyn Fn(&Vec<i32>, i32) -> Option<usize> // The searching algorithm's coresponding function.
/// ```
pub fn run_tests(func: &dyn Fn(&Vec<i32>, i32) -> Option<usize>) {
    /* Todo: Get actual function name, preferablly with a helper_function. */
    println!("<METHOD_NAME>\n");

    /* Speed Tests */
    speed_test(func, 1000);
    speed_test(func, 10000);
    speed_test(func, 100000);
    speed_test(func, 500000);
    speed_test(func, 1000000);
    speed_test(func, 5000000);
    speed_test(func, 10000000);
    speed_test(func, 50000000);

    println!("<METHOD_NAME/>");
}


/// # Speed Test
/// Tests a searching algorithm's speed by searching through a vector of size `length`.
/// ### Parameters:
/// ```rust
/// func: &dyn Fn(&Vec<i32>, i32) -> Option<usize>  // The searching algorithm's coresponding function.
/// length: i32 // Desired len() of testing Vector.
/// ```
pub fn speed_test(func: &dyn Fn(&Vec<i32>, i32) -> Option<usize>, length: i32) {
    println!("Search Speed Test");

    let vec: Vec<i32> = create_integer_vector(length, RAND_MIN, RAND_MAX);

    /* Todo: Make these based off length? */
    const RAND_MIN: i32 = -2500000;
    const RAND_MAX: i32 = 2500000;

    let begin_time: Instant = Instant::now();
    let finding: i32 = rand::thread_rng().gen_range(RAND_MIN .. RAND_MAX);
    let found: Option<usize> = func(&vec, finding);
    let end_time: Duration = begin_time.elapsed();

    match found {
        Some(idx) => println!("Searched for '{}', found at index '{}'.", finding, idx),
        None => println!("Searched for '{}', '{}' does not exist within given Vector.", finding, finding),
    }

    println!("Time Elapsed: {}ms\n", end_time.as_millis());
}