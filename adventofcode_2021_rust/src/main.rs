#[cfg(feature = "day1")]
mod day1;
#[cfg(feature = "day2")]
mod day2;
#[cfg(feature = "day22")]
mod day22;
#[cfg(feature = "day3")]
mod day3;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    #[cfg(feature = "day1")]
    day1::exec();

    #[cfg(feature = "day2")]
    day2::exec(get_lines_from("src/day2/input"));

    #[cfg(feature = "day3")]
    day3::exec(get_lines_from("src/day3/input"));

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
