use std::collections::HashMap;

/// A cell of the spiral
#[derive(PartialEq, Eq, Hash)]
struct Cell {
    x: i64,
    y: i64,
}

/// Takes in the index of the cell,
/// and returns the coordinates relative to the cell #1 in the spiral.
fn spiral_coord_from_index(index: u64) -> Cell {
    // Let us determine the side length of the smallest square
    // centered around the cell #1 containing the index:
    let square_length = {
        let mut side = 1;
        while index > side*side {side += 2;}
        side - 1 // side elements => length side-1
    };
    
    if square_length < 2 {
        // Center of the spiral
        return Cell {x: 0, y: 0};
    }

    let half_length = (square_length/2) as i64;
    if (square_length + 1) * (square_length + 1) == index {
        // Bottom-right corner
        return Cell {x: half_length, y: -half_length};
    } 

    // We find on which side of the square the cell is,
    // running counter-clockwise from the bottom-right corner.
    let start_index = (square_length - 1) * (square_length - 1);
    let which_side = (index - start_index) / square_length;
    let rem = ((index - start_index) % square_length) as i64;
    let (x, y) = match which_side {
        0 => (half_length, rem - half_length),
        1 => (half_length - rem, half_length),
        2 => (-half_length, half_length - rem),
        3 => (rem - half_length, -half_length),
        _ => unreachable!()
    };
    Cell {x, y}
}

/// Finds the distance from the center of the spiral
/// to the provided index.
/// 
/// # Examples
/// ```
/// use advent_of_code::day3::one;
/// assert_eq!("0", one("1"));
/// assert_eq!("3", one("12"));
/// assert_eq!("2", one("23"));
/// assert_eq!("31", one("1024"));
/// ```
pub fn one(s: &str) -> String {
    let index = s.trim().parse().unwrap_or(1);
    let cell = spiral_coord_from_index(index);
    (cell.x.abs() + cell.y.abs()).to_string()
}

fn neighbors(c1: &Cell, c2: &Cell) -> bool {
    (c1.x - c2.x).abs() <= 1 && (c1.y - c2.y).abs() <= 1
}

fn sum_neighbors(target: &Cell, cells: &HashMap<Cell, u64>) -> u64 {
    cells.iter()
        .filter(|&(c, _)| neighbors(target, c))
        .map(|(_, val)| val)
        .sum()
}

/// Finds the first element of the "cumulative" spiral
/// which is larger than the provided value
/// 
/// # Examples
/// ```
/// use advent_of_code::day3::two;
/// assert_eq!("1", two("1"));
/// assert_eq!("2", two("2"));
/// assert_eq!("4", two("3"));
/// assert_eq!("10", two("6"));
/// ```
pub fn two(s: &str) -> String {
    let objective = s.trim().parse().unwrap_or(1);

    let mut cells = HashMap::new();

    let mut index = 1;
    let mut val = 1;
    cells.insert(Cell {x: 0, y: 0}, val);

    while val < objective {
        index += 1;
        let cell = spiral_coord_from_index(index);
        // We compute the sum of the values of all its neighbors
        val = sum_neighbors(&cell, &cells);
        cells.insert(cell, val);
    }

    val.to_string()
}
