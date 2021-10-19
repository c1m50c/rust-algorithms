use super::helper_functions::{create_integer_vector, get_vector_as_string};

use std::time::{Duration, Instant};
use std::ops::Range;
use std::vec::Vec;

use term_painter::{Color::*, ToStyle};
use rand::Rng;


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

    speed_test(func, 1_000);
    speed_test(func, 10_000);
    speed_test(func, 100_000);
    
    /* Skip subsequent tests on slow algorithms */
    /* Note: I think some algorithms are performing incredibly poorly for seemlingly no reason, look into it */
    if func_name == "SelectionSort" || func_name == "InsertionSort" || func_name == "BubbleSort" {
        println!("{}",
            Black.bold().paint("Skipping subsequent tests due to specified algorithm's speed.\n")
        );
    } else {
        speed_test(func, 50_0000);
        speed_test(func, 1_000_000);
    }

    average_time_test(func, 100_000);
    assertion_test(func);
    
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


pub fn assertion_test(func: &dyn Fn(&mut Vec<i32>)) {
    let (mut unsorted_1, sorted_1) = (vec![3, 4, 2, 1, 5], vec![1, 2, 3, 4, 5]);
    let (mut unsorted_2, sorted_2) = (vec![9, 3, 2, 1, 7], vec![1, 2, 3, 7, 9]);
    let (mut unsorted_3, sorted_3) = (vec![8, 7, 4, 3, 5], vec![3, 4, 5, 7, 8]);
    let (mut unsorted_4, sorted_4) = (vec![4, 6, 5, 6, 0], vec![0, 4, 5, 6, 6]);
    let (mut unsorted_5, sorted_5) = (vec![-1, -1, -3, 4, 3], vec![-3, -1, -1, 3, 4]);

    println!("{}",
        Black.bold().paint("Assertion Test"),
    );

    compare_vectors(func, &mut unsorted_1, &sorted_1);
    compare_vectors(func, &mut unsorted_2, &sorted_2);
    compare_vectors(func, &mut unsorted_3, &sorted_3);
    compare_vectors(func, &mut unsorted_4, &sorted_4);
    compare_vectors(func, &mut unsorted_5, &sorted_5);
    println!();
}


fn compare_vectors(func: &dyn Fn(&mut Vec<i32>), unsorted: &mut Vec<i32>, sorted: &Vec<i32>) {
    func(unsorted);

    if unsorted == sorted {
        println!("{}{}{}{}{}",
            Green.bold().paint(get_vector_as_string(&unsorted)),
            Green.paint(" == "),
            Green.bold().paint(get_vector_as_string(&sorted)),
            Green.paint(" = "),
            Green.bold().paint("True ✅"),
        );
    } else {
        println!("{}{}{}{}{}",
            Red.bold().paint(get_vector_as_string(&unsorted)),
            Red.paint(" == "),
            Red.bold().paint(get_vector_as_string(&sorted)),
            Red.paint(" = "),
            Red.bold().paint("False ❌"),
        );
    }
}


pub fn average_time_test(func: &dyn Fn(&mut Vec<i32>), trials: i32) {
    println!("{}",
        Black.bold().paint("Average Time Test"),
    );

    let mut times: Vec<f32> = Vec::with_capacity(trials as usize);

    const VECTOR_LENGTH: usize = 1_000;
    const RAND_RANGE: Range<i32> = -500 .. 500;

    println!("{}{}{}{}{}{}{}{}{}",
        Blue.paint("Settings: "),
        Blue.bold().paint("{ VECTOR_LENGTH: "),
        Blue.bold().paint(VECTOR_LENGTH),
        Blue.bold().paint(", RAND_RANGE: "),
        Blue.bold().paint(RAND_RANGE.start),
        Blue.bold().paint(" .. "),
        Blue.bold().paint(RAND_RANGE.end),
        Blue.bold().paint(" }"),
        Blue.paint(" 🔧"),
    );

    println!("{}{}{}{}{}",
        Yellow.paint("Starting Average Time Test with "),
        Yellow.bold().paint("'"),
        Yellow.bold().paint(trials),
        Yellow.bold().paint("'"),
        Yellow.paint(" trials... ⏳"),
    );

    for _i in 0 .. trials {
        let mut vec: Vec<i32> = Vec::with_capacity(VECTOR_LENGTH);
        for _i in 0 .. VECTOR_LENGTH { vec.push(rand::thread_rng().gen_range(RAND_RANGE)); }
        
        let begin_time: Instant = Instant::now();
        func(&mut vec);
        let end_time: Duration = begin_time.elapsed();
        times.push(end_time.as_secs_f32());
    }

    let sum: f32 = times.iter().sum();
    let average: f32 = sum / times.len() as f32;

    println!("{}",
        Green.bold().paint("Completed Average Time Test! ✅"),
    );

    println!("{}{}{}",
        Magenta.paint("Average Running Time: "),
        Magenta.bold().paint(average),
        Magenta.bold().paint("s 🕑\n"),
    );
}