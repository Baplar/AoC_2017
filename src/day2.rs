/// Calculates the checksum of the spreadsheet
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
    let checksum = |line: &str| {
        let coll: Vec<u32> = line.split_whitespace().filter_map(|n| n.parse::<u32>().ok()).collect();
        let min = match coll.iter().min() {
            Some(&min) => min,
            None => 0
        };
        let max = match coll.iter().max() {
            Some(&max) => max,
            None => 0
        };
        max - min
    };
    let result: u32 = s.split("\n").map(checksum).sum();
    result.to_string()
}

/// Calculates the checksum of the spreadsheet
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
    let checksum = |line: &str| {
        // Unique elements sorted 
        let mut coll: Vec<u32> = line.split_whitespace().filter_map(|n| n.parse::<u32>().ok()).collect();
        coll.sort();
        coll.dedup();

        // Finding the only pair of non-coprime numbers in the list,
        // and returning their gcd
        for &larger in coll.iter().rev() {
            for &lower in coll.iter().take_while(|&x| *x < larger) {
                if larger % lower == 0 {
                    return larger / lower;
                }
            }
        }
        0
    };
    let result: u32 = s.split("\n").map(checksum).sum();
    result.to_string()
}
