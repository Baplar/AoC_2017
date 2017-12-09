extern crate advent_of_code;
use advent_of_code::{get_solver, get_input};
use std::env;

fn parse_args() -> Result<(String, fn(&str) -> String), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        return Err(String::from("not enough arguments"));
    }

    let day: u32 = match args[1].parse() {
        Ok(day) => day,
        Err(_) => return Err(String::from("day must be an integer (1 to 25)"))
    };

    let part: u32 = match args[2].parse() {
        Ok(part) => part,
        Err(_) => return Err(String::from("part must be an integer (1 or 2)"))
    };

    let filename = if args.len() > 3 {
        args[3].to_string()
    } else {
        format!("day{}", day)
    };

    let solver = match get_solver(day, part) {
        Some(solver) => solver,
        None => return Err(format!("the function for day {}, part {} has not been implemented yet", day, part))
    };
    Ok((filename, solver))
}

fn main() {
    let usage = "Usage: advent_of_code [day] [part] [input]";

    let (filename, solver) = match parse_args() {
        Ok((filename, solver)) => (filename, solver),
        Err(reason) => {
            eprintln!("{}\n\n{}", reason, usage);
            return;
        }
    };

    let input = match get_input(&filename) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("{}\n\n{}", e, usage);
            return;
        }
    };

    println!("{}", solver(&input));
}