use std::collections::HashSet;
use std::collections::HashMap;
use regex::{Error, Regex};

struct Parser {
    re: Regex,
}

impl Parser {
    fn new() -> Result<Self, Error> {
        let re = Regex::new(r"(\d+) <-> (\d+(?:, \d+)*)")?;
        Ok(Parser { re })
    }

    /// Parses a pipe definition string
    fn parse_pipe(&self, s: &str) -> Result<(usize, HashSet<usize>), String> {
        let caps = self.re.captures(s.trim()).ok_or("Not a pipe definition")?;

        let first: usize = caps[1]
            .parse()
            .map_err(|e| format!("Could not parse first: {}", e))?;

        let neighbors = caps[2].split(", ").filter_map(|n| n.parse().ok()).collect();

        Ok((first, neighbors))
    }

    /// Parses a list of pipe definitions
    /// /!\ The list must be ordered by root and complete
    fn parse_neighbors(&self, s: &str) -> HashMap<usize, HashSet<usize>> {
        s.trim()
            .split('\n')
            .filter_map(|s| self.parse_pipe(s.trim()).ok())
            .collect()
    }
}

/// Calculates the group of nodes that are connected,
/// directly or indirectly, to the root
fn reduce_group(pipes: &HashMap<usize, HashSet<usize>>, root: usize) -> HashSet<usize> {
    let mut group = HashSet::new();
    let mut old = HashSet::new();
    old.insert(root);
    while !old.is_empty() {
        group = group.union(&old).cloned().collect();
        old = old.into_iter()
            .fold(HashSet::new(), |new, x| {
                new.union(&pipes[&x]).cloned().collect()
            })
            .difference(&group)
            .cloned()
            .collect();
    }
    group
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
    let parser = Parser::new().expect("Could not create parser");
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
    let parser = Parser::new().expect("Could not create parser");
    let mut pipes = parser.parse_neighbors(s);
    let mut groups = Vec::new();
    while let Some(&root) = pipes.keys().next() {
        let group = reduce_group(&pipes, root);
        for x in &group {
            pipes.remove(x);
        }
        groups.push(group);
    }
    groups.len().to_string()
}
