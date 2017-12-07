extern crate advent_of_code;
use advent_of_code::day1;
use advent_of_code::get_input;

#[test]
fn one() {
    assert_eq!("1031", day1::one(get_input("day1").unwrap().as_str()));
}

#[test]
fn two() {
    assert_eq!("1080", day1::two(get_input("day1").unwrap().as_str()));
}