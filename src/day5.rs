/// Calculates the number of steps to leave the list
/// 
/// # Examples
/// ```
/// use advent_of_code::day5::one;
/// assert_eq!("5", one("0 3 0 1 -3"));
/// ```
pub fn one(s: &str) -> String {
    let mut offsets: Vec<isize> = s.split_whitespace().filter_map(|w| w.parse().ok()).collect();
    let size = offsets.len();
    let mut index = 0;
    let mut steps = 0;
    while index < size {
        steps += 1;
        offsets[index] += 1;
        if (index as isize) + offsets[index] - 1 < 0 {
            break;
        }
        index = ((index as isize) + offsets[index] - 1) as usize;
    }
    steps.to_string()
}

/// Calculates the number of steps to leave the list
/// 
/// # Examples
/// ```
/// use advent_of_code::day5::two;
/// assert_eq!("10", two("0 3 0 1 -3"));
/// ```
pub fn two(s: &str) -> String {
    let mut offsets: Vec<isize> = s.split_whitespace().filter_map(|w| w.parse().ok()).collect();
    let size = offsets.len();
    let mut index = 0;
    let mut steps = 0;
    while index < size {
        steps += 1;
        let old_offset = offsets[index];
        offsets[index] += if old_offset >= 3 {
            -1
        } else {
            1
        };

        if (index as isize) + old_offset < 0 {
            break;
        }
        index = ((index as isize) + old_offset) as usize;
    }
    steps.to_string()
}
