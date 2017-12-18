use std::collections::HashMap;
use std::hash::Hash;
use self::Move::*;

/// The 16 dancers as they are ordered at the beginning
static INITIAL_DANCERS: &str = "abcdefghijklmnop";

/// Describes a move to dance
pub enum Move<T> {
    Spin(usize),
    Exchange(usize, usize),
    Partner(T, T),
}

/// Trait allowing a collection to simulate dancers
pub trait Dance<T> {
    /// Moves the last n elements at the front
    /// without changing their order
    fn spin(&mut self, n: usize);

    /// Exchanges the dancers at positions a and b
    fn exchange(&mut self, a: usize, b: usize);

    /// Exchanges the dancers named a and b
    fn partner(&mut self, a: &T, b: &T);

    /// Performs a move
    fn perform(&mut self, m: &Move<T>);
}

impl<T> Dance<T> for Vec<T>
where
    T: PartialEq,
{
    /// # Examples
    ///
    /// ```
    /// use advent_of_code::day16::Dance;
    /// let mut v = vec!['a', 'b', 'c', 'd', 'e'];
    /// v.spin(1);
    /// assert_eq!("eabcd", v.iter().collect::<String>());
    /// ```
    fn spin(&mut self, n: usize) {
        let size = self.len();
        let mut end = self.split_off(size - n);
        end.append(self);
        *self = end;
    }

    /// # Examples
    ///
    /// ```
    /// use advent_of_code::day16::Dance;
    /// let mut v = vec!['e', 'a', 'b', 'c', 'd'];
    /// v.swap(3, 4);
    /// assert_eq!("eabdc", v.iter().collect::<String>());
    /// ```
    fn exchange(&mut self, a: usize, b: usize) {
        self.swap(a, b);
    }

    /// # Examples
    ///
    /// ```
    /// use advent_of_code::day16::Dance;
    /// let mut v = vec!['e', 'a', 'b', 'd', 'c'];
    /// v.partner(&'e', &'b');
    /// assert_eq!("baedc", v.iter().collect::<String>());
    /// ```
    fn partner(&mut self, a: &T, b: &T) {
        let pos_a = self.iter().position(|x| *x == *a).unwrap_or(0);
        let pos_b = self.iter().position(|x| *x == *b).unwrap_or(0);
        self.swap(pos_a, pos_b);
    }

    fn perform(&mut self, m: &Move<T>) {
        match *m {
            Spin(n) => self.spin(n),
            Exchange(a, b) => self.exchange(a, b),
            Partner(ref a, ref b) => self.partner(a, b),
        }
    }
}

/// Parses a single move
fn parse_move(s: &str) -> Result<Move<char>, String> {
    let mut it = s.chars();
    if let Some(t) = it.next() {
        match t {
            's' => {
                let n = it.collect::<String>()
                    .parse()
                    .map_err(|_| String::from("Wrong parameter for spin"))?;
                return Ok(Spin(n));
            }
            'x' => {
                let v: Vec<usize> = it.collect::<String>()
                    .split('/')
                    .filter_map(|x| x.parse().ok())
                    .collect();
                if v.len() != 2 {
                    return Err(String::from("Wrong parameters for exchange"));
                }
                return Ok(Exchange(v[0], v[1]));
            }
            'p' => {
                let v: Vec<char> = it.collect::<String>()
                    .split('/')
                    .filter_map(|s| s.chars().next())
                    .collect();
                if v.len() != 2 {
                    return Err(String::from("Wrong parameters for partner"));
                }
                return Ok(Partner(v[0], v[1]));
            }
            op => {
                return Err(format!("Unknown operation {}", op));
            }
        }
    }

    Err(String::from("Empty string for move"))
}

/// Parses a list of comma-separated moves
fn parse(s: &str) -> Vec<Move<char>> {
    s.trim()
        .split(',')
        .filter_map(|s| parse_move(s).ok())
        .collect()
}

/// Performs a list of moves and returns the final ordering of dancers
pub fn one(s: &str) -> String {
    let moves = parse(s);
    let mut dancers: Vec<char> = INITIAL_DANCERS.chars().collect();
    for m in &moves {
        dancers.perform(m);
    }
    dancers.into_iter().collect()
}

