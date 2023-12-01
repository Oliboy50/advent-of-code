use std::fmt::Display;

pub fn exec(lines: Vec<String>) {
    println!("[day2][part1] = {}", part1(lines.clone()));
    println!("[day2][part2] = {}", part2(lines));
}

fn part1(lines: Vec<String>) -> impl Display {
    let mut horizontal = 0;
    let mut depth = 0;

    for line in lines {
        let line_parts = line
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let number = line_parts[1].parse::<u64>().unwrap();

        match line_parts[0].as_str() {
            "forward" => horizontal = horizontal + number,
            "up" => depth = depth - number,
            "down" => depth = depth + number,
            _ => {}
        }
    }

    horizontal * depth
}

fn part2(lines: Vec<String>) -> impl Display {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in lines {
        let line_parts = line
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let number = line_parts[1].parse::<u64>().unwrap();

        match line_parts[0].as_str() {
            "forward" => {
                horizontal = horizontal + number;
                depth = depth + (aim * number);
            }
            "up" => aim = aim - number,
            "down" => aim = aim + number,
            _ => {}
        }
    }

    horizontal * depth
}
