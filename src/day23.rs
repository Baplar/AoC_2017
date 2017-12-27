use std::collections::HashMap;
use self::Value::{Integer, Register};
use self::Op::{Jnz, Mul, Set, Sub};

/// Represents the right-hand value of an instruction.
/// It either contains a direct value, as an Integer,
/// or refers to a register's content.
#[derive(Clone, Copy, Debug)]
pub enum Value {
    Integer(isize),
    Register(char),
}

/// Parses a value description into a Value enum,
/// depending on its nature.
pub fn parse_val(s: &str) -> Option<Value> {
    if let Ok(i) = s.parse() {
        Some(Integer(i))
    } else if let Some(c) = s.chars().next() {
        Some(Register(c))
    } else {
        None
    }
}

/// Represents a Duet assembly instruction
#[derive(Clone, Copy, Debug)]
pub enum Op {
    Set(char, Value),
    Sub(char, Value),
    Mul(char, Value),
    Jnz(Value, Value),
}

/// Parses an instruction description into an Op enum.
pub fn parse_op(s: &str) -> Option<Op> {
    let mut it = s.trim().split_whitespace();
    match it.next()? {
        "set" => {
            let c = it.next()?.chars().next()?;
            let v = parse_val(it.next()?)?;
            Some(Set(c, v))
        }
        "sub" => {
            let c = it.next()?.chars().next()?;
            let v = parse_val(it.next()?)?;
            Some(Sub(c, v))
        }
        "mul" => {
            let c = it.next()?.chars().next()?;
            let v = parse_val(it.next()?)?;
            Some(Mul(c, v))
        }
        "jnz" => {
            let v1 = parse_val(it.next()?)?;
            let v2 = parse_val(it.next()?)?;
            Some(Jnz(v1, v2))
        }
        _ => None,
    }
}

/// Parses the complete program in Duet assembly
/// into a vector of instructions
pub fn parse(s: &str) -> Vec<Op> {
    s.trim().split('\n').filter_map(|s| parse_op(s)).collect()
}

/// Represents a running program
struct Program {
    regs: HashMap<char, isize>,
    instructions: Vec<Op>,
    pos: usize,
}

/// Represents the external behavior of the program:
/// whether it just sent a value,
/// is waiting to receive one because its buffer is empty,
/// or has left its code space and has terminated.
pub enum ProgramIO {
    Sent(isize),
    Receive,
    Terminate,
}

impl Program {
    /// Create a new program with the corresponding id and code
    fn new(id: isize, instructions: &[Op]) -> Self {
        let mut regs = HashMap::new();
        regs.insert('p', id);
        Program {
            regs,
            instructions: instructions.to_vec(),
            pos: 0,
        }
    }

    /// Retrieve the value of a right-hand member
    /// in the context of the program, i.e. the value of the Integer
    /// or the content of the Register
    fn get(&self, v: &Value) -> isize {
        match *v {
            Integer(i) => i,
            Register(c) => *self.regs.get(&c).unwrap_or(&0),
        }
    }

    /// Executes the current instruction if possible,
    /// and returns the emitted IO state if applicable
    fn exec(&mut self) -> Option<Op> {
        let op = match self.instructions.get(self.pos) {
            Some(op) => *op,
            None => return None,
        };
        let mut next_address = self.pos + 1;
        match op {
            Set(c, ref v) => {
                let val = self.get(v);
                self.regs.insert(c, val);
            }
            Sub(c, ref v) => {
                *self.regs.entry(c).or_insert(0) -= self.get(v);
            }
            Mul(c, ref v) => {
                *self.regs.entry(c).or_insert(0) *= self.get(v);
            }
            Jnz(ref v, ref j) => {
                if self.get(v) != 0 {
                    next_address = (self.pos as isize + self.get(j)) as usize;
                }
            }
        };
        self.pos = next_address;
        Some(op)
    }
}

impl Iterator for Program {
    type Item = Op;
    /// Executes the instructions until the program
    /// either terminates, sends a value,
    /// or waits for its buffer to be filled.
    fn next(&mut self) -> Option<Self::Item> {
        self.exec()
    }
}

/// Launch a single program of id 0,
/// and inspect the number of mul instructions
pub fn one(s: &str) -> String {
    let instructions = parse(s);
    let program = Program::new(0, &instructions);
    program
        .filter(|x| if let Mul(_, _) = *x { true } else { false })
        .count()
        .to_string()
}

/// Launch a single program of id 0,
/// and inspect the value of register h.
pub fn two_brute_force(s: &str) -> String {
    let instructions = parse(s);
    let mut program = Program::new(0, &instructions);
    program.regs.insert('a', 1);
    for i in 0.. {
        match program.next() {
            None => break,
            Some(_) => {}
        };
        if i % 100_000 == 0 {
            eprintln!("{:?}", program.regs)
        }
    }

    program.regs.get(&'h').unwrap_or(&0).to_string()
}

/// Simulates the non-debug program.
pub fn two(_s: &str) -> String {
    let mut b = 100 * 99 + 100_000;
    let c = b + 17_000;
    let mut h = 0;
    while b <= c {
        if (2..b).any(|x| b % x == 0) {
            h += 1;
        }
        b += 17;
    }

    h.to_string()
}
