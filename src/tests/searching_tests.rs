use super::helper_functions::create_integer_vector;

use std::time::{Duration, Instant};
use std::option::Option;
use std::vec::Vec;

use term_painter::{Color::*, ToStyle};
use rand::Rng;


/// # Run Tests
/// Runs tests to ensure the specified searching algorithm can properly search, while also testing it for speed.
/// ### Parameters:
/// ```rust
/// func: &dyn Fn(&Vec<i32>, i32) -> Option<usize> // The searching algorithm's coresponding function.
/// ```
pub fn run_tests(func: &dyn Fn(&Vec<i32>, i32) -> Option<usize>, func_name: &'static str) {
    /* Todo: Get actual function name, preferablly with a helper_function. */
    println!("<{}>\n", func_name);

    /* Speed Tests */
    speed_test(func, 1000);
    speed_test(func, 10000);
    speed_test(func, 100000);
    speed_test(func, 500000);
    speed_test(func, 1000000);
    speed_test(func, 5000000);
    speed_test(func, 10000000);
    speed_test(func, 50000000);

    println!("<{}/>", func_name);
}


/// # Speed Test
/// Tests a searching algorithm's speed by searching through a vector of size `length`.
/// ### Parameters:
/// ```rust
/// func: &dyn Fn(&Vec<i32>, i32) -> Option<usize>  // The searching algorithm's coresponding function.
/// length: i32 // Desired len() of testing Vector.
/// ```
pub fn speed_test(func: &dyn Fn(&Vec<i32>, i32) -> Option<usize>, length: i32) {
    println!("{}",
        Black.bold().paint("Search Speed Test"),
    );

    let rand_min: i32 = -length;
    let rand_max: i32 = length;
    
    let vec: Vec<i32> = create_integer_vector(length, rand_min, rand_max);

    let begin_time: Instant = Instant::now();
    let finding: i32 = rand::thread_rng().gen_range(rand_min .. rand_max);
    let found: Option<usize> = func(&vec, finding);
    let end_time: Duration = begin_time.elapsed();

    match found {
        Some(idx) => {
            println!("{}{}{}{}{}{}{}{}{}",
                Green.paint("Searched for "),
                Green.bold().paint("'"),
                Green.bold().paint(finding),
                Green.bold().paint("'"),
                Green.paint(", found at index "),
                Green.bold().paint("'"),
                Green.bold().paint(idx),
                Green.bold().paint("'"),
                Green.paint(" ✅")
            );
        }

        None => {
            println!("{}{}{}{}{}{}{}{}{}",
                Red.paint("Searched for "),
                Red.bold().paint("'"),
                Red.bold().paint(finding),
                Red.bold().paint("'"),
                Red.paint(", Could not find "),
                Red.bold().paint("'"),
                Red.bold().paint(finding),
                Red.bold().paint("'"),
                Red.paint(" anywhere within the given Vector ❌"),
            );
        }
    }

    println!("{}{}{}{}",
        BrightMagenta.paint("Search Time: "),
        BrightMagenta.bold().paint(end_time.as_secs_f32()),
        BrightMagenta.bold().paint("s "),
        BrightMagenta.paint("🕑\n"),
    );
}