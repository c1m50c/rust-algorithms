/* Searching Algorithms */
use super::super::searching_algorithms::binary_search::binary_search;
use super::super::searching_algorithms::linear_search::linear_search;

/* Sorting Algorithms */
use super::super::sorting_algorithms::selection_sort::selection_sort;

/* Tests */
use super::super::tests::searching_tests::run_tests as run_searching_tests;

use term_painter::{Color::*, ToStyle};
use std::process::exit;


/// # Run Command
/// Looks over the `input` for a valid command to run.
/// ### Parameters:
/// ```rust
/// input: String // String with the command you want to run.
/// ```
pub fn run_command(input: String) {
    /* Todo: Try to find a simpler way to do this. */
    match input.to_lowercase().as_ref() {
        "help" | "h" => {
            /* Todo: Find a more efficent way to print known commands, rather than inputing them by hand. */
            println!("Help: Commands (");
            println!("help, h");
            println!("quit, exit");
            println!("run_all, all");
            println!("searching");
            println!("linear_search");
            println!(")");
        }

        "quit" | "exit" => {
            exit(0);
        }

        "run_all" | "all" => {
            run_searching_tests(&linear_search as &dyn Fn(&Vec<i32>, i32) -> Option<usize>, "LinearSearch");
        }

        "searching" => {
            run_searching_tests(&linear_search as &dyn Fn(&Vec<i32>, i32) -> Option<usize>, "LinearSearch");
        }

        "linear_search" => {
            run_searching_tests(&linear_search as &dyn Fn(&Vec<i32>, i32) -> Option<usize>, "LinearSearch");
        }

        _ => {
            println!("{}{}{}{}{}{}{}",
                "Command ",
                Red.bold().paint("'"),
                Red.bold().paint(input),
                Red.bold().paint("'"),
                " does not exist, try ",
                Green.bold().paint("'help'"),
                ".",
            );
        }
    }
}