/// Takes in the index of the cell,
/// and returns the coordinates relative to the cell #1 in the spiral.
fn spiral_coord_from_index(index: u64) -> (i64, i64) {
    // Let us determine the side length of the smallest square
    // centered around the cell #1 containing the index:
    let square_length = {
        let mut side = 1;
        while index > side*side {side += 2;}
        side - 1 // side elements => length side-1
    };
    
    if square_length < 2 {
        // Center of the spiral
        return (0, 0);
    }

    let half_length = (square_length/2) as i64;
    if (square_length + 1) * (square_length + 1) == index {
        // Bottom-right corner
        return (half_length, -half_length);
    } 

    // We find on which side of the square the cell is,
    // running counter-clockwise from the bottom-right corner.
    let start_index = (square_length - 1) * (square_length - 1);
    let which_side = (index - start_index) / square_length;
    let rem = ((index - start_index) % square_length) as i64;
    match which_side {
        0 => (half_length, rem - half_length),
        1 => (half_length - rem, half_length),
        2 => (-half_length, half_length - rem),
        3 => (rem - half_length, -half_length),
        _ => unreachable!()
    }
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
    let index: u64 = s.trim().parse().unwrap_or(1);
    let (x, y) = spiral_coord_from_index(index);
    (x.abs() + y.abs()).to_string()
}

fn neighbors(x1: i64, y1: i64, x2: i64, y2: i64) -> bool {
    (x1 - x2).abs() <= 1 && (y1 - y2).abs() <= 1
}

/// An already computed cell to compare to its neighbors
struct SpiralCell {
    x: i64,
    y: i64,
    val: u64,
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
    let objective: u64 = s.trim().parse().unwrap_or(1);

    let mut cells: Vec<SpiralCell> = vec!();

    let mut index = 0;
    let mut val = 0;
    while val < objective {
        index += 1;
        let (x, y) = spiral_coord_from_index(index);
        val = if index == 1 {
            // Central cell initialized with 1
            1
        } else {
            // We compute the sum of the values of all its neighbors
            cells.iter().fold(0, |acc, cell| if neighbors(x, y, cell.x, cell.y) {acc + cell.val} else {acc})
        };
        cells.push(SpiralCell {x, y, val});
    }

    val.to_string()
}
