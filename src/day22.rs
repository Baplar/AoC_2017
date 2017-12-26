use std::collections::HashMap;
use std::ops::Add;
use self::Dir::{Left, Right, Reverse};
use self::Flag::{Clean, Weakened, Infected, Flagged};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Cell {
    x: isize,
    y: isize
}

#[derive(Clone, Copy, PartialEq)]
enum Flag {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

enum Dir {
    Left,
    Right,
    Reverse,
}

impl Cell {
    fn new(x: isize, y: isize) -> Cell {
        Cell {x, y}
    }

    fn turn(&self, d: Dir) -> Cell {
        let (x, y) = match d {
            Left => {
                match (self.x, self.y) {
                    (0,1) => (1,0),
                    (1,0) => (0,-1),
                    (0,-1) => (-1,0),
                    (-1,0) => (0,1),
                    _ => (0,-1)
                }
            },
            Right => {
                match (self.x, self.y) {
                    (1,0) => (0,1),
                    (0,1) => (-1,0),
                    (-1,0) => (0,-1),
                    (0,-1) => (1,0),
                    _ => (0,-1)
                }
            },
            Reverse => {
                match (self.x, self.y) {
                    (1,0) => (-1,0),
                    (-1,0) => (1,0),
                    (0,1) => (0,-1),
                    (0,-1) => (0,1),
                    _ => (0,-1)
                }
            }
        };
        Cell::new(x, y)
    }
}

impl Add for Cell {
    type Output = Cell;

    fn add(self, other: Self) -> Self::Output {
        Cell {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

struct Grid {
    grid: HashMap<Cell, Flag>,
    carrier: Cell,
    direction: Cell,
    simple: bool,
}

impl Grid {
    fn parse_grid(s: &str, simple: bool) -> Grid {
        let size = s.trim().split('\n').count();
        let grid = s.trim()
            .split('\n')
            .enumerate()
            .flat_map(|(j, row)| {
                row.trim()
                    .chars()
                    .enumerate()
                    .filter_map(move |(i, c)| match c {
                        '#' => Some((Cell::new(i as isize, j as isize), Infected)),
                        _ => None,
                    })
            })
            .collect();
        let pos = ((size - 1) / 2) as isize;
        Grid {
            grid,
            carrier: Cell::new(pos, pos),
            direction: Cell::new(0, -1),
            simple
        }
    }

    fn step(&mut self) -> Flag {
        let flag = *self.grid.entry(self.carrier).or_insert_with(|| Clean);
        match flag {
            Clean => {
                let f = Weakened;
                self.direction = self.direction.turn(Left);
                self.grid.insert(self.carrier, f);
                self.carrier = self.carrier + self.direction;
                return f;
            },
            Weakened => {
                let f = Infected;
                self.grid.insert(self.carrier, f);
                self.carrier = self.carrier + self.direction;
                return f;
            }
            Infected => {
                let f = Flagged;
                self.direction = self.direction.turn(Right);
                self.grid.insert(self.carrier, f);
                self.carrier = self.carrier + self.direction;
                return f;
            },
            Flagged => {
                let f = Clean;
                self.direction = self.direction.turn(Reverse);
                self.grid.insert(self.carrier, f);
                self.carrier = self.carrier + self.direction;
                return f;
            }
        }
    }


    fn simple_step(&mut self) -> Flag {
        let flag = *self.grid.entry(self.carrier).or_insert_with(|| Clean);
        match flag {
            Clean => {
                let f = Infected;
                self.direction = self.direction.turn(Left);
                self.grid.insert(self.carrier, f);
                self.carrier = self.carrier + self.direction;
                return f;
            },
            Infected => {
                let f = Clean;
                self.direction = self.direction.turn(Right);
                self.grid.insert(self.carrier, f);
                self.carrier = self.carrier + self.direction;
                return f;
            },
            x => x
        }
    }
}

impl Iterator for Grid {
    type Item = Flag;

    fn next(&mut self) -> Option<Self::Item> {
        let x = if self.simple {
            self.simple_step()
        } else {
            self.step()
        };
        Some(x)
    }
}

/// Counts the number of bursts among the first 10_000
/// that lead to a cell becoming infected.
/// 
/// # Examples
/// ```
/// use advent_of_code::day22::one;
/// let s = "\
/// ..#
/// #..
/// ...";
/// assert_eq!("5587", one(s));
/// ```
pub fn one(s: &str) -> String {
    Grid::parse_grid(s, true)
        .take(10_000)
        .filter(|x| *x == Infected)
        .count()
        .to_string()
}

/// Counts the number of bursts among the first 10_000_000
/// that lead to a cell becoming infected.
/// 
/// # Examples
/// ```
/// use advent_of_code::day22::two;
/// let s = "\
/// ..#
/// #..
/// ...";
/// assert_eq!("2511944", two(s));
/// ```
pub fn two(s: &str) -> String {
    Grid::parse_grid(s, false)
        .take(10_000_000)
        .filter(|x| *x == Infected)
        .count()
        .to_string()
}
