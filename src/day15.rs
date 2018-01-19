/// Retrieves the starting values of the generators
fn parse_input(s: &str) -> (usize, usize) {
    let v: Vec<usize> = s.trim()
        .split('\n')
        .take(2)
        .filter_map(|s| s.trim().split_whitespace().last()?.parse().ok())
        .collect();
    if v.len() < 2 {
        return (0, 0);
    }
    (v[0], v[1])
}

/// A sequence generator
struct Generator {
    factor: usize,
    value: usize,
}

impl Generator {
    fn new(factor: usize, value: usize) -> Generator {
        Generator { factor, value }
    }
}

impl Iterator for Generator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.value = (self.value * self.factor) % 2_147_483_647;
        Some(self.value)
    }
}

/// Compares the last 16 bits of two numbers,
/// returning true if they match.
fn judge(a: usize, b: usize) -> bool {
    (a ^ b).trailing_zeros() >= 16
}

/// Counts the number of matching pairs
/// in the first 40 million pairs generated by A and B
///
/// # Examples
/// ```
/// use advent_of_code::day15::one;
/// let s = "65\n8921";
/// assert_eq!("588", one(s));
/// ```
pub fn one(s: &str) -> String {
    let (value_a, value_b) = parse_input(s);
    let a = Generator::new(16_807, value_a);
    let b = Generator::new(48_271, value_b);

    a.zip(b)
        .take(40_000_000)
        .filter(|&(a, b)| judge(a, b))
        .count()
        .to_string()
}

/// Counts the number of matching pairs
/// in the first 40 million pairs generated by A and B
///
/// # Examples
/// ```
/// use advent_of_code::day15::two;
/// let s = "65\n8921";
/// assert_eq!("309", two(s));
/// ```
pub fn two(s: &str) -> String {
    let (value_a, value_b) = parse_input(s);
    let a = Generator::new(16_807, value_a).filter(|x| x % 4 == 0);
    let b = Generator::new(48_271, value_b).filter(|x| x % 8 == 0);

    a.zip(b)
        .take(5_000_000)
        .filter(|&(a, b)| judge(a, b))
        .count()
        .to_string()
}
