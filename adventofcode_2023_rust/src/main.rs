#[cfg(feature = "day1")]
mod day1;
#[cfg(feature = "day2")]
mod day2;
#[cfg(feature = "day3")]
mod day3;
#[cfg(feature = "day4")]
mod day4;
#[cfg(feature = "day5")]
mod day5;
#[cfg(feature = "day6")]
mod day6;
#[cfg(feature = "day7")]
mod day7;
#[cfg(feature = "day8")]
mod day8;
#[cfg(feature = "day9")]
mod day9;
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
    #[cfg(feature = "day1")]
    day1::exec(get_lines_from("src/day1/input"));

    #[cfg(feature = "day2")]
    day2::exec(get_lines_from("src/day2/input"));

    #[cfg(feature = "day3")]
    day3::exec(get_lines_from("src/day3/input"));

    #[cfg(feature = "day4")]
    day4::exec(get_lines_from("src/day4/input"));

    #[cfg(feature = "day5")]
    day5::exec(get_lines_from("src/day5/input"));

    #[cfg(feature = "day6")]
    day6::exec(get_lines_from("src/day6/input"));

    #[cfg(feature = "day7")]
    day7::exec(get_lines_from("src/day7/input"));

    #[cfg(feature = "day8")]
    day8::exec(get_lines_from("src/day8/input"));

    #[cfg(feature = "day9")]
    day9::exec(get_lines_from("src/day9/input"));

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
