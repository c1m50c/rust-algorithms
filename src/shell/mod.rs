use term_painter::{Color::*, ToStyle};
use std::io::{self, Write};


pub fn run() {
    loop {
        let inp = get_input();
        println!("{}", inp);
    }
}


fn get_input() -> String {
    let mut input: String = String::new();
    
    print!("{}{}",
        /* Prompt */
        BrightMagenta.bold().paint("rust-algorithms"),
        BrightGreen.bold().paint(":$ "),
    );

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line.");
    return input;
}