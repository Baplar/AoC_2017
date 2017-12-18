use std::collections::HashMap;
use self::Op::*;
use self::Cmp::*;

/// The arithmetic operator to apply to the target register
enum Op {
    INC, // Increment
    DEC, // Decrement
}

/// The comparison operator to use on the compared register
enum Cmp {
    EQ, // Equality
    NE, // Inequality
    LT, // Strictly lower
    LE, // Lower or equal
    GT, // Strictly greater
    GE, // Greater or equal
}

/// The instruction to execute on the register bank
pub struct Instruction {
    target: String,
    op: Op,
    val: isize,
    compared: String,
    cmp: Cmp,
    cond: isize,
}

/// Parses a complete block of instruction
pub fn parse(s: &str) -> Vec<Instruction> {
    s.trim()
        .split('\n')
        .filter_map(|line| parse_instruction(line).ok())
        .collect()
}

/// Parses a single instruction, checking its validity
pub fn parse_instruction(s: &str) -> Result<Instruction, String> {
    if s.trim().split_whitespace().count() != 7 {
        Err(format!("Malformed instruction: {}", s))?
    }
    let mut tokens = s.trim().split_whitespace();

    let target = String::from(tokens.next().ok_or("Missing target register")?);

    let op = match tokens.next().ok_or("Missing operation")? {
        "inc" => INC,
        "dec" => DEC,
        x => Err(format!("Unknown operation {}", x))?,
    };

    let val = tokens
        .next()
        .ok_or("Missing operation value")?
        .parse()
        .map_err(|e| format!("Could not parse value as int: {}", e))?;

    // "if"
    tokens.next().ok_or("Missing if statement")?;

    let compared = String::from(tokens.next().ok_or("Missing compared register")?);

    let cmp = match tokens.next().ok_or("Missing comparator")? {
        "==" => EQ,
        "!=" => NE,
        "<" => LT,
        "<=" => LE,
        ">" => GT,
        ">=" => GE,
        x => Err(format!("Unknown comparator {}", x))?,
    };

    let cond = tokens
        .next()
        .ok_or("Missing condition")?
        .parse()
        .map_err(|e| format!("Could not parse condition as int: {}", e))?;

    Ok(Instruction {
        target,
        op,
        val,
        compared,
        cmp,
        cond,
    })
}

/// The register bank
pub type Registers = HashMap<String, isize>;

/// Evaluates the instruction on place
/// on the provided mutable register bank
///
/// # Examples
/// ```
/// use advent_of_code::day8::{Registers, parse, eval, max};
/// let input = "\
/// b inc 5 if a > 1
/// a inc 1 if b < 5
/// c dec -10 if a >= 1
/// c inc -20 if c == 10";
/// let v = parse(input);
/// let mut regs = Registers::new();
/// assert_eq!(0, max(&regs));
///
/// eval(&v[0], &mut regs);
/// assert_eq!(0, max(&regs));
///
/// eval(&v[1], &mut regs);
/// assert_eq!(1, regs["a"]);
///
/// eval(&v[2], &mut regs);
/// assert_eq!(10, regs["c"]);
///
/// eval(&v[3], &mut regs);
/// assert_eq!(-10, regs["c"]);
///
/// assert_eq!(1, max(&regs));
/// ```
pub fn eval(i: &Instruction, regs: &mut Registers) {
    let &compared = regs.get(&i.compared).unwrap_or(&0);
    let cmp = match i.cmp {
        EQ => compared == i.cond,
        NE => compared != i.cond,
        GT => compared > i.cond,
        GE => compared >= i.cond,
        LT => compared < i.cond,
        LE => compared <= i.cond,
    };
    if cmp {
        let &edited = regs.get(&i.target).unwrap_or(&0);
        let edited = match i.op {
            INC => edited + i.val,
            DEC => edited - i.val,
        };
        regs.insert(i.target.to_owned(), edited);
    }
}

/// Evaluates all the provided instructions on a blank register bank,
/// and returns the resulting register bank
pub fn eval_all(v: &[Instruction]) -> Registers {
    let mut regs = Registers::new();
    for i in v {
        eval(i, &mut regs);
    }
    regs
}

/// Calculates the maximal value currently stored in the bank
pub fn max(regs: &Registers) -> isize {
    *regs.values().max().unwrap_or(&0)
}

/// Calculates the maximum value stored in the register bank
/// at the end of the program
pub fn one(s: &str) -> String {
    let v = parse(s);
    let regs = eval_all(&v);
    max(&regs).to_string()
}

/// Calculates the maximum value ever stored in the bank
/// during the whole execution of the program
pub fn two(s: &str) -> String {
    let v = parse(s);
    let mut regs = Registers::new();
    let mut m = max(&regs);

    for i in v {
        eval(&i, &mut regs);
        let new_m = max(&regs);
        if new_m > m {
            m = new_m;
        }
    }

    m.to_string()
}
