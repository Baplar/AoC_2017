/// Checks the validity of a passphrase
/// 
/// # Examples
/// ```
/// use advent_of_code::day4::valid;
/// assert!(valid("aa bb cc dd ee"));
/// assert!(!valid("aa bb cc dd aa"));
/// assert!(valid("aa bb cc dd aaa"));
/// ```
pub fn valid(s: &str) -> bool {
    let mut words: Vec<&str> = s.split_whitespace().collect();
    let nb_words = words.len();
    if nb_words == 0 {
        return false;
    }
    words.sort();
    words.dedup();
    let nb_unique = words.len();
    nb_unique == nb_words
}

pub fn one(s: &str) -> String {
    s.split("\n").filter(|&s| valid(s)).count().to_string()
}

/// Checks the validity of a passphrase forbidding anagrams
/// 
/// # Examples
/// ```
/// use advent_of_code::day4::valid_anagram;
/// assert!(valid_anagram("abcde fghij"));
/// assert!(!valid_anagram("abcde xyz ecdab"));
/// assert!(valid_anagram("a ab abc abd abf abj"));
/// assert!(valid_anagram("iiii oiii ooii oooi oooo"));
/// assert!(!valid_anagram("oiii ioii iioi iiio"));
/// ```
pub fn valid_anagram(s: &str) -> bool {
    let mut words: Vec<String> = s
        .split_whitespace()
        .map(|w| {
            let mut chars: Vec<char> = w.chars().collect();
            chars.sort();
            chars.iter().fold(String::new(), |acc, &c| acc + c.to_string().as_str())})
        .collect();
    let nb_words = words.len();
    if nb_words == 0 {
        return false;
    }
    
    words.sort();
    words.dedup();
    let nb_unique = words.len();
    nb_unique == nb_words
}

pub fn two(s: &str) -> String {
    s.split("\n").filter(|&s| valid_anagram(s)).count().to_string()
}
