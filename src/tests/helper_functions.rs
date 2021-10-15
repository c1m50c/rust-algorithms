use std::vec::Vec;
use rand::Rng;


pub fn create_integer_vector(length: i32, rand_min: i32, rand_max: i32) -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::with_capacity(length as usize);

    for _i in 0 .. length {
        let number: i32 = rand::thread_rng().gen_range(rand_min .. rand_max);
        vec.push(number);
    }

    return vec;
}