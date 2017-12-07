/// Redistributes the blocks of the bank with the most blocks
/// 
/// # Examples
/// ```
/// use advent_of_code::day6::redistribute;
/// let banks = vec![0, 2, 7, 0];
/// let banks = redistribute(&banks);
/// assert_eq!(banks, [2, 4, 1, 2]);
/// let banks = redistribute(&banks);
/// assert_eq!(banks, [3, 1, 2, 3]);
/// let banks = redistribute(&banks);
/// assert_eq!(banks, [0, 2, 3, 4]);
/// let banks = redistribute(&banks);
/// assert_eq!(banks, [1, 3, 4, 1]);
/// let banks = redistribute(&banks);
/// assert_eq!(banks, [2, 4, 1, 2]);
/// ```
pub fn redistribute(banks: &Vec<usize>) -> Vec<usize> {
    let n = banks.len();
    let (i_max, max) = {
        let mut i_max = 0;
        let mut max = 0;
        for (i, &val) in banks.iter().enumerate() {
            if val > max {
                i_max = i;
                max = val;
            }
        }
        (i_max, max)
    };
    let q = max / n;
    let r = max % n;
    let mut new_banks = banks.clone();
    new_banks[i_max] = 0;
    for i in 0..n {
        new_banks[i] += q;
        if (n + i - (i_max + 1)) % n < r {
            new_banks[i] += 1;
        }
    };
    new_banks
}

/// Finds the number of iterations before looping
/// 
/// # Examples
/// ```
/// use advent_of_code::day6::one;
/// assert_eq!("5", one("0 2 7 0"));
/// ```
pub fn one(s: &str) -> String {
    let banks: Vec<usize> = s.split_whitespace().filter_map(|w| w.parse().ok()).collect();
    let mut all_banks = vec![banks];
    let mut i = 0;
    loop {
        i += 1;
        let banks = redistribute(all_banks.last().unwrap());
        if all_banks.iter().any(|b| *b == banks) {
            break;
        }
        all_banks.push(banks);
    };
    i.to_string()
}

/// Finds the number of cycles in the infinite loop
/// 
/// # Examples
/// ```
/// use advent_of_code::day6::two;
/// assert_eq!("4", two("0 2 7 0"));
/// ```
pub fn two(s: &str) -> String {
    let banks: Vec<usize> = s.split_whitespace().filter_map(|w| w.parse().ok()).collect();
    let mut all_banks = vec![banks];
    let mut i = 0;
    let j = loop {
        i += 1;
        let banks = redistribute(all_banks.last().unwrap());
        if let Some(j) = all_banks.iter().position(|b| *b == banks) {
            break i - j;
        }
        all_banks.push(banks);
    };
    j.to_string()
}
