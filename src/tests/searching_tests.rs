use super::helper_functions::create_integer_vector;
use super::super::sorting_algorithms::merge_sort::merge_sort;

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
    speed_test(func, 1000, func_name);
    speed_test(func, 10000, func_name);
    speed_test(func, 100000, func_name);
    speed_test(func, 500000, func_name);
    speed_test(func, 1000000, func_name);
    speed_test(func, 5000000, func_name);
    speed_test(func, 10000000, func_name);
    speed_test(func, 50000000, func_name);

    /* Assertion Test */
    assertion_test(func);

    println!("<{}/>", func_name);
}


/// # Speed Test
/// Tests a searching algorithm's speed by searching through a vector of size `length`.
/// ### Parameters:
/// ```rust
/// func: &dyn Fn(&Vec<i32>, i32) -> Option<usize>  // The searching algorithm's coresponding function.
/// length: i32 // Desired len() of testing Vector.
/// ```
pub fn speed_test(func: &dyn Fn(&Vec<i32>, i32) -> Option<usize>, length: i32, func_name: &'static str) {
    println!("{}",
        Black.bold().paint("Search Speed Test"),
    );

    let rand_min: i32 = -length;
    let rand_max: i32 = length;

    let mut vec: Vec<i32> = create_integer_vector(length, rand_min, rand_max);

    /* Sort vector for functions that require sorting */
    if func_name == "BinarySearch" {
        pre_sort_vector(&mut vec, &merge_sort as &dyn Fn(&mut Vec<i32>), "MergeSort");
    }

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


/// # Assertion Test
/// Tests the searching algorithm to ensure that it can search properly.
/// ### Parameters:
/// ```rust
/// func: &dyn Fn(&Vec<i32>, i32) -> Option<usize> // Sorting Algorithm's coresponding function.
/// ```
pub fn assertion_test(func: &dyn Fn(&Vec<i32>, i32) -> Option<usize>) {
    /* Vec[  ], finding, index of finding */
    /* Pre-sorted for algorithms that need things to be sorted */
    let (vec_one, finding_one, index_one) = (vec![0, 1, 2, 3, 4, 5, 6], 3, 3);
    let (vec_two, finding_two, index_two) = (vec![3, 3, 4, 5, 7, 8, 9], 9, 6);
    let (vec_three, finding_three, index_three) = (vec![0, 0, 0, 4, 5, 6, 7], 2, -1);
    let (vec_four, finding_four, index_four) = (vec![-2, -1, 1, 0, 5, 6, 6], -1, 1);
    let (vec_five, finding_five, index_five) = (vec![-4, -3, 0, 5, 6, 7, 8], -4, 0);

    println!("{}",
        Black.bold().paint("Assertion Test"),
    );

    assert_compare(func, vec_one, finding_one, index_one);
    assert_compare(func, vec_two, finding_two, index_two);
    assert_compare(func, vec_three, finding_three, index_three);
    assert_compare(func, vec_four, finding_four, index_four);
    assert_compare(func, vec_five, finding_five, index_five);
    
    println!();
}


/// # Assert Compare
/// Compares the results from the searching algorithm against the known values for `finding`.
/// ### Parameters:
/// ```rust
/// func: &dyn Fn(&Vec<i32>, i32) -> Option<usize> // Sorting Algorithm's coresponding function.
/// vec: Vec<i32> // Vector to look through.
/// finding: i32 // What the searching algorithm is trying to find.
/// index: i32 // Known vector index of `finding`.
/// ```
fn assert_compare(func: &dyn Fn(&Vec<i32>, i32) -> Option<usize>, vec: Vec<i32>, finding: i32, index: i32) {
    let found: Option<usize> = func(&vec, finding);
    /* Todo: Display Vector */
    match found {
        Some(idx) => {
            if idx == index as usize {
                println!("{}{}{}{}{}{}{}{}",
                    Green.bold().paint("'"),
                    Green.bold().paint(finding),
                    Green.bold().paint("'"),
                    Green.paint(" is at "),
                    Green.bold().paint("'"),
                    Green.bold().paint(idx),
                    Green.bold().paint("'"),
                    Green.paint(" == True ✅"),
                );
            }

            else {
                println!("{}{}{}{}{}{}{}{}",
                    Red.bold().paint("'"),
                    Red.bold().paint(finding),
                    Red.bold().paint("'"),
                    Red.paint(" is at "),
                    Red.bold().paint("'"),
                    Red.bold().paint(idx),
                    Red.bold().paint("'"),
                    Red.paint(" == False ❌"),
                );
            }
        }

        None => {
            if index == -1 {
                println!("{}{}{}{}{}{}",
                    Green.bold().paint("'"),
                    Green.bold().paint(finding),
                    Green.bold().paint("'"),
                    Green.paint(" is at "),
                    Green.bold().paint("'-1'"),
                    Green.paint("  == True ✅"),
                );
            }

            else {
                println!("{}{}{}{}{}{}",
                    Red.bold().paint("'"),
                    Red.bold().paint(finding),
                    Red.bold().paint("'"),
                    Red.paint(" is at "),
                    Red.bold().paint("'-1'"),
                    Red.paint("  == False ❌"),
                );
            }
        }
    }
}


fn pre_sort_vector(vec: &mut Vec<i32>, sorting_func: &dyn Fn(&mut Vec<i32>), sorting_func_name: &'static str) {
    let begin_time: Instant = Instant::now();
    println!("{}{}{}{}{}",
        Yellow.paint("Pre-sorting Vector with "),
        Yellow.bold().paint("'"),
        Yellow.bold().paint(sorting_func_name),
        Yellow.bold().paint("'"),
        Yellow.paint(" ⏳"),
    );

    sorting_func(vec);
    let end_time: Duration = begin_time.elapsed();

    println!("{}{}{}{}{}{}{}{}{}",
        Green.paint("Pre-sorted Vector of length "),
        Green.bold().paint("'"),
        Green.bold().paint(vec.len()),
        Green.bold().paint("'"),
        Green.paint(" in "),
        Green.bold().paint("'"),
        Green.bold().paint(end_time.as_secs_f32()),
        Green.bold().paint("s'"),
        Green.paint(" ✅"),
    );
}