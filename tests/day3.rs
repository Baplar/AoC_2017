extern crate advent_of_code;
use advent_of_code::day3;
use advent_of_code::get_input;

#[test]
fn one() {
    assert_eq!("552", day3::one(get_input("day3").unwrap().as_str()));
}

#[test]
fn two() {
    assert_eq!("330785", day3::two(get_input("day3").unwrap().as_str()));
}