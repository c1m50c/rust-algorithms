use std::time::{Duration, Instant};
use std::sync::mpsc;
use std::vec::Vec;
use std::thread;

use term_painter::{Color::*, ToStyle};
use rand::Rng;


/// # Create Integer Vector
/// Creats a vector filled with `i32` integers, workload split across two threads.
/// ### Parameters:
/// ```rust
/// length: i32 // Desired length of the returned vector.
/// rand_min: i32 // Minimum random value assigned to each integer.
/// rand_max: i32 // Maximum random value assigned to each integer.
/// ```
pub fn create_integer_vector(length: i32, rand_min: i32, rand_max: i32) -> Vec<i32> {
    println!("{}{}{}{}{}",
        Yellow.paint("Creating Vector of length "),
        Yellow.bold().paint("'"),
        Yellow.bold().paint(length),
        Yellow.bold().paint("'"),
        Yellow.paint(" ⏳"),
    );
    let begin_time: Instant = Instant::now();

    /* Todo: Make parameter for threads to use? */
    /* Split length into half for each thread to loop over for their respective vector. */
    let first_slice: i32 = length / 2;
    let second_slice: i32 = length - first_slice;

    let (t, r) = mpsc::channel(); // Create Channel for extending main vector later.
    let mut main_vec: Vec<i32> = Vec::with_capacity(length as usize);

    /* Alt Thread */
    let handle = thread::spawn(move || {
        let mut vec: Vec<i32> = Vec::with_capacity(second_slice as usize);
        for _i in 0 .. second_slice {
            let number: i32 = rand::thread_rng().gen_range(rand_min .. rand_max);
            vec.push(number);
        }
        t.send(vec).unwrap();
    });

    /* Main Thread */
    for _i in 0 .. first_slice {
        let number: i32 = rand::thread_rng().gen_range(rand_min .. rand_max);
        main_vec.push(number);
    }

    handle.join().unwrap();
    main_vec.extend(r.recv().unwrap());

    let elapsed_time: Duration = begin_time.elapsed();
    println!("{}{}{}{}{}{}{}{}{}",
        Green.paint("Created Vector of length "),
        Green.bold().paint("'"),
        Green.bold().paint(length),
        Green.bold().paint("'"),
        Green.paint(" in "),
        Green.bold().paint("'"),
        Green.bold().paint(elapsed_time.as_secs_f32()),
        Green.bold().paint("s'"),
        Green.paint(" ✅"),
    );

    return main_vec;
}


/// # Get Vector as String
/// Formats a vector to a string.
/// ### Example:
/// ```rust
/// let vec = vec![4, 5, 3, 21, 20];
/// assert_eq!(get_vector_as_string(&vec), "[4 .. 20]");
/// ```
/// ### Parameters:
/// ```rust
/// vec: &Vec<i32> // Vector to format to get as string.
/// ```
pub fn get_vector_as_string(vec: &Vec<i32>) -> String {
    return format!("[{} .. {}]", vec[0], vec[vec.len() - 1]);
}