/// Computes the "captcha sum" of a string representing a sequence of digits.
///
/// # Examples
/// ```
/// use advent_of_code::day1::one;
/// assert_eq!("3", one("1122"));
/// assert_eq!("4", one("1111"));
/// assert_eq!("0", one("1234"));
/// assert_eq!("9", one("91212129"));
/// ```
pub fn one(s: &str) -> String {
    // We create a vector of the digits of the input
    let coll: Vec<u32> = s.chars().filter_map(|c| c.to_digit(10)).collect();
    let n = coll.len();

    // We create a first iterator on the collection
    let first = coll.iter();
    // Then a second iterator which skips the first character and puts it at the end
    let second = coll.iter().cycle().skip(1).take(n);

    // We take the sum of the "duplicated" characters
    let result: u32 = first
        .zip(second)
        .filter(|&(&x, &y)| x == y)
        .map(|(&x, _)| x)
        .sum();
    result.to_string()
}

/// Same thing, but according to the second algorithm
///
/// # Examples
/// ```
/// use advent_of_code::day1::two;
/// assert_eq!("6", two("1212"));
/// assert_eq!("0", two("1221"));
/// assert_eq!("4", two("123425"));
/// assert_eq!("12", two("123123"));
/// assert_eq!("4", two("12131415"));
/// ```
pub fn two(s: &str) -> String {
    // We create a vector of the digits of the input
    let coll: Vec<u32> = s.chars().filter_map(|c| c.to_digit(10)).collect();
    let n = coll.len();

    // We create a first iterator on the collection
    let first = coll.iter();
    // Then a second iterator which skips the first character and puts it at the end
    let second = coll.iter().cycle().skip(n / 2).take(n);

    // We take the sum of the "duplicated" characters
    let result: u32 = first
        .zip(second)
        .filter(|&(&x, &y)| x == y)
        .map(|(&x, _)| x)
        .sum();
    result.to_string()
}
