use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;

#[derive(Debug)]
pub struct Program {
    name: String,
    weight: usize,
    children: Vec<String>,
}

pub type ProgramTower = HashMap<String, Program>; 

/// Creates a hash-map describing each node of the tree.
fn parse_tower(s: &str) -> ProgramTower {
    let mut map = ProgramTower::new();
    let re = match Regex::new(r"(?P<name>\w+) \((?P<weight>\d+)\)(?: -> (?P<children>.*))?") {
        Ok(re) => re,
        Err(e) => {panic!(e);}
    };

    for program in s.trim().split("\n") {
        let caps = match re.captures(program.trim()) {
            Some(caps) => caps,
            None => {continue;}
        };
        let name =  match caps.name("name") {
            Some(m) => m.as_str(),
            None => "NAME NOT FOUND"
        };
        let weight = match caps.name("weight") {
            Some(m) => m.as_str().parse().unwrap_or(0),
            None => 0
        };
        let children = match caps.name("children") {
            None => vec![],
            Some(children) => {
                children.as_str().split(", ").map(|c| c.to_string()).collect()
            }
        };
        let p = Program {name: name.to_string(), weight, children};
        map.insert(name.to_string(), p);
    }
    map
}

fn root(tower: &ProgramTower) -> String {
    let keys: HashSet<String> = tower.keys().map(|k| k.to_string()).collect();
    let children: HashSet<String> = tower.values().flat_map(|node| &node.children).map(|child| child.to_string()).collect();
    let roots: Vec<String> = keys.difference(&children).map(|key| key.to_string()).collect();
    roots[0].to_string()
}

/// # Examples
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
    root(&parse_tower(s))
}

pub type ProgramWeights = HashMap<String, usize>;

fn compute_weight(program: &str, tower: &ProgramTower, weights: &mut ProgramWeights) -> usize {
    if let Some(w) = weights.get(program) {
        return *w;
    }

    let mut w = tower[program].weight;
    for child in tower[program].children.iter() {
        w += compute_weight(child.as_str(), tower, weights);
    }

    weights.insert(program.to_string(), w);
    w
}

fn adjustment_required(program: &Program, tower: &ProgramTower, weights: &ProgramWeights) -> Option<(String, usize)> {
        if program.children.len() == 0 {
            // No children
            return None;
        }

        let children_weights: HashMap<usize, Vec<&str>> = program.children.iter()
        .fold(HashMap::new(), |mut map, c| {
            let w = weights[c];
            if let Some(mut v) = map.remove(&w) {
                v.push(c);
                map.insert(w, v);
            } else {
                map.insert(w, vec![c]);
            };
            map
        });
        if children_weights.len() == 1 {
            // All children have the same weight
            return None;
        }

        let (current_weight, culprits) = children_weights.iter().find(|&(_, v)| v.len() == 1).unwrap();
        let culprit = &tower[culprits[0]];
        let (desired_weight, _) = children_weights.iter().find(|&(_, v)| v.len() > 1).unwrap();
        let new_weight = culprit.weight + desired_weight - current_weight;

        match adjustment_required(&culprit, &tower, &weights) {
            Some((child, adjustment)) => Some((child, adjustment)),
            None => Some((culprit.name.to_string(), new_weight))
        }
}

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
    let tower = parse_tower(s);
    let root = root(&tower);
    let mut weights = ProgramWeights::new();
    compute_weight(&root, &tower, &mut weights);

    let (_, new_weight) = adjustment_required(&tower[&root], &tower, &weights).unwrap();

    new_weight.to_string()
}