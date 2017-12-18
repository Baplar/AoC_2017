use std::collections::HashMap;
use std::collections::VecDeque;
use self::Value::*;
use self::Op::*;
use self::ProgramIO::*;

/// Represents the right-hand value of an instruction.
/// It either contains a direct value, as an Integer,
/// or refers to a register's content.
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
pub enum Op {
    Snd(Value),
    Set(char, Value),
    Add(char, Value),
    Mul(char, Value),
    Mod(char, Value),
    Rcv(char),
    Jgz(Value, Value),
}

/// Parses an instruction description into an Op enum.
pub fn parse_op(s: &str) -> Option<Op> {
    let mut it = s.trim().split_whitespace();
    match it.next()? {
        "snd" => {
            let v = parse_val(it.next()?)?;
            Some(Snd(v))
        }
        "set" => {
            let c = it.next()?.chars().next()?;
            let v = parse_val(it.next()?)?;
            Some(Set(c, v))
        }
        "add" => {
            let c = it.next()?.chars().next()?;
            let v = parse_val(it.next()?)?;
            Some(Add(c, v))
        }
        "mul" => {
            let c = it.next()?.chars().next()?;
            let v = parse_val(it.next()?)?;
            Some(Mul(c, v))
        }
        "mod" => {
            let c = it.next()?.chars().next()?;
            let v = parse_val(it.next()?)?;
            Some(Mod(c, v))
        }
        "rcv" => {
            let c = it.next()?.chars().next()?;
            Some(Rcv(c))
        }
        "jgz" => {
            let v1 = parse_val(it.next()?)?;
            let v2 = parse_val(it.next()?)?;
            Some(Jgz(v1, v2))
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
    buffer: VecDeque<isize>,
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
            buffer: VecDeque::new(),
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
    fn exec(&mut self) -> Option<ProgramIO> {
        let op = match self.instructions.get(self.pos) {
            Some(op) => op,
            None => return Some(Terminate),
        };
        let mut next_address = self.pos + 1;
        match *op {
            Snd(ref v) => {
                let sent = self.get(v);
                self.pos = next_address;
                return Some(Sent(sent));
            }
            Set(c, ref v) => {
                let val = self.get(v);
                self.regs.insert(c, val);
            }
            Add(c, ref v) => {
                *self.regs.entry(c).or_insert(0) += self.get(v);
            }
            Mul(c, ref v) => {
                *self.regs.entry(c).or_insert(0) *= self.get(v);
            }
            Mod(c, ref v) => {
                *self.regs.entry(c).or_insert(0) %= self.get(v);
            }
            Rcv(c) => {
                if let Some(val) = self.buffer.pop_front() {
                    self.regs.insert(c, val);
                } else {
                    return Some(Receive);
                }
            }
            Jgz(ref v, ref j) => {
                if self.get(v) > 0 {
                    next_address = (self.pos as isize + self.get(j)) as usize;
                }
            }
        };
        self.pos = next_address;
        None
    }
}

impl Iterator for Program {
    type Item = ProgramIO;
    /// Executes the instructions until the program
    /// either terminates, sends a value,
    /// or waits for its buffer to be filled.
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.exec() {
                Some(Terminate) => return None,
                Some(x) => return Some(x),
                None => continue,
            }
        }
    }
}

/// Launch a single program of id 0,
/// and inspect the last value it sent before reaching a deadlock.
pub fn one(s: &str) -> String {
    let instructions = parse(s);
    let program = Program::new(0, &instructions);
    let mut sent = VecDeque::new();
    for result in program {
        match result {
            Sent(i) => sent.push_back(i),
            Receive => return sent.pop_back().unwrap_or(0).to_string(),
            Terminate => break,
        }
    }
    String::from("ERROR: Program terminated before deadlock")
}

/// Launches two programs of id 0 and 1,
/// and counts the number of values sent by program 1
/// before a deadlock is reached.
pub fn two(s: &str) -> String {
    let instructions = parse(s);
    let mut program0 = Program::new(0, &instructions);
    let mut program1 = Program::new(1, &instructions);
    let mut nb_sent = 0;

    let mut done = false;
    while !done {
        done = true;
        match program0.next() {
            Some(Sent(i)) => {
                program1.buffer.push_back(i);
                done = false;
            }
            Some(_) | None => {}
        };
        match program1.next() {
            Some(Sent(i)) => {
                nb_sent += 1;
                program0.buffer.push_back(i);
                done = false;
            }
            Some(_) | None => {}
        };
    }

    nb_sent.to_string()
}
