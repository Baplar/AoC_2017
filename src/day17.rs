/// Simulates the circular buffer
/// and finds the number positioned
/// after the 2017th inserted number.
///
/// # Examples
/// ```
/// use advent_of_code::day17::one;
/// assert_eq!("638", one("3"));
/// ```
pub fn one(s: &str) -> String {
    let n = s.trim().parse().unwrap_or(0);
    let mut v = vec![0];
    let mut i = 0;
    for j in 1..2018 {
        i = (i + n) % j;
        v.insert(i + 1, j);
        i += 1;
    }

    v[i + 1].to_string()
}

/// Instead of simulating the whole buffer,
/// only simulates the first two elements
/// and saves only the relevant data.
pub fn two(s: &str) -> String {
    let n = s.trim().parse().unwrap_or(0);
    let mut v1 = 0;
    let mut i = 0;
    for j in 1..50_000_000 {
        i = (i + n) % j;
        if i == 0 {
            v1 = j;
        }
        i += 1;
        if j % 10_000 == 0 {
            println!("{}", j);
        }
    }
    v1.to_string()
}
