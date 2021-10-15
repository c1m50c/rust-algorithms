use term_painter::{Color::*, ToStyle};
use std::io::{self, Write};


pub(crate) mod cmd;


/// # Run
/// Runs the shell.
pub fn run() {
    loop {
        /* Todo: Get colors visible */
        let inp = get_input();

        cmd::run_command(inp);
    }
}


/// # Get Input
/// Gets user input from the terminal.
/// ### Paramaters:
/// ```rust
/// prompt: String // Prompt to be shown before user's input.
/// ```
fn get_input() -> String {
    let mut input: String = String::new();

    print!("{}{}",
        /* Prompt */
        BrightMagenta.bold().paint("rust-algorithms"),
        BrightGreen.bold().paint(":$ ")
    );

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line.");
    return input.trim().to_string();
}