/// Simplifies a sequence of moves into 2 permutations "matrices"
///
/// The first vector corresponds to the position-wise permutations
/// (spin & exchange), while the second hashmap corresponds
/// to the name-wise permutations (partner).
fn reduce(moves: &[Move<char>]) -> (Vec<usize>, HashMap<char, char>) {
    let mut pos: Vec<char> = INITIAL_DANCERS.chars().collect();
    let mut name = pos.clone();
    for m in moves {
        match *m {
            Spin(_) | Exchange(_, _) => pos.perform(m),
            Partner(_, _) => name.perform(m),
        }
    }

    let p = pos.into_iter()
        .filter_map(|a| INITIAL_DANCERS.chars().position(|b| a == b))
        .collect();

    let q = INITIAL_DANCERS.chars().zip(name).collect();

    (p, q)
}

pub trait Permutable<T> {
    /// Permutes by position
    fn permute_pos(&mut self, p: &[usize]);

    /// Permute by name
    fn permute_name(&mut self, q: &HashMap<T, T>);
}

impl<T> Permutable<T> for Vec<T>
where
    T: Eq + Hash + Clone,
{
    fn permute_pos(&mut self, p: &[usize]) {
        *self = p.into_iter()
            .filter_map(|&i| self.get(i))
            .cloned()
            .collect();
    }

    fn permute_name(&mut self, q: &HashMap<T, T>) {
        *self = self.into_iter()
            .filter_map(|x| q.get(x))
            .cloned()
            .collect()
    }
}

/// Repeats a position-wise permutation on itself
///
/// # Examples
/// ```
/// use advent_of_code::day16::repeat_p;
/// let mut p: Vec<usize> = (0..16).collect();
/// p[5] = 12;
/// p[12] = 5;
/// let p2 = repeat_p(&p, 1);
/// assert_eq!(p, p2);
///
/// let p1: Vec<usize> = (0..16).collect();
/// let p2 = repeat_p(&p, 2);
/// assert_eq!(p1, p2);
/// ```
pub fn repeat_p(p: &Vec<usize>, mut n: usize) -> Vec<usize> {
    let mut exponent: Vec<usize> = p.clone();
    let mut new_p: Vec<usize> = (0..16).collect();
    while n > 0 {
        if n & 1 > 0 {
            new_p.permute_pos(&exponent);
        }
        let exp_copy = exponent.clone();
        exponent.permute_pos(&exp_copy);
        n >>= 1;
    }
    new_p
}

/// Permutes a hashmap with another one
fn permute_hashmap(m: &HashMap<char, char>, q: &HashMap<char, char>) -> HashMap<char, char> {
    m.into_iter()
        .map(|(a, b)| (*a, *q.get(b).unwrap_or(a)))
        .collect()
}

/// Repeats a name-wise permutation on itself
///
/// # Examples
/// ```
/// use std::collections::HashMap;
/// use advent_of_code::day16::repeat_q;
/// let mut q: HashMap<char, char> = "abcdefghijklmnop".chars()
///     .zip("abcdefghijklmnop".chars())
///     .collect();
/// q.insert('a', 'b');
/// q.insert('b', 'd');
/// q.insert('c', 'c');
/// q.insert('d', 'a');
/// let q2 = repeat_q(&q, 1);
/// assert_eq!(q, q2);
///
/// let mut q1 = q.clone();
/// q1.insert('a', 'd');
/// q1.insert('b', 'a');
/// q1.insert('d', 'b');
/// let q2 = repeat_q(&q, 2);
/// assert_eq!(q1, q2);
/// ```
pub fn repeat_q(q: &HashMap<char, char>, mut n: usize) -> HashMap<char, char> {
    let mut new_q: HashMap<char, char> = INITIAL_DANCERS
        .chars()
        .zip(INITIAL_DANCERS.chars())
        .collect();
    let mut exponent = q.clone();
    while n > 0 {
        if n & 1 > 0 {
            new_q = permute_hashmap(&new_q, &exponent);
        }
        exponent = permute_hashmap(&exponent, &exponent);
        n >>= 1;
    }
    new_q
}

/// Performs a list of moves 1 billion times in a row
/// and returns the result
pub fn two(s: &str) -> String {
    let moves = parse(s);
    let (p, q) = reduce(&moves);
    let p = repeat_p(&p, 1_000_000_000);
    let q = repeat_q(&q, 1_000_000_000);

    let mut dancers: Vec<char> = INITIAL_DANCERS.chars().collect();
    dancers.permute_pos(&p);
    dancers.permute_name(&q);

    dancers.into_iter().collect()
}
