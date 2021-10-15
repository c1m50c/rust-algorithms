use super::helper_functions::create_integer_vector;
use std::time::{Duration, Instant};
use std::result::Result;
use std::vec::Vec;
use rand::Rng;


/// # Run Tests
/// Runs tests to ensure the specified searching algorithm can properly search, while also testing it for speed.
/// ### Parameters:
/// ```rust
/// func: &dyn Fn(&Vec<i32>, i32) -> Result<usize, &'static str> // The searching algorithm's coresponding function.
/// ```
pub fn run_tests(func: &dyn Fn(&Vec<i32>, i32) -> Result<usize, &'static str>) {
    println!("<METHOD_NAME>");

    speed_test(func, 1000);

    println!("<METHOD_NAME/>");
}


/// # Speed Test
/// Tests a searching algorithm's speed by searching through a vector of size `length`.
/// ### Parameters:
/// ```rust
/// func: &dyn Fn(&Vec<i32>, i32) -> Result<usize, &'static str> // The searching algorithm's coresponding function.
/// length: i32 // Desired len() of testing Vector.
/// ```
pub fn speed_test(func: &dyn Fn(&Vec<i32>, i32) -> Result<usize, &'static str>, length: i32) {
    println!("Search Speed Test");

    let vec: Vec<i32> = create_integer_vector(length, RAND_MIN, RAND_MAX);
    const RAND_MIN: i32 = -2500000;
    const RAND_MAX: i32 = 2500000;

    println!("Vector Length: {}", vec.len());
    let begin_time: Instant = Instant::now();
    let finding: i32 = rand::thread_rng().gen_range(RAND_MIN .. RAND_MAX);
    let found: Result<usize, &'static str> = func(&vec, finding);
    let end_time: Duration = begin_time.elapsed();
    println!("Searched for '{}', found at index '{}'.", finding, found.unwrap());
    println!("Time Elapsed: {}ms", end_time.as_millis());
}