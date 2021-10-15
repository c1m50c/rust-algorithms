use std::result::Result;
use std::vec::Vec;


pub fn linear_search(vec: &Vec<i32>, finding: i32) -> Result<usize, &'static str> {
    for index in 0 .. vec.len() {
        if vec[index] == finding {
            return Ok(index);
        }
    }

    return Err("Could not find `finding`.");
}