/* Searching Algorithms */
use super::super::searching_algorithms::linear_search::linear_search;
use super::super::searching_algorithms::binary_search::binary_search;

/* Sorting Algorithms */
use super::super::sorting_algorithms::selection_sort::selection_sort;
use super::super::sorting_algorithms::insertion_sort::insertion_sort;
use super::super::sorting_algorithms::merge_sort::merge_sort;
use super::super::sorting_algorithms::bubble_sort::bubble_sort;

/* Tests */
use super::super::tests::searching_tests::run_tests as run_searching_tests;
use super::super::tests::sorting_tests::run_tests as run_sorting_tests;

use term_painter::{Color::*, ToStyle};
use std::process::exit;


/// # Run Command
/// Looks over the `input` for a valid command to run.
/// ### Parameters:
/// ```rust
/// input: String // String with the command you want to run.
/// ```
pub fn run_command(input: String) {
    /* Todo: This is really spaghetti, find a simpler way to do this. */
    match input.to_lowercase().as_ref() {
        "help" | "h" => {
            /* Todo: Find a more efficent way to print known commands, rather than inputing them by hand. */
            /* Help ~ Prints commands */
            println!("Help: Commands (");
            println!("help, h");
            println!("quit, exit");
            println!("run_all, all");
            println!("searching");
            println!("sorting");
            println!("linear_search");
            println!("binary_search");
            println!("selection_sort");
            println!("insertion_sort");
            println!("merge_sort");
            println!("bubble_sort");
            println!(")");
        }

        "quit" | "exit" => {
            /* Quit ~ Quits the application */
            exit(0);
        }

        "run_all" | "all" => {
            /* Run All ~ Runs all algorithms */
            run_searching_tests(&linear_search as &dyn Fn(&Vec<i32>, i32) -> Option<usize>, "LinearSearch");
            run_searching_tests(&binary_search as &dyn Fn(&Vec<i32>, i32) -> Option<usize>, "BinarySearch");
            run_sorting_tests(&selection_sort as &dyn Fn(&mut Vec<i32>), "SelectionSort");
            run_sorting_tests(&insertion_sort as &dyn Fn(&mut Vec<i32>), "InsertionSort");
            run_sorting_tests(&merge_sort as &dyn Fn(&mut Vec<i32>), "MergeSort");
            run_sorting_tests(&bubble_sort as &dyn Fn(&mut Vec<i32>), "BubbleSort");
        }

        "searching" => {
            /* Searching ~ Runs all searching algorithms */
            run_searching_tests(&linear_search as &dyn Fn(&Vec<i32>, i32) -> Option<usize>, "LinearSearch");
            run_searching_tests(&binary_search as &dyn Fn(&Vec<i32>, i32) -> Option<usize>, "BinarySearch");
        }

        "sorting" => {
            /* Sorting ~ Runs all sorting algorithms */
            run_sorting_tests(&selection_sort as &dyn Fn(&mut Vec<i32>), "SelectionSort");
            run_sorting_tests(&insertion_sort as &dyn Fn(&mut Vec<i32>), "InsertionSort");
            run_sorting_tests(&merge_sort as &dyn Fn(&mut Vec<i32>), "MergeSort");
            run_sorting_tests(&bubble_sort as &dyn Fn(&mut Vec<i32>), "BubbleSort");
        }

        "linear_search" => {
            run_searching_tests(&linear_search as &dyn Fn(&Vec<i32>, i32) -> Option<usize>, "LinearSearch");
        }

        "binary_search" => {
            run_searching_tests(&binary_search as &dyn Fn(&Vec<i32>, i32) -> Option<usize>, "BinarySearch");
        }

        "selection_sort" => {
            run_sorting_tests(&selection_sort as &dyn Fn(&mut Vec<i32>), "SelectionSort");
        }

        "insertion_sort" => {
            run_sorting_tests(&insertion_sort as &dyn Fn(&mut Vec<i32>), "InsertionSort");
        }

        "merge_sort" => {
            run_sorting_tests(&merge_sort as &dyn Fn(&mut Vec<i32>), "MergeSort");
        }

        "bubble_sort" => {
            run_sorting_tests(&bubble_sort as &dyn Fn(&mut Vec<i32>), "BubbleSort");
        }

        _ => {
            /* Invalid Command ~ Gives feedback when entering a invalid command */
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