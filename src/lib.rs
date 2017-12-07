pub mod day1;
pub mod day2;
pub mod day3;

use std::fs::File;
use std::io::Read;

pub fn get_solver(day: u32, part: u32) -> Option<fn(&str) -> String>
{
    match (day, part) {
        (1, 1) => Some(day1::one),
        (1, 2) => Some(day1::two),
        (2, 1) => Some(day2::one),
        (2, 2) => Some(day2::two),
        (3, 1) => Some(day3::one),
        (3, 2) => Some(day3::two),
        _ => None
    }
}

pub fn get_input(filename: &str) -> Result<String, std::io::Error> {
    let mut contents = String::new();
    let mut f = File::open(filename)?;
    f.read_to_string(&mut contents)?;
    Ok(contents)
}