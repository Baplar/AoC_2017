pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;
pub mod day12;

use std::fs::File;
use std::io::Read;
extern crate regex;

pub fn get_solver(day: u32, part: u32) -> Option<fn(&str) -> String>
{
    match (day, part) {
        // (1, 1) => Some(day1::one),
        // (1, 2) => Some(day1::two),
        // (2, 1) => Some(day2::one),
        // (2, 2) => Some(day2::two),
        // (3, 1) => Some(day3::one),
        // (3, 2) => Some(day3::two),
        // (4, 1) => Some(day4::one),
        // (4, 2) => Some(day4::two),
        // (5, 1) => Some(day5::one),
        // (5, 2) => Some(day5::two),
        // (6, 1) => Some(day6::one),
        // (6, 2) => Some(day6::two),
        // (7, 1) => Some(day7::one),
        // (7, 2) => Some(day7::two),
        // (8, 1) => Some(day8::one),
        // (8, 2) => Some(day8::two),
        // (9, 1) => Some(day9::one),
        // (9, 2) => Some(day9::two),
        // (10, 1) => Some(day10::one),
        // (10, 2) => Some(day10::two),
        // (11, 1) => Some(day11::one),
        // (11, 2) => Some(day11::two),
        (12, 1) => Some(day12::one),
        (12, 2) => Some(day12::two),
        _ => None
    }
}

pub fn get_input(filename: &str) -> Result<String, std::io::Error> {
    let mut contents = String::new();
    let mut f = File::open(filename)?;
    f.read_to_string(&mut contents)?;
    Ok(contents)
}