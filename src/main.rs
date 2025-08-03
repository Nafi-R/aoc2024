mod days;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Not enough arguments given! Format: aoc2024 <days>");
        process::exit(1);
    }

    let day_str: &str = &args[1];

    let day = match day_str.parse::<i32>() {
        Ok(d) => d,
        Err(_) => {
            println!("Error bad day given");
            process::exit(1);
        }
    };

    match day {
        1 => days::day1(),
        2 => days::day2(),
        3 => days::day3(),
        _ => {
            println!("Day {day} not found");
            process::exit(1);
        }
    }
}
