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
#[cfg(feature = "day13")]
pub mod day13;
#[cfg(feature = "day14")]
pub mod day14;
#[cfg(feature = "day15")]
pub mod day15;
#[cfg(feature = "day16")]
pub mod day16;
#[cfg(feature = "day17")]
pub mod day17;
#[cfg(feature = "day18")]
pub mod day18;
#[cfg(feature = "day19")]
pub mod day19;
#[cfg(feature = "day20")]
pub mod day20;
#[cfg(feature = "day21")]
pub mod day21;
#[cfg(feature = "day22")]
pub mod day22;
#[cfg(feature = "day23")]
pub mod day23;
#[cfg(feature = "day24")]
pub mod day24;
#[cfg(feature = "day25")]
pub mod day25;

pub fn get_lines_from(filepath: &str) -> Vec<String> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let file = File::open(filepath).unwrap();

    let mut result = vec![];
    for line in BufReader::new(file).lines() {
        if let Ok(line) = line {
            result.push(line);
        }
    }

    result
}
