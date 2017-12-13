use std::str::Chars;

/// Parses the content of a well-formed croup,
/// and returns the corresponding group score
pub fn parse_group_score(stream: &mut Chars, level: usize) -> usize {
    let mut score = level;
    while let Some(c) = stream.next() {
        match c {
            '{' => {
                score += parse_group_score(stream, level + 1);
            }
            '<' => {
                score_garbage(stream);
            }
            '}' => {
                return score;
            }
            ',' => {} // Continue to the next sub-group
            _ => {
                eprintln!("Observed unexpected character '{}' in group", c);
            }
        }
    }
    score
}

/// Parses the content of a garbage block,
/// and returns the number of unescaped characters
pub fn parse_group_garbage(stream: &mut Chars) -> usize {
    let mut score = 0;
    while let Some(c) = stream.next() {
        match c {
            '{' => {
                score += parse_group_garbage(stream);
            }
            '<' => {
                score += score_garbage(stream);
            }
            '}' => {
                return score;
            }
            ',' => {} // Continue to the next sub-group
            _ => {
                eprintln!("Observed unexpected character '{}' in group", c);
            }
        }
    }
    score
}

/// Parses the content of a well-formed croup,
/// and returns the number of unescaped characters in its garbage
pub fn score_garbage(stream: &mut Chars) -> usize {
    let mut score = 0;
    while let Some(c) = stream.next() {
        match c {
            '>' => {
                return score;
            }
            '!' => {
                stream.next();
            }
            _ => {
                score += 1;
            }
        }
    }

    score
}

/// Calculates the score of a stream block
///
/// # Examples
/// ```
/// use advent_of_code::day9::one;
/// assert_eq!("1", one("{}"));
/// assert_eq!("6", one("{{{}}}"));
/// assert_eq!("5", one("{{},{}}"));
/// assert_eq!("16", one("{{{},{},{{}}}}"));
/// assert_eq!("1", one("{<a>,<a>,<a>,<a>}"));
/// assert_eq!("9", one("{{<ab>},{<ab>},{<ab>},{<ab>}}"));
/// assert_eq!("9", one("{{<!!>},{<!!>},{<!!>},{<!!>}}"));
/// assert_eq!("3", one("{{<a!>},{<a!>},{<a!>},{<ab>}}"));
/// ```
pub fn one(s: &str) -> String {
    let mut stream = s.trim().chars();
    parse_group_score(&mut stream, 0).to_string()
}

/// Calculates the number of characters
/// in the garbage of a stream block
///
/// # Examples
/// ```
/// use advent_of_code::day9::two;
/// assert_eq!("0", two("<>"));
/// assert_eq!("17", two("<random characters>"));
/// assert_eq!("3", two("<<<<>"));
/// assert_eq!("2", two("<{!>}>"));
/// assert_eq!("0", two("<!!>"));
/// assert_eq!("0", two("<!!!>>"));
/// assert_eq!("10", two("<{o\"i!a,<{i<a>"));
/// ```
pub fn two(s: &str) -> String {
    let mut stream = s.trim().chars();
    parse_group_garbage(&mut stream).to_string()
}
