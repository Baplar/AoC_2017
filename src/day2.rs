/// Calculates the difference between the maximum
/// and minimum values in the list
fn checksum_one(s: &str) -> u32 {
        let coll: Vec<u32> = s.split_whitespace().filter_map(|n| n.parse().ok()).collect();
        let min = coll.iter().min().unwrap_or(&0);
        let max = coll.iter().max().unwrap_or(&0);
        max - min
}

/// Calculates the checksum of the spreadsheet
/// with the first definition
/// 
/// # Examples
/// ```
/// use advent_of_code::day2::one;
/// let spreadsheet = "\
/// 5 1 9 5
/// 7 5 3
/// 2 4 6 8";
/// assert_eq!("18", one(spreadsheet));
/// ```
pub fn one(s: &str) -> String {
    let result: u32 = s.split("\n").map(checksum_one).sum();
    result.to_string()
}

/// Calculates the gcd of the only pair
/// of non-coprime numbers in the list
fn checksum_two(s: &str) -> u32 {
        // Unique elements sorted 
        let mut coll: Vec<u32> = s.split_whitespace().filter_map(|n| n.parse().ok()).collect();
        coll.sort();
        coll.dedup();

        // Finding the only pair of non-coprime numbers in the list,
        // and returning their gcd
        for larger in coll.iter().rev() {
            for lower in coll.iter().take_while(|&x| x < larger) {
                if larger % lower == 0 {
                    return larger / lower;
                }
            }
        }
        0
}

/// Calculates the checksum of the spreadsheet
/// with the second definition
/// 
/// # Examples
/// ```
/// use advent_of_code::day2::two;
/// let spreadsheet = "\
/// 5 9 2 8
/// 9 4 7 3
/// 3 8 6 5";
/// assert_eq!("9", two(spreadsheet));
/// ```
pub fn two(s: &str) -> String {
    let result: u32 = s.split("\n").map(checksum_two).sum();
    result.to_string()
}
