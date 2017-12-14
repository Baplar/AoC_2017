use super::day10::knot_hash;
use std::collections::HashSet;

/// Converts a byte to a vector of its bits
fn to_bits(x: u8) -> Vec<bool> {
    [128, 64, 32, 16, 8, 4, 2, 1]
        .into_iter()
        .map(|b| x & b > 0)
        .collect()
}

/// Converts a vector of bytes
/// into a vector of 4 times as many bits
fn to_row(v: Vec<u8>) -> Vec<u8> {
    v.into_iter()
        .flat_map(to_bits)
        .enumerate()
        .filter_map(|(i, b)| if b { Some(i as u8) } else { None })
        .collect()
}

/// Creates the disk grid from the key.
///
/// # Examples
/// ```
/// use advent_of_code::day14::grid;
/// let g = grid("flqrgnkx");
/// let used = [(0,0),(1,0),(3,0),(1,1),(3,1),(0,3),(2,3)];
/// assert!(used.into_iter().all(|c| g.get(&c).is_some()));
/// let free = [(2,0),(0,1),(2,1),(0,2),(1,2),(2,2),(3,2),(1,3),(3,3)];
/// assert!(free.into_iter().all(|c| g.get(&c).is_none()));
/// ```
pub fn grid(key: &str) -> HashSet<(u8, u8)> {
    (0..128)
        .map(|j| (j as u8, format!("{}-{}", key, j)))
        .map(|(j, s)| (j, knot_hash(s.as_str())))
        .map(|(j, h)| (j, to_row(h)))
        .flat_map(|(j, r)| {
            r.into_iter().map(|i| (i, j)).collect::<Vec<(u8, u8)>>()
        })
        .collect()
}

/// Counts the number of used cells in the grid
pub fn one(key: &str) -> String {
    grid(key).len().to_string()
}

/// Returns the neighbors of the cell
fn neighbors(cell: &(u8, u8)) -> HashSet<(u8, u8)> {
    let mut set = HashSet::new();
    let &(x, y) = cell;
    if x > 0 {
        set.insert((x - 1, y));
    }
    if x < 127 {
        set.insert((x + 1, y));
    }
    if y > 0 {
        set.insert((x, y - 1));
    }
    if y < 127 {
        set.insert((x, y + 1));
    }
    set
}

/// Forms a group with all the cells connected to the root
fn reduce_group(root: (u8, u8), grid: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)> {
    let mut group = HashSet::new();
    let mut old = HashSet::new();
    old.insert(root);
    while !old.is_empty() {
        group = group.union(&old).map(|&x| x).collect();
        old = old.into_iter()
            .flat_map(|c| neighbors(&c))
            .filter(|c| grid.get(c).is_some())
            .collect::<HashSet<(u8, u8)>>()
            .difference(&group)
            .map(|&x| x)
            .collect();
    }
    group
}

/// Counts the contiguous groups in the drive
pub fn two(s: &str) -> String {
    let mut grid = grid(s);
    let mut nb_groups = 0;
    while !grid.is_empty() {
        let &root = grid.iter().next().unwrap();
        let group = reduce_group(root, &grid);
        for x in group {
            grid.remove(&x);
        }
        nb_groups += 1;
    }
    nb_groups.to_string()
}
