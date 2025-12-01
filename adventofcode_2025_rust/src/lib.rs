// expose days code for benchmarking purpose

#[cfg(feature = "day01")]
pub mod day01;
#[cfg(feature = "day02")]
pub mod day02;
#[cfg(feature = "day03")]
pub mod day03;
#[cfg(feature = "day04")]
pub mod day04;
#[cfg(feature = "day05")]
pub mod day05;
#[cfg(feature = "day06")]
pub mod day06;
#[cfg(feature = "day07")]
pub mod day07;
#[cfg(feature = "day08")]
pub mod day08;
#[cfg(feature = "day09")]
pub mod day09;
#[cfg(feature = "day10")]
pub mod day10;
#[cfg(feature = "day11")]
pub mod day11;
#[cfg(feature = "day12")]
pub mod day12;

pub fn get_lines_from(filepath: &str) -> Vec<String> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let file = File::open(filepath).unwrap();

    let mut result = vec![];
    for line in BufReader::new(file).lines().map_while(Result::ok) {
        result.push(line);
    }

    result
}
