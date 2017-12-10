/// Creates an initial list, with increasing values
pub fn new_list() -> Vec<u8> {
    let mut list = vec![];
    for i in 0..256 {
        list.push(i as u8);
    }
    list
}

/// Ties a knot in the list
/// 
/// # Examples
/// ```
/// use advent_of_code::day10::{new_list, knot};
/// let mut list = new_list();
/// knot(&mut list, 10, 10);
/// let slice = [19, 18, 17, 16, 15, 14, 13, 12, 11, 10];
/// assert_eq!(list[10..20], slice);
/// ```
pub fn knot(list: &mut Vec<u8>, position: usize, length: usize) {    
    for i in 0..length/2 {
        let n1 = (position + i) % list.len();
        let n2 = (position + length - 1 - i) % list.len();
        list.swap(n1, n2);
    }
}

/// Creates a new list and hashes it with the provided lengths
/// 
/// # Examples
/// ```
/// use advent_of_code::day10::hash;
/// let (list, last_pos, last_skip) = hash(&vec![3, 3, 3]);
/// let slice = [2, 1, 0, 5, 4, 3, 6, 9, 8, 7];
/// assert_eq!(list[0..10], slice);
/// assert_eq!(12, last_pos);
/// assert_eq!(3, last_skip);
/// ```
pub fn hash(lengths: &Vec<usize>) -> (Vec<u8>, usize, usize) {
    let mut list = new_list();
    let mut position = 0;
    let mut skip = 0;
    for &length in lengths {
        knot(&mut list, position, length);
        position = (position + length + skip) % list.len();
        skip = (skip + 1) % list.len();
    }
    (list, position, skip)
}

/// Parses a list of lengths
/// 
/// # Examples
/// ```
/// use advent_of_code::day10::parse_lengths;
/// let s = "3, 4, 1, 5";
/// let v = vec![3, 4, 1, 5];
/// assert_eq!(v, parse_lengths(s));
/// ```
pub fn parse_lengths(s: &str) -> Vec<usize> {
    s.trim()
        .split(",")
        .filter_map(|s| s.trim().parse().ok())
        .collect()
}

/// Calculates the product of the two first numbers
/// of the hash generated with the provided lengths
pub fn one(s: &str) -> String {
    let lengths = parse_lengths(s);
    let (list, _, _) = hash(&lengths);
    (list[0] * list[1]).to_string()
}

/// Parses a list of ascii characters into lengths
/// 
/// # Examples
/// ```
/// use advent_of_code::day10::parse_ascii;
/// let s = "1,2,3";
/// let v = vec![49,44,50,44,51];
/// assert_eq!(v, parse_ascii(s));
/// ```
pub fn parse_ascii(s: &str) -> Vec<usize> {
    s.trim()
        .as_bytes()
        .iter()
        .map(|&l| l as usize)
        .collect()
}

/// A hasher structure that keeps an internal state
/// to memorise its iterations
pub struct Hasher {
    list: Vec<u8>,
    position: usize,
    skip: usize,
    lengths: Vec<usize>
}

impl Hasher {
    /// Passes a round of hashing on its internal state
    pub fn hash(&mut self) {
        for &length in self.lengths.iter() {
            knot(&mut self.list, self.position, length);
            self.position = (self.position + length + self.skip) % self.list.len();
            self.skip = (self.skip + 1) % self.list.len();
        }
    }
}

/// Takes a sparse hash and turns it into a dense hash
/// 
/// # Examples
/// ```
/// use advent_of_code::day10::densify;
/// let sparse = vec![65, 27, 9, 1, 4, 3, 40, 50, 91, 7, 6, 0, 2, 5, 68, 22];
/// assert_eq!(vec![64], densify(sparse));
/// ```
pub fn densify(sparse: Vec<u8>) -> Vec<u8> {
    let mut it = sparse.iter();
    let mut dense_hash = vec![];
    let mut group = 0;
    let mut group_size = 0;
    while let Some(&x) = it.next() {
        group ^= x;
        group_size += 1;
        if group_size == 16 {
            dense_hash.push(group);
            group = 0;
            group_size = 0;
        }
    }
    if group_size > 0 {
        dense_hash.push(group);
    }

    dense_hash
}

/// Implements the complete hashing algorithm
/// 
/// # Examples
/// ```
/// use advent_of_code::day10::two;
/// assert_eq!(two(""), "a2582a3a0e66e6e86e3812dcb672a272");
/// assert_eq!(two("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
/// assert_eq!(two("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
/// assert_eq!(two("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
/// ```
pub fn two(s: &str) -> String {
    let mut lengths = parse_ascii(s);
    lengths.append(&mut vec![17, 31, 73, 47, 23]);

    let mut hasher = Hasher{list: new_list(), position: 0, skip: 0, lengths};
    for _ in 0..64 {
        hasher.hash();
    }
    let dense = densify(hasher.list);

    dense.iter().fold(String::new(), |s, u| s + &format!("{:02x}", u))
}
