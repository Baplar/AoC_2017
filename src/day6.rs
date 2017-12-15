#[derive(PartialEq, Clone)]
pub struct MemoryBank {
    pub bank: Vec<usize>,
}

impl MemoryBank {
    pub fn new(s: &str) -> Self {
        Self {
            bank: s.split_whitespace()
                .filter_map(|w| w.parse().ok())
                .collect(),
        }
    }

    /// Redistributes the blocks of the bank with the most blocks
    ///
    /// # Examples
    /// ```
    /// use advent_of_code::day6::MemoryBank;
    /// let b = MemoryBank::new("0 2 7 0");
    /// let b = b.redistribute();
    /// assert_eq!(b.bank, [2, 4, 1, 2]);
    /// let b = b.redistribute();
    /// assert_eq!(b.bank, [3, 1, 2, 3]);
    /// let b = b.redistribute();
    /// assert_eq!(b.bank, [0, 2, 3, 4]);
    /// let b = b.redistribute();
    /// assert_eq!(b.bank, [1, 3, 4, 1]);
    /// let b = b.redistribute();
    /// assert_eq!(b.bank, [2, 4, 1, 2]);
    /// ```
    pub fn redistribute(&self) -> Self {
        let n = self.bank.len();
        let (i_max, max) = self.bank
            .iter()
            .enumerate()
            .fold(
                (0, 0),
                |(i_max, max), (i, &val)| if val > max { (i, val) } else { (i_max, max) },
            );
        let q = max / n;
        let r = max % n;
        let mut new_bank = self.clone();
        new_bank.bank[i_max] = 0;
        for i in 0..n {
            new_bank.bank[i] += q;
            if (n + i - (i_max + 1)) % n < r {
                new_bank.bank[i] += 1;
            }
        }
        new_bank
    }
}

/// Finds the number of iterations before looping
///
/// # Examples
/// ```
/// use advent_of_code::day6::one;
/// assert_eq!("5", one("0 2 7 0"));
/// ```
pub fn one(s: &str) -> String {
    let mut bank = MemoryBank::new(s);
    let mut all_banks = vec![];
    let mut i = 0;
    loop {
        if all_banks.iter().any(|b| *b == bank) {
            break;
        }
        let new_bank = bank.redistribute();
        all_banks.push(bank);
        bank = new_bank;
        i += 1;
    }
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
    let mut bank = MemoryBank::new(s);
    let mut all_banks = vec![];
    let mut i = 0;
    let j = loop {
        if let Some(j) = all_banks.iter().position(|b| *b == bank) {
            break i - j;
        }
        let new_bank = bank.redistribute();
        all_banks.push(bank);
        bank = new_bank;
        i += 1;
    };
    j.to_string()
}
