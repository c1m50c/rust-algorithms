use std::time::{Duration, Instant};
use std::vec::Vec;
use rand::Rng;


/// # Create Integer Vector
/// Creats a vector filled with `i32` integers.
/// ### Parameters:
/// ```rust
/// length: i32 // Desired length of the returned vector.
/// rand_min: i32 // Minimum random value assigned to each integer.
/// rand_max: i32 // Maximum random value assigned to each integer.
/// ```
pub fn create_integer_vector(length: i32, rand_min: i32, rand_max: i32) -> Vec<i32> {
    /* Todo: Multi-threading? Or anything to speed up function time. */
    let mut vec: Vec<i32> = Vec::with_capacity(length as usize);

    println!("Creating Vector of length '{}'...", length);
    let begin_time: Instant = Instant::now();

    for _i in 0 .. length {
        let number: i32 = rand::thread_rng().gen_range(rand_min .. rand_max);
        vec.push(number);
    }

    let elapsed_time: Duration = begin_time.elapsed();
    println!("Created Vector of length '{}' in {}ms", length, elapsed_time.as_millis());

    return vec;
}