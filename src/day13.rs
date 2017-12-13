use std::collections::HashMap;

/// Parses the range of the scanner of each depth
/// provided in the input
fn parse_scanners(s: &str) -> HashMap<usize, usize> {
    let mut result = HashMap::new();
    for line in s.trim().split("\n") {
        let mut it = line.trim().split(": ");
        let depth = match it.next() {
            Some(d) => match d.parse() {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("Could not parse {} as int: {}", d, e);
                    continue;
                }
            },
            None => {
                eprintln!("Missing depth");
                continue;
            }
        };
        let range = match it.next() {
            Some(r) => match r.parse() {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("Could not parse {} as int: {}", r, e);
                    continue;
                }
            },
            None => {
                eprintln!("Missing range");
                continue;
            }
        };
        result.insert(depth, range);
    }
    result
}

/// Calculates the penalty at the depth
/// if the packet leaves at t=offset
/// 
/// # Examples
/// ```
/// use std::collections::HashMap;
/// use advent_of_code::day13::penalty;
/// let mut map = HashMap::new();
/// map.insert(0, 3);
/// map.insert(1, 2);
/// map.insert(4, 4);
/// map.insert(6, 4);
/// assert_eq!(true, penalty(&map, 0, 0));
/// assert_eq!(false, penalty(&map, 1, 0));
/// assert_eq!(false, penalty(&map, 4, 0));
/// assert_eq!(true, penalty(&map, 6, 0));
/// ```
pub fn penalty(scanners: &HashMap<usize, usize>, depth: usize, offset: usize) -> bool {
    let &range = scanners.get(&depth).unwrap_or(&0);
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
    let scanners = parse_scanners(s);
    let mut severity = 0;
    for &depth in scanners.keys() {
        if penalty(&scanners, depth, 0) {
            severity += depth * scanners.get(&depth).unwrap_or(&0);
        };
    }

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
    let mut offset = 0;
    let mut found = true;
    while found {
        found = false;
        offset += 1;

        for &depth in scanners.keys() {
            if penalty(&scanners, depth, offset) {
                found = true;
                break
            };
        }
    }

    offset.to_string()
}