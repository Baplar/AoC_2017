use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;

/// A program in the tower
pub struct Program {
    name: String,
    weight: usize,
    children: Vec<Box<Program>>,
    cumulated_weight: usize,
}

impl Program {
    /// Creates a program without info on its children
    fn new(name: &str, weight: usize) -> Program {
        Program {name: String::from(name), weight, children: vec![], cumulated_weight: 0}
    }
}

/// Stores the intermediate data needed to construct the program tree
type ParserMap = HashMap<String, (Program, Vec<String>)>;

/// Extracts the specified program from the map,
/// and recursively calculates the information relative to its children
fn set_children(name: &str, map_programs: &mut ParserMap) -> Result<Program, String> {
    let (mut p, children) = map_programs.remove(name).ok_or("Not found")?;
    let mut w = p.weight;
    for child in children {
        let child = set_children(&child, map_programs)?;
        w += child.cumulated_weight;
        p.children.push(Box::new(child));
    }
    p.cumulated_weight = w;
    Ok(p)
}

/// Creates a hash-map describing each node of the tree,
/// then returns its root.
fn parse_tower(s: &str) -> Result<Program, String> {
    let re = match Regex::new(r"(?P<name>\w+) \((?P<weight>\d+)\)(?: -> (?P<children>.*))?") {
        Ok(re) => re,
        Err(e) => Err(e.to_string())?
    };

    let mut map_programs: ParserMap = ParserMap::new();

    for program in s.trim().split("\n") {
        let caps = match re.captures(program.trim()) {
            Some(caps) => caps,
            None => {continue;}
        };
        let name =  String::from(match caps.name("name") {
            Some(m) => m.as_str(),
            None => Err(String::from("Program with no name"))?
        });
        let weight = match caps.name("weight") {
            Some(m) => m.as_str().parse().unwrap_or(0),
            None => Err(String::from("Program with no weight"))?
        };
        let children = match caps.name("children") {
            Some(children) => children.as_str().split(", ").map(|c| c.to_string()).collect(),
            None => vec![]
        };
        let p = Program::new(&name, weight);
        map_programs.insert(name, (p, children));
    }

    let with_children: HashSet<String> = map_programs.values().flat_map(|&(_, ref children)| children).map(|k| k.clone()).collect();
    let all_names: HashSet<String> = map_programs.keys().map(|k| k.clone()).collect();
    let without_children: Vec<String> = all_names.difference(&with_children).map(|k| k.clone()).collect();
    if without_children.len() != 1 {
        Err(String::from("The tree has no root"))?
    }

    let root = without_children.get(0).ok_or("Root not found")?;
    let root = set_children(&root, &mut map_programs)?;
    Ok(root)
}

/// Returns the root of the program tower
/// 
///  # Examples
/// ```
/// use advent_of_code::day7::one;
/// let list = "\
/// pbga (66)
/// xhth (57)
/// ebii (61)
/// havc (66)
/// ktlj (57)
/// fwft (72) -> ktlj, cntj, xhth
/// qoyq (66)
/// padx (45) -> pbga, havc, qoyq
/// tknk (41) -> ugml, padx, fwft
/// jptl (61)
/// ugml (68) -> gyxo, ebii, jptl
/// gyxo (61)
/// cntj (57)";
/// assert_eq!("tknk", one(list));
/// ```
pub fn one(s: &str) -> String {
    match parse_tower(s) {
        Ok(p) => p.name.to_string(),
        Err(e) => format!("Parsing error: {}", e)
    }
}

/// Recursively goes down the subtree and finds
/// the program responsible for the unbalance 
fn find_unbalanced(program: &Program) -> Option<(&Program, usize)> {
    if program.children.len() == 0 {
        // No children
        return None;
    }

    let mut children_weights: HashMap<usize, Vec<&Box<Program>>> = HashMap::new();
    for child in program.children.iter() {
        let w = child.cumulated_weight;
        children_weights.entry(w).or_insert(vec![]).push(child);
    }
    if children_weights.len() <= 1 {
        // All children have the same weight
        return None;
    }


    let (current_weight, culprit) = match children_weights.iter().find(|&(_, v)| v.len() == 1) {
        Some((w, v)) => (w, v[0]),
        None => None?
    };
    let (desired_weight, _) = children_weights.iter().find(|&(_, v)| v.len() > 1).unwrap();
    let new_weight = culprit.weight + desired_weight - current_weight;

    match find_unbalanced(&culprit) {
        Some((child, adjustment)) => Some((child, adjustment)),
        None => Some((culprit, new_weight))
    }
}

/// Calculates the new weight to give
/// to the unbalanced program of the tower
/// 
/// # Examples
/// ```
/// use advent_of_code::day7::two;
/// let list = "\
/// pbga (66)
/// xhth (57)
/// ebii (61)
/// havc (66)
/// ktlj (57)
/// fwft (72) -> ktlj, cntj, xhth
/// qoyq (66)
/// padx (45) -> pbga, havc, qoyq
/// tknk (41) -> ugml, padx, fwft
/// jptl (61)
/// ugml (68) -> gyxo, ebii, jptl
/// gyxo (61)
/// cntj (57)";
/// assert_eq!("60", two(list));
/// ```
pub fn two(s: &str) -> String {
    let root = match parse_tower(s) {
        Ok(p) => p,
        Err(e) => {return e.to_string();}
    };

    let (_, new_weight) = find_unbalanced(&root).unwrap();

    new_weight.to_string()
}