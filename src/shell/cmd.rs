/* Searching Algorithms */
use super::super::searching_algorithms::linear_search::linear_search;
use super::super::searching_algorithms::binary_search::binary_search;

/* Sorting Algorithms */
use super::super::sorting_algorithms::selection_sort::selection_sort;
use super::super::sorting_algorithms::insertion_sort::insertion_sort;
use super::super::sorting_algorithms::merge_sort::merge_sort;
use super::super::sorting_algorithms::bubble_sort::bubble_sort;
use super::super::sorting_algorithms::quick_sort::quick_sort;

/* Tests */
use super::super::tests::searching_tests::run_tests as run_searching_tests;
use super::super::tests::sorting_tests::run_tests as run_sorting_tests;

use term_painter::{Color::*, ToStyle};
use std::process::exit;


/// # Run Command
/// Looks over the `input` for a valid command to run.
/// ### Parameters:
/// ```rust
/// input: String // String with the command you want to run, not case-sensitive.
/// ```
pub fn run_command(input: String) {
    match input.to_lowercase().as_ref() {
        "help" | "h" => {
            /* Help ~ Prints commands */
            println!("Help: Commands (");
            println!(". help, h {}", Cyan.paint("# Prints every command keyword."));
            println!(". quit, exit {}", Cyan.paint("# Quits the application."));
            println!(". run_all, all {}", Cyan.paint("# Runs every algorithm."));
            println!(". searching {}", Cyan.paint("# Runs every searching algorithm."));
            println!(". sorting {}", Cyan.paint("# Runs every sorting algorithm."));
            println!(". linear_search {}", Cyan.paint("# Runs the `linear_search` algorithm."));
            println!(". binary_search {}", Cyan.paint("# Runs the `binary_search` algorithm."));
            println!(". selection_sort {}", Cyan.paint("# Runs the `selection_sort` algorithm."));
            println!(". insertion_sort {}", Cyan.paint("# Runs the `insertion_sort` algorithm."));
            println!(". merge_sort {}", Cyan.paint("# Runs the `merge_sort` algorithm."));
            println!(". bubble_sort {}", Cyan.paint("# Runs the `bubble_sort` algorithm."));
            println!(". quick_sort {}", Cyan.paint("# Runs the `quick_sort` algorithm."));
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
            run_sorting_tests(&quick_sort as &dyn Fn(&mut Vec<i32>), "QuickSort");
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
            run_sorting_tests(&quick_sort as &dyn Fn(&mut Vec<i32>), "QuickSort");
        }

        "linear_search" => {
            /* LinearSearch ~ Runs the `linear_search` algorithm. */
            run_searching_tests(&linear_search as &dyn Fn(&Vec<i32>, i32) -> Option<usize>, "LinearSearch");
        }

        "binary_search" => {
            /* BinarySearch ~ Runs the `binary_search` algorithm. */
            run_searching_tests(&binary_search as &dyn Fn(&Vec<i32>, i32) -> Option<usize>, "BinarySearch");
        }

        "selection_sort" => {
            /* SelectionSort ~ Runs the `selection_sort` algorithm. */
            run_sorting_tests(&selection_sort as &dyn Fn(&mut Vec<i32>), "SelectionSort");
        }

        "insertion_sort" => {
            /* InsertionSort ~ Runs the `insertion_sort` algorithm. */
            run_sorting_tests(&insertion_sort as &dyn Fn(&mut Vec<i32>), "InsertionSort");
        }

        "merge_sort" => {
            /* MergeSort ~ Runs the `merge_sort` algorithm. */
            run_sorting_tests(&merge_sort as &dyn Fn(&mut Vec<i32>), "MergeSort");
        }

        "bubble_sort" => {
            /* BubbleSort ~ Runs the `bubble_sort` algorithm. */
            run_sorting_tests(&bubble_sort as &dyn Fn(&mut Vec<i32>), "BubbleSort");
        }

        "quick_sort" => {
            /* QuickSort ~ Runs the `quick_sort` algorithm. */
            run_sorting_tests(&quick_sort as &dyn Fn(&mut Vec<i32>), "QuickSort");
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