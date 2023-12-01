#[cfg(feature = "day01")]
mod day01;
#[cfg(feature = "day02")]
mod day02;
#[cfg(feature = "day03")]
mod day03;
#[cfg(feature = "day04")]
mod day04;
#[cfg(feature = "day05")]
mod day05;
#[cfg(feature = "day06")]
mod day06;
#[cfg(feature = "day07")]
mod day07;
#[cfg(feature = "day08")]
mod day08;
#[cfg(feature = "day09")]
mod day09;
#[cfg(feature = "day10")]
mod day10;
#[cfg(feature = "day11")]
mod day11;
#[cfg(feature = "day12")]
mod day12;
#[cfg(feature = "day13")]
mod day13;
#[cfg(feature = "day14")]
mod day14;
#[cfg(feature = "day15")]
mod day15;
#[cfg(feature = "day16")]
mod day16;
#[cfg(feature = "day17")]
mod day17;
#[cfg(feature = "day18")]
mod day18;
#[cfg(feature = "day19")]
mod day19;
#[cfg(feature = "day20")]
mod day20;
#[cfg(feature = "day21")]
mod day21;
#[cfg(feature = "day22")]
mod day22;
#[cfg(feature = "day23")]
mod day23;
#[cfg(feature = "day24")]
mod day24;
#[cfg(feature = "day25")]
mod day25;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    #[cfg(feature = "day01")]
    day01::exec(get_lines_from("src/day01/input"));

    #[cfg(feature = "day02")]
    day02::exec(get_lines_from("src/day02/input"));

    #[cfg(feature = "day03")]
    day03::exec(get_lines_from("src/day03/input"));

    #[cfg(feature = "day04")]
    day04::exec(get_lines_from("src/day04/input"));

    #[cfg(feature = "day05")]
    day05::exec(get_lines_from("src/day05/input"));

    #[cfg(feature = "day06")]
    day06::exec(get_lines_from("src/day06/input"));

    #[cfg(feature = "day07")]
    day07::exec(get_lines_from("src/day07/input"));

    #[cfg(feature = "day08")]
    day08::exec(get_lines_from("src/day08/input"));

    #[cfg(feature = "day09")]
    day09::exec(get_lines_from("src/day09/input"));

    #[cfg(feature = "day10")]
    day10::exec(get_lines_from("src/day10/input"));

    #[cfg(feature = "day11")]
    day11::exec(get_lines_from("src/day11/input"));

    #[cfg(feature = "day12")]
    day12::exec(get_lines_from("src/day12/input"));

    #[cfg(feature = "day13")]
    day13::exec(get_lines_from("src/day13/input"));

    #[cfg(feature = "day14")]
    day14::exec(get_lines_from("src/day14/input"));

    #[cfg(feature = "day15")]
    day15::exec(get_lines_from("src/day15/input"));

    #[cfg(feature = "day16")]
    day16::exec(get_lines_from("src/day16/input"));

    #[cfg(feature = "day17")]
    day17::exec(get_lines_from("src/day17/input"));

    #[cfg(feature = "day18")]
    day18::exec(get_lines_from("src/day18/input"));

    #[cfg(feature = "day19")]
    day19::exec(get_lines_from("src/day19/input"));

    #[cfg(feature = "day20")]
    day20::exec(get_lines_from("src/day20/input"));

    #[cfg(feature = "day21")]
    day21::exec(get_lines_from("src/day21/input"));

    #[cfg(feature = "day22")]
    day22::exec(get_lines_from("src/day22/input"));

    #[cfg(feature = "day23")]
    day23::exec(get_lines_from("src/day23/input"));

    #[cfg(feature = "day24")]
    day24::exec(get_lines_from("src/day24/input"));

    #[cfg(feature = "day25")]
    day25::exec(get_lines_from("src/day25/input"));
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
