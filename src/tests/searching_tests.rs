use std::result::Result;
use std::vec::Vec;


/// # Run Tests
/// Runs tests to ensure the specified searching algorithm can properly search, while also testing it for speed.
/// ### Parameters:
/// ```rust
/// func: &dyn Fn(&Vec<i32>, i32) -> Result<usize, &'static str> // The searching algorithm's coresponding function.
/// ```
pub fn run_tests(func: &dyn Fn(&Vec<i32>, i32) -> Result<usize, &'static str>) {
    
}