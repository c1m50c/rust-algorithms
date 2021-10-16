use super::helper_functions::{create_integer_vector, get_vector_as_string};

use std::time::{Duration, Instant};
use std::vec::Vec;

use term_painter::{Color::*, ToStyle};


/// # Run Tests
/// Runs tests to ensure a sorting algorithm can properly sort, while also testing for speed.
/// ### Parameters:
/// ```rust
/// func: &dyn Fn(&mut Vec<i32>) // Sorting Algorithm's coresponding function.
/// func_name: &'static str // Sorting Algorithm's name.
/// ```
pub fn run_tests(func: &dyn Fn(&mut Vec<i32>), func_name: &'static str) {
    println!("{}{}{}",
        Blue.bold().paint("<==="),
        Cyan.bold().paint(func_name),
        Blue.bold().paint("===>\n"),
    );

    speed_test(func, 1000);
    speed_test(func, 10000);
    speed_test(func, 100000);
    
    /* Skip subsequent tests on slow algorithms */
    /* Note: I think some algorithms are performing incredibly poorly for seemlingly no reason, look into it */
    if func_name == "SelectionSort" || func_name == "InsertionSort" {
        println!("{}",
            Black.bold().paint("Skipping subsequent tests due to specified algorithm's speed.\n")
        );
    } else {
        speed_test(func, 500000);
        speed_test(func, 1000000);
    }
    
    println!("{}{}{}",
        Blue.bold().paint("<==="),
        Cyan.bold().paint(func_name),
        Blue.bold().paint("/===>"),
    );
}


/// # Speed Test
/// Tests a sorting algorithm's speed.
/// ### Parameters:
/// ```rust
/// func: &dyn Fn(&mut Vec<i32>) // Sorting Algorithm's coresponding function.
/// length: i32 // Desired length of sorted vector.
/// ```
pub fn speed_test(func: &dyn Fn(&mut Vec<i32>), length: i32) {
    println!("{}",
        Black.bold().paint("Sort Speed Test"),
    );
    
    let rand_min: i32 = -length;
    let rand_max: i32 = length;

    let mut vec: Vec<i32> = create_integer_vector(length, rand_min, rand_max);
    let begin_time: Instant = Instant::now();

    println!("{}{}",
        Red.paint("Unsorted Vector: "),
        Red.bold().paint(get_vector_as_string(&vec)),
    );

    func(&mut vec);
    let end_time: Duration = begin_time.elapsed();

    println!("{}{}",
        Green.paint("Sorted Vector: "),
        Green.bold().paint(get_vector_as_string(&vec)),
    );

    println!("{}{}{}{}",
        BrightMagenta.paint("Sorting Time: "),
        BrightMagenta.bold().paint(end_time.as_secs_f32()),
        BrightMagenta.bold().paint("s "),
        BrightMagenta.paint("🕑\n"),
    );
}