use self::Direction::*;

/// The six possible directions on the grid
enum Direction {
    NE,
    N,
    NW,
    SW,
    S,
    SE
}

impl Direction {
    /// Creates a direction from an integer
    /// (From NE, counterclockwise, cyclic)
    fn dir(x: isize) -> Direction {
        let i = if x >= 0 {
            x % 6
        } else {
            x % 6 + 6
        };
        match i {
            0 => NE,
            1 => N,
            2 => NW,
            3 => SW,
            4 => S,
            5 => SE,
            _ => unreachable!()
        }
    }

    /// Returns the integer value linked to the direction, from 1 to 6
    /// (From NE, counterclockwise, cyclic)
    fn val(&self) -> usize {
        match *self {
            NE => 0,
            N => 1,
            NW => 2,
            SW => 3,
            S => 4,
            SE => 5
        }
    }

    /// Applies an offset to the direction (counter-clockwise)
    /// and returns the resulting new direction
    fn off(&self, n: isize) -> Self {
        Self::dir(self.val() as isize + n)
    }

    /// Opposite direction
    fn opp(&self) -> Self {
        self.off(3)
    }

    /// Complementary direction (2 steps counterclockwise)
    fn comp(&self) -> Self {
        self.off(2)
    }

    /// Next direction (1 step counterclockwise)
    fn next(&self) -> Self {
        self.off(1)
    }

    /// Reverse complementary (2 steps clockwise)
    fn rcomp(&self) -> Self {
        self.off(-2)
    }

    /// Previous (1 step clockwise)
    fn prev(&self) -> Self {
        self.off(-1)
    }
}

/// Stores a simplified path, with three axis coordinates
struct Path {
    path: [isize; 3]
}

impl Path {
    /// New path, at initial hex
    fn new() -> Self {
        Path {path: [0; 3]}
    }

    /// Number of steps to take in the given direction
    fn get(&self, dir: &Direction) -> usize {
        let result = match dir.val() % 6 {
            x if x < 3 => self.path[x],
            x => - self.path[x-3]
        };
        result.max(0) as usize
    }

    /// Execute a number of steps
    fn steps(&mut self, dir: &Direction, n: isize) {
        match dir.val() % 6 {
            x if x < 3 => {self.path[x] += n;},
            x => {self.path[x-3] -= n;}
        };
    }

    /// Execute one step
    #[inline]
    fn step(&mut self, dir: &Direction) {
        self.steps(dir, 1);
    }

    /// Transforms a potentially non-optimal path
    /// into the shortest equivalent one
    fn shorten(&mut self) {
        for i in 0..6 {
            let dir = Direction::dir(i);
            if self.get(&dir) > 0 && self.get(&dir.comp()) > 0 {
                let m = self.get(&dir).min(self.get(&dir.comp())) as isize;
                self.steps(&dir.comp(), -m);
                self.steps(&dir, -m);
                self.steps(&dir.next(), m);
            }
        }
    }

    /// Distance of the complete path
    fn dist(&self) -> usize {
        (0..6)
            .map(|x| Direction::dir(x))
            .map(|dir| self.get(&dir))
            .sum()
    }

    /// Shortest distance to get to the end point of the path
    fn shortest_dist(&mut self) -> usize {
        self.shorten();
        self.dist()
    }

    /// Execute a step and immediately "simplify" it if possible
    fn short_step(&mut self, dir: &Direction) {
        if self.get(&dir.opp()) > 0 {
            self.step(dir);
        } else if self.get(&dir.comp()) > 0 {
            self.steps(&dir.comp(), -1);
            self.step(&dir.next());
        } else if self.get(&dir.rcomp()) > 0 {
            self.steps(&dir.rcomp(), -1);
            self.step(&dir.prev());
        } else {
            self.step(dir);
        }
    }
}

/// Parses a single direction string
fn parse_dir(s: &str) -> Direction {
    match s {
        "ne" => NE,
        "n" => N,
        "nw" => NW,
        "sw" => SW,
        "s" => S,
        "se" => SE,
        _ => {panic!("Unable to parse direction: '{}'", s)}
    }
}

/// Parses a list of directions
fn parse_path(s: &str) -> Vec<Direction> {
    s.trim()
        .split(",")
        .map(|s| s.trim())
        .map(|s| parse_dir(s))
        .collect()
}

/// Finds the minimal number of steps to reach a child
/// that took the provided path.
/// 
/// # Examples
/// ```
/// use advent_of_code::day11::one;
/// assert_eq!("3", one("ne,ne,ne"));
/// assert_eq!("0", one("ne,ne,sw,sw"));
/// assert_eq!("2", one("ne,ne,s,s"));
/// assert_eq!("3", one("se,sw,se,sw,sw"));
/// ``` 
pub fn one(s: &str) -> String {
    let mut path = parse_path(s).iter().fold(Path::new(), |mut path, dir| {path.step(dir); path});
    path.shortest_dist().to_string()
}

/// Finds the maximal distance at which the child ever was.
/// 
/// # Examples
/// ```
/// use advent_of_code::day11::two;
/// assert_eq!("3", two("ne,ne,ne"));
/// assert_eq!("2", two("ne,ne,sw,sw"));
/// assert_eq!("2", two("ne,ne,s,s"));
/// assert_eq!("3", two("sw,sw,sw,ne,se"));
/// ```
pub fn two(s: &str) -> String {
    let dirs = parse_path(s);
    let mut path = Path::new();
    let mut max_dist = 0;
    for dir in dirs.iter() {
        path.short_step(dir);
        max_dist = max_dist.max(path.dist());
    }
    max_dist.to_string()
}
