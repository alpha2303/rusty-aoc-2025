use std::io;
use std::io::Write;

mod aoc25;
mod helpers;

fn get_input(prompt: &str) -> Result<String, io::Error> {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input_buf: String = String::new();
    match io::stdin().read_line(&mut input_buf) {
        Ok(_) => Ok(input_buf.trim().to_string()),
        Err(err) => Err(err),
    }
}

fn execute_solution(day: &usize) -> Option<[String; 2]> {
    match *day {
        1 => Some(aoc25::day1::Day1::get_results()),
        2 => Some(aoc25::day2::Day2::get_results()),
        _ => {
            println!("Solution is not implemented");
            None
        }
    }
}

fn main() {
    let choice: usize = get_input("Enter Day number: ")
        .unwrap_or_else(|_| {
            println!("Error: Could not read input");
            std::process::exit(1);
        })
        .parse::<usize>()
        .unwrap();

    let result_option: Option<[String; 2]> = execute_solution(&choice);

    if let Some(results) = result_option {
        println!("{:?}", results);
    } else {
        println!("Solution is not implemented");
    }
}
