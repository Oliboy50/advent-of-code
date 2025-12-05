#![allow(dead_code)]

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
}
