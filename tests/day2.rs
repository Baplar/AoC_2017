extern crate advent_of_code;
use advent_of_code::day2;
use advent_of_code::get_input;

#[test]
fn one() {
    assert_eq!("58975", day2::one(get_input("day2").unwrap().as_str()));
}

#[test]
fn two() {
    assert_eq!("308", day2::two(get_input("day2").unwrap().as_str()));
}