extern crate advent_of_code;
use advent_of_code::{get_input, get_solver};
use std::env;

fn parse_args() -> Result<(String, fn(&str) -> String), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        return Err(String::from("not enough arguments"));
    }

    let day: u32 = args[1].parse().or(Err(String::from(
        "day must be an integer (1 to 25)",
    )))?;

    let part: u32 = args[2].parse().or(Err(String::from(
        "part must be an integer (1 or 2)",
    )))?;

    let filename = if args.len() > 3 {
        args[3].to_string()
    } else {
        format!("input/day{}", day)
    };

    let solver = get_solver(day, part).ok_or(format!(
        "the function for day {}, part {} has not been implemented yet",
        day,
        part
    ))?;
    Ok((filename, solver))
}

fn main() {
    let usage = "Usage: advent_of_code (day) (part) [input_file]";

    let (filename, solver) = match parse_args() {
        Ok((filename, solver)) => (filename, solver),
        Err(reason) => {
            eprintln!("{}\n\n{}", reason, usage);
            std::process::exit(1);
        }
    };

    let input = match get_input(&filename) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("{}\n\n{}", e, usage);
            std::process::exit(2);
        }
    };

    println!("{}", solver(&input));
}
