use self::Cell::*;
use std::collections::HashMap;
use std::ops::Add;

/// A point on the grid
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

/// A walkable cell of the grid
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Cell {
    Ver,
    Hor,
    Cross,
    Letter(char),
}

/// Parses the grid description character
/// into the corresponding enum variant
fn parse_cell(c: char) -> Option<Cell> {
    match c {
        '|' => Some(Ver),
        '-' => Some(Hor),
        '+' => Some(Cross),
        x if x.is_alphabetic() => Some(Letter(x)),
        _ => None,
    }
}

/// Parses the grid description file
/// into a complete grid
fn parse_grid(s: &str) -> HashMap<Point, Cell> {
    let mut grid = HashMap::new();
    for (j, line) in s.split('\n').enumerate() {
        for (i, c) in line.chars().enumerate() {
            if let Some(cell) = parse_cell(c) {
                grid.insert(Point::new(i as isize, j as isize), cell);
            }
        }
    }
    grid
}

/// A packet on a grid, with its current position
/// and direction of movement
pub struct Packet<'a> {
    pos: Point,
    dir: Point,
    grid: &'a HashMap<Point, Cell>,
}

impl<'a> Packet<'a> {
    /// Initialises the packet on the grid
    fn new(grid: &'a HashMap<Point, Cell>) -> Packet {
        let &point = grid.keys()
            .find(|&p| p.y == 0)
            .expect("No entry point on row 0 of the grid");
        Packet {
            pos: point + Point::new(0, -1),
            dir: Point::new(0, 1),
            grid,
        }
    }

    /// Finds the new direction to take
    /// when on a crossing
    fn turn(&mut self) {
        let p = self.dir;

        let (d1, d2, dir_cell) = if p.x == 0 {
            (Point::new(1, 0), Point::new(-1, 0), Hor)
        } else {
            (Point::new(0, 1), Point::new(0, -1), Ver)
        };

        let (p1, p2) = (self.pos + d1, self.pos + d2);

        if let Some(&cell) = self.grid.get(&p1) {
            if cell == dir_cell {
                self.dir = d1;
            }
        } else if let Some(&cell) = self.grid.get(&p2) {
            if cell == dir_cell {
                self.dir = d2;
            }
        }
    }
}

impl<'a> Iterator for Packet<'a> {
    type Item = Option<char>;

    /// Tries to continue walking in the same direction.
    ///
    /// - If it encounters nothing, it stops.
    /// - If it encounters a letter, it returns it.
    /// - If it encounters a crossing, it tries to turn.
    /// - Else, it goes ahead and returns Some(None).
    fn next(&mut self) -> Option<Self::Item> {
        self.pos = self.pos + self.dir;
        if let Some(cell) = self.grid.get(&self.pos) {
            match *cell {
                Letter(l) => return Some(Some(l)),
                Cross => self.turn(),
                _ => {}
            };
            Some(None)
        } else {
            None
        }
    }
}

/// Recovers the letters found by the packet on its path.
pub fn one(s: &str) -> String {
    let grid = parse_grid(s);
    let p = Packet::new(&grid);

    p.into_iter().filter_map(|c| c).collect()
}

/// Counts the number of steps to take to reach the end of the path.
pub fn two(s: &str) -> String {
    let grid = parse_grid(s);
    let p = Packet::new(&grid);

    p.into_iter().count().to_string()
}
