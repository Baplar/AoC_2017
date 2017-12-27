use regex::{Error, Regex};

/// A conversion rule, mapping all the symmetries of the original pattern
/// to the corresponding resulting pattern.
struct Rule {
    size: usize,
    patterns: Vec<Vec<bool>>,
    result: Vec<bool>,
}

impl Rule {
    /// Generates all the symmetries once and for all
    fn new(pattern: &[Vec<bool>], result: Vec<bool>) -> Self {
        let size = pattern.len();
        let mut patterns = vec![];

        // Correct order
        patterns.push(pattern.iter().cloned().flat_map(|x| x).collect());

        // Horizontal flip
        patterns.push(
            pattern
                .iter()
                .cloned()
                .flat_map(|x| x.into_iter().rev())
                .collect(),
        );

        // Vertical flip
        patterns.push(pattern.iter().cloned().rev().flat_map(|x| x).collect());

        // Horizontal + vertical flip (= 180° rotation)
        patterns.push(
            pattern
                .iter()
                .cloned()
                .rev()
                .flat_map(|x| x.into_iter().rev())
                .collect(),
        );

        // Transposed matrix, i.e. 90° rotation + flip
        let transposed: Vec<Vec<bool>> = (0..size)
            .map(|i| pattern.iter().map(|v| v[i]).collect())
            .collect();
        // Then we repeat the all flips on the transposed matrix

        patterns.push(transposed.iter().cloned().flat_map(|x| x).collect());

        patterns.push(
            transposed
                .iter()
                .cloned()
                .flat_map(|x| x.into_iter().rev())
                .collect(),
        );

        patterns.push(transposed.iter().cloned().rev().flat_map(|x| x).collect());

        patterns.push(
            transposed
                .iter()
                .cloned()
                .rev()
                .flat_map(|x| x.into_iter().rev())
                .collect(),
        );

        Rule {
            size,
            patterns,
            result,
        }
    }

    // Returns true if the rule corresponds to the square
    fn match_rule(&self, square: &[bool]) -> bool {
        if self.size.pow(2) != square.len() {
            return false;
        }
        self.patterns.iter().any(|p| *p == square)
    }
}

struct Parser {
    re: Regex,
}

impl Parser {
    fn new() -> Result<Self, Error> {
        let re = Regex::new(r"^(.+) => (.+)$")?;
        Ok(Parser { re })
    }

    /// Parses a rule definition string
    fn parse_rule(&self, s: &str) -> Result<Rule, String> {
        let caps = self.re.captures(s.trim()).ok_or("Not a rule definition")?;

        let pattern: Vec<Vec<bool>> = caps[1]
            .split('/')
            .map(|s| s.chars().map(|c| c == '#').collect())
            .collect();

        let result: Vec<bool> = caps[2]
            .split('/')
            .flat_map(|s| s.chars().map(|c| c == '#').collect::<Vec<bool>>())
            .collect();

        Ok(Rule::new(&pattern, result))
    }

    /// Parses a list of rule definitions
    fn parse(&self, s: &str) -> Vec<Rule> {
        s.trim()
            .split('\n')
            .filter_map(|s| self.parse_rule(s.trim()).ok())
            .collect()
    }
}

/// Splits a fractal (2D matrix stored in a vector)
/// into a vector of squares to be mapped to enhancement rules
fn split(fractal: &[bool]) -> Vec<Vec<bool>> {
    let size = (fractal.len() as f64).sqrt() as usize;
    let size_cut = if size % 2 == 0 { 2 } else { 3 };
    let cuts = size / size_cut;

    let mut result = vec![];
    for y in 0..cuts {
        for x in 0..cuts {
            let mut v = vec![];
            for j in 0..size_cut {
                for i in 0..size_cut {
                    v.push(fractal[(size_cut * y + j) * size + (size_cut * x + i)]);
                }
            }
            result.push(v);
        }
    }

    result
}

/// Merges a vector of squares into a new fractal,
/// once the enhancements have been applied
fn merge(squares: Vec<Vec<bool>>) -> Vec<bool> {
    let cuts = (squares.len() as f64).sqrt() as usize;
    let size_cut = (squares[0].len() as f64).sqrt() as usize;
    let size = size_cut * cuts;

    let mut result = vec![];
    for _ in 0..size * size {
        result.push(false);
    }

    for (n, square) in squares.into_iter().enumerate() {
        let x = n % cuts;
        let y = n / cuts;
        for j in 0..size_cut {
            for i in 0..size_cut {
                result[(size_cut * y + j) * size + (size_cut * x + i)] = square[size_cut * j + i];
            }
        }
    }

    result
}

/// Iterates a step of the enhancement process
/// on the provided fractal, and with the provided rules.
fn iterate(rules: &[Rule], fractal: &[bool]) -> Vec<bool> {
    let squares = split(fractal);
    let replaced = squares
        .into_iter()
        .map(|s| {
            rules
                .into_iter()
                .find(|r| r.match_rule(&s))
                .expect(&format!("No rule matches this cell pattern:\n{:?}", s))
                .result
                .clone()
        })
        .collect();

    merge(replaced)
}

/// Counts the number of "on" pixels in the fractal
/// after 5 iterations
pub fn one(s: &str) -> String {
    let parser = if let Ok(parser) = Parser::new() {
        parser
    } else {
        return String::from("Could not create parser");
    };
    let rules = parser.parse(s);

    let mut fractal = vec![false, true, false, false, false, true, true, true, true];

    for _ in 0..5 {
        fractal = iterate(&rules, &fractal);
    }

    fractal.into_iter().filter(|&x| x).count().to_string()
}

/// Counts the number of "on" pixels in the fractal
/// after 18 iterations
pub fn two(s: &str) -> String {
    let parser = if let Ok(parser) = Parser::new() {
        parser
    } else {
        return String::from("Could not create parser");
    };
    let rules = parser.parse(s);

    let mut fractal = vec![false, true, false, false, false, true, true, true, true];

    for _ in 0..18 {
        fractal = iterate(&rules, &fractal);
    }

    fractal.into_iter().filter(|&x| x).count().to_string()
}
