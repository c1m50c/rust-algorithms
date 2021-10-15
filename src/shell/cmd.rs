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
            println!(")");
        }

        "quit" | "exit" => {
            exit(0);
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