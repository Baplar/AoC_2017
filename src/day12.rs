use std::collections::HashSet;
use std::collections::HashMap;
use regex::{Regex, Error};

struct Parser {
    re: Regex
}

impl Parser {
    fn new() -> Result<Self, Error> {
        let re = match Regex::new(r"(\d+) <-> (\d+(?:, \d+)*)") {
            Ok(re) => re,
            Err(e) => Err(e)?
        };
        Ok(Parser {re})
    }

    /// Parses a pipe definition string
    fn parse_pipe(&self, s: &str) -> Result<(usize, HashSet<usize>), String> {
        let caps = match self.re.captures(s.trim()) {
            Some(caps) => caps,
            None => Err("Not a pipe definition")?
        };

        let first: usize = match caps[1].parse() {
            Ok(first) => first,
            Err(e) => Err(e.to_string())?
        };

        let neighbors: HashSet<usize> = caps[2]
            .split(", ")
            .filter_map(|n| n.parse().ok())
            .collect();

        Ok((first, neighbors))
    }

    /// Parses a list of pipe definitions
    /// /!\ The list must be ordered by root and complete
    fn parse_neighbors(&self, s: &str) -> HashMap<usize, HashSet<usize>> {
        let v: HashMap<usize, HashSet<usize>> = s.trim()
            .split("\n")
            .filter_map(|s| self.parse_pipe(s.trim()).ok())
            .collect();    
        v
    }
}

/// Calculates the group of nodes that are connected,
/// directly or indirectly, to the root
fn reduce_group(pipes: &HashMap<usize, HashSet<usize>>, root: usize) -> HashSet<usize> {
    let mut group = HashSet::new();
    let mut old = HashSet::new();
    old.insert(root);
    while old.len() > 0 {
        group = group.union(&old).map(|&x| x).collect();
        old = old.iter()
            .fold(HashSet::new(), |new, x| new.union(&pipes[x]).map(|&x| x).collect())
            .difference(&group)
            .map(|&x| x)
            .collect();
    }
    ;group
}

/// Calculates the number of nodes connected to 0
/// in the provided list of pipes
/// 
/// # Examples
/// ```
/// use advent_of_code::day12::one;
/// let pipes = "\
/// 0 <-> 2
/// 1 <-> 1
/// 2 <-> 0, 3, 4
/// 3 <-> 2, 4
/// 4 <-> 2, 3, 6
/// 5 <-> 6
/// 6 <-> 4, 5";
/// assert_eq!("6", one(pipes));
/// ```
pub fn one(s: &str) -> String {
    let parser = Parser::new().unwrap();
    let pipes = parser.parse_neighbors(s);
    let group = reduce_group(&pipes, 0);
    group.len().to_string()
}

/// Calculates the number of interconnected groups
/// in the provided list of pipes
/// 
/// # Examples
/// ```
/// use advent_of_code::day12::two;
/// let pipes = "\
/// 0 <-> 2
/// 1 <-> 1
/// 2 <-> 0, 3, 4
/// 3 <-> 2, 4
/// 4 <-> 2, 3, 6
/// 5 <-> 6
/// 6 <-> 4, 5";
/// assert_eq!("2", two(pipes));
/// ```
pub fn two(s: &str) -> String {
    let parser = Parser::new().unwrap();
    let mut pipes = parser.parse_neighbors(s);
    let mut groups = Vec::new();
    while pipes.len() > 0 {
        let (&root, _) = pipes.iter().next().unwrap();
        let group = reduce_group(&pipes, root);
        for x in group.iter() {
            pipes.remove(&x);
        }
        groups.push(group);
    }
    groups.len().to_string()
}
