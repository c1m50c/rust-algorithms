use std::time::{Duration, Instant};
use std::vec::Vec;

use term_painter::{Color::*, ToStyle};
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

    println!("{}{}{}{}{}",
        Yellow.paint("Creating Vector of length "),
        Yellow.bold().paint("'"),
        Yellow.bold().paint(length),
        Yellow.bold().paint("'"),
        Yellow.paint(" ⌛"),
    );
    let begin_time: Instant = Instant::now();

    for _i in 0 .. length {
        let number: i32 = rand::thread_rng().gen_range(rand_min .. rand_max);
        vec.push(number);
    }

    let elapsed_time: Duration = begin_time.elapsed();
    println!("{}{}{}{}{}{}{}{}{}",
        Green.paint("Created Vector of length "),
        Green.bold().paint("'"),
        Green.bold().paint(length),
        Green.bold().paint("'"),
        Green.paint(" in "),
        Green.bold().paint("'"),
        Green.bold().paint(elapsed_time.as_secs_f32()),
        Green.bold().paint("s'"),
        Green.paint(" ✅"),
    );

    return vec;
}