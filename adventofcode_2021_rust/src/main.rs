#[cfg(feature = "day01")]
mod day01;
#[cfg(feature = "day02")]
mod day02;
#[cfg(feature = "day22")]
mod day22;
#[cfg(feature = "day03")]
mod day03;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    #[cfg(feature = "day01")]
    day01::exec();

    #[cfg(feature = "day02")]
    day02::exec(get_lines_from("src/day02/input"));

    #[cfg(feature = "day03")]
    day03::exec(get_lines_from("src/day03/input"));

    #[cfg(feature = "day22")]
    day22::exec(get_lines_from("src/day22/input"));
}

fn get_lines_from(filepath: &str) -> Vec<String> {
    let file = File::open(filepath).unwrap();

    let mut result = vec![];
    for line in BufReader::new(file).lines() {
        if let Ok(line) = line {
            result.push(line);
        }
    }

    result
}
