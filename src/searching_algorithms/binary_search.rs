use std::result::Result;
use std::vec::Vec;


pub fn binary_search(vec: &Vec<i32>, finding: i32) -> Result<usize, &'static str> {
    let mut left: usize = 0;
    let mut right: usize = vec.len() - 1;

    while left <= right {
        let middle: usize = (left + right) / 2;
        if finding == vec[middle] { return Ok(middle); }
        else if finding < vec[middle] { right = middle - 1; }
        else { left = middle + 1; }
    }

    return Err("Could not find `finding`.");
}