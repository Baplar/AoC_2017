/// Parses a single line describing a scanner
fn parse_scanner(s: &str) -> Result<(usize, usize), String> {
    let mut it = s.trim().split(": ");
    let depth = it.next()
        .ok_or("Missing depth")?
        .parse()
        .map_err(|e| format!("Could not parse depth as int: {}", e))?;
    let range = it.next()
        .ok_or("Missing range")?
        .parse()
        .map_err(|e| format!("Could not parse range as int: {}", e))?;
    Ok((depth, range))
}

/// Parses the range of the scanner of each depth
/// provided in the input
fn parse_scanners(s: &str) -> Vec<(usize, usize)> {
    s.trim()
        .split('\n')
        .filter_map(|line| parse_scanner(line).ok())
        .collect()
}

/// Calculates the penalty at the depth
/// if the packet leaves at t=offset
///
/// # Examples
/// ```
/// use advent_of_code::day13::penalty;
/// assert_eq!(true, penalty(0, 3, 0));
/// assert_eq!(false, penalty(1, 2, 0));
/// assert_eq!(false, penalty(4, 4, 0));
/// assert_eq!(true, penalty(6, 4, 0));
/// ```
#[inline]
pub fn penalty(depth: usize, range: usize, offset: usize) -> bool {
    range > 0 && (depth + offset) % (2 * (range - 1)) == 0
}

/// Calculates the severity of a trip through the firewall
/// when leaving at t=0
///
/// # Examples
/// ```
/// use advent_of_code::day13::one;
/// let s = "\
/// 0: 3
/// 1: 2
/// 4: 4
/// 6: 4";
/// assert_eq!("24", one(s));
/// ```
pub fn one(s: &str) -> String {
    let severity: usize = parse_scanners(s)
        .into_iter()
        .filter(|&(d, r)| penalty(d, r, 0))
        .map(|(d, r)| d * r)
        .sum();
    severity.to_string()
}

/// Finds the lowest number of picoseconds to wait
/// before leaving in order not to be caught.
///
/// # Warning
/// For now, the algorithm uses brute force.
/// This can take a long time for large inputs.
///
/// # Examples
/// ```
/// use advent_of_code::day13::two;
/// let s = "\
/// 0: 3
/// 1: 2
/// 4: 4
/// 6: 4";
/// assert_eq!("10", two(s));
/// ```
pub fn two(s: &str) -> String {
    let scanners = parse_scanners(s);
    (0..)
        .find(|&offset| scanners.iter().all(|&(d, r)| !penalty(d, r, offset)))
        .unwrap_or(0)
        .to_string()
}
