use regex::Regex;

pub fn exec(lines: Vec<String>) {
    println!("[day03][part1] = {}", part1(lines.clone()));
    println!("[day03][part2] = {}", part2(lines));
}

fn part1(lines: Vec<String>) -> String {
    let mut numbers = Vec::default();
    for (index, line) in lines.iter().enumerate() {
        let previous_line = if index == 0 {
            None
        } else {
            Some(lines[index - 1].clone())
        };
        let next_line = if index == lines.len() - 1 {
            None
        } else {
            Some(lines[index + 1].clone())
        };

        numbers.push(part1_get_numbers_from_line(Line {
            previous: previous_line,
            current: line.clone(),
            next: next_line,
        }));
    }

    numbers
        .iter()
        .flatten()
        .filter(|n| n.is_closed_to_symbol)
        .map(|n| n.value)
        .sum::<u64>()
        .to_string()
}

fn part2(lines: Vec<String>) -> String {
    let mut gears = Vec::default();
    for (index, line) in lines.iter().enumerate() {
        let previous_line = if index == 0 {
            None
        } else {
            Some(lines[index - 1].clone())
        };
        let next_line = if index == lines.len() - 1 {
            None
        } else {
            Some(lines[index + 1].clone())
        };

        gears.push(part2_get_gears_from_line(Line {
            previous: previous_line,
            current: line.clone(),
            next: next_line,
        }));
    }

    gears
        .iter()
        .flatten()
        .filter(|g| g.numbers.len() == 2)
        .map(|g| g.numbers[0].value * g.numbers[1].value)
        .sum::<u64>()
        .to_string()
}

#[derive(Debug, Clone, PartialEq)]
struct Line {
    previous: Option<String>,
    current: String,
    next: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
struct Part1SymbolHorizontalBlastRadius {
    start_index: u32,
    end_index: u32,
}

#[derive(Debug, Clone, PartialEq)]
struct Part1Number {
    start_index: u32,
    end_index: u32,
    value: u64,
    is_closed_to_symbol: bool,
}

fn part1_get_symbols_blast_radius_from_line(line: &str) -> Vec<Part1SymbolHorizontalBlastRadius> {
    let mut result = vec![];
    let regex_for_symbols = Regex::new(r"(?<symbol>[^0-9.]+)*").unwrap();

    for captures in regex_for_symbols.captures_iter(line) {
        let Some(symbol) = captures.name("symbol") else {
            continue;
        };

        let start = symbol.start() as u32;
        let end = symbol.end() as u32;
        result.push(Part1SymbolHorizontalBlastRadius {
            start_index: start.saturating_sub(1),
            end_index: end.saturating_add(1),
        });
    }

    result
}

fn part1_get_numbers_from_line(line: Line) -> Vec<Part1Number> {
    let mut result = vec![];
    let regex_for_numbers = Regex::new(r"(?<number>[0-9]+)*").unwrap();

    for captures in regex_for_numbers.captures_iter(&line.current) {
        let Some(number) = captures.name("number") else {
            continue;
        };
        let start_index = number.start() as u32;
        let end_index = number.end() as u32;

        result.push(Part1Number {
            start_index,
            end_index,
            value: number.as_str().parse::<u64>().unwrap(),
            is_closed_to_symbol: part1_is_number_closed_to_symbol(&line, start_index, end_index),
        });
    }

    result
}

fn part1_is_number_closed_to_symbol(
    line: &Line,
    number_start_index: u32,
    number_end_index: u32,
) -> bool {
    let mut result = false;

    let symbols_blast_radius_for_current_line =
        part1_get_symbols_blast_radius_from_line(&line.current);
    let symbols_blast_radius_for_previous_line = if let Some(previous_line) = &line.previous {
        part1_get_symbols_blast_radius_from_line(previous_line)
    } else {
        vec![]
    };
    let symbols_blast_radius_for_next_line = if let Some(next_line) = &line.next {
        part1_get_symbols_blast_radius_from_line(next_line)
    } else {
        vec![]
    };

    for symbol_blast_radius in symbols_blast_radius_for_current_line
        .iter()
        .chain(symbols_blast_radius_for_previous_line.iter())
        .chain(symbols_blast_radius_for_next_line.iter())
    {
        let blast_radius = symbol_blast_radius.start_index..symbol_blast_radius.end_index;
        if blast_radius.contains(&number_start_index)
            || blast_radius.contains(&(number_end_index - 1))
        {
            result = true;
            break;
        }
    }

    result
}

#[derive(Debug, Clone, PartialEq)]
struct Part2Gear {
    start_index: u32,
    end_index: u32,
    numbers: Vec<Part2Number>,
}

#[derive(Debug, Clone, PartialEq, Copy)]
struct Part2Number {
    start_index: u32,
    end_index: u32,
    value: u64,
}

fn part2_get_numbers_from_line(line: &str) -> Vec<Part2Number> {
    let mut result = vec![];
    let regex_for_numbers = Regex::new(r"(?<number>[0-9]+)*").unwrap();

    for captures in regex_for_numbers.captures_iter(line) {
        let Some(number) = captures.name("number") else {
            continue;
        };
        let start_index = number.start() as u32;
        let end_index = number.end() as u32;

        result.push(Part2Number {
            start_index,
            end_index,
            value: number.as_str().parse::<u64>().unwrap(),
        });
    }

    result
}

fn part2_get_gears_from_line(line: Line) -> Vec<Part2Gear> {
    let mut result = vec![];
    let regex_for_symbols = Regex::new(r"(?<symbol>\*)*").unwrap();

    for captures in regex_for_symbols.captures_iter(&line.current) {
        let Some(symbol) = captures.name("symbol") else {
            continue;
        };

        let start = symbol.start() as u32;
        let start_index = start.saturating_sub(1);
        let end = symbol.end() as u32;
        let end_index = end.saturating_add(1);
        result.push(Part2Gear {
            start_index,
            end_index,
            numbers: part2_get_numbers_closed_to_gear(&line, start_index, end_index),
        });
    }

    result
}

fn part2_get_numbers_closed_to_gear(
    line: &Line,
    gear_start_index: u32,
    gear_end_index: u32,
) -> Vec<Part2Number> {
    let mut result = Vec::default();

    let numbers_for_current_line = part2_get_numbers_from_line(&line.current);
    let numbers_for_previous_line = if let Some(previous_line) = &line.previous {
        part2_get_numbers_from_line(previous_line)
    } else {
        vec![]
    };
    let numbers_for_next_line = if let Some(next_line) = &line.next {
        part2_get_numbers_from_line(next_line)
    } else {
        vec![]
    };

    for number in numbers_for_current_line
        .iter()
        .chain(numbers_for_previous_line.iter())
        .chain(numbers_for_next_line.iter())
    {
        let blast_radius = gear_start_index..gear_end_index;
        if blast_radius.contains(&number.start_index)
            || blast_radius.contains(&(number.end_index - 1))
        {
            result.push(*number);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let lines = vec![
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(
            part1(lines),
            "4361" // 467 + 35 + 633 + 617 + 592 + 755 + 664 + 598
        );
    }

    #[test]
    fn part1_more_cases() {
        let lines = vec![
            "12.......*..",
            "+.........34",
            ".......-12..",
            "..78........",
            "..*....60...",
            "78.........9",
            ".5.....23..$",
            "8...90*12...",
            "............",
            "2.2......12.",
            ".*.........*",
            "1.1..503+.56",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(part1(lines), "925"); // 12 + 34 + 12 + 78 + 78 + 9 + 23 + 90 + 12 + 2 + 2 + 12 + 1 + 1 + 503 + 56
    }

    #[test]
    fn part1_more_cases_2() {
        let lines = vec![
            "....................",
            "..-52..52-..52..52..",
            "..................-.",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(part1(lines), "156"); // 52 + 52 + 52
    }

    #[test]
    fn part1_more_cases_3() {
        let lines = vec![
            "......124..................418.......587......770...........672.................564............................438..........512......653....",
            "665/...*......................*599.....*.983......794*..140..*...........@..963*....................445........*......*.........709.....*...",
            ".......246.....581......701..........108....%.532........../.73..699...927............................*....579.354.464..............298..86.",
        ]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(part1(lines), "10303"); // 124 + 418 + 587 + 672 + 564 + 438 + 653 + 665 + 599 + 983 + 794 + 140 + 963 + 445 + 246 + 108 + 73 + 927 + 354 + 464 + 86
    }

    #[test]
    fn part1_more_cases_4() {
        let lines = vec![
            "......124..................418.......587......770...........672.................564............................438..........512......653....",
            "665/...*......................*599.....*.983......794*..140..*...........@..963*....................445........*......*.........709.....*...",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(part1(lines), "6477"); // 124 + 418 + 587 + 672 + 564 + 438 + 653 + 665 + 599 + 794 + 963
    }

    #[test]
    fn part1_more_cases_5() {
        let lines = vec![
            "665/...*......................*599.....*.983......794*..140..*...........@..963*....................445........*......*.........709.....*...",
            ".......246.....581......701..........108....%.532........../.73..699...927............................*....579.354.464..............298..86.",
        ] // 877 = 709 + 73 + 95
        .iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(part1(lines), "6847"); // 665 + 599 + 983 + 794 + 140 + 963 + 445 + 246 + 108 + 73 + 927 + 354 + 464 + 86
    }

    #[test]
    fn part1_more_cases_6() {
        let lines = vec![
            "...............",
            "...............",
            "...709.....*...",
            ".......298..86.",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(part1(lines), "86");
    }

    #[test]
    fn part1_get_symbols_blast_radius_from_line_success() {
        for (input, expected) in [
            ("467..114..", vec![]),
            ("..35..6335", vec![]),
            (
                "...*......",
                vec![Part1SymbolHorizontalBlastRadius {
                    start_index: 2,
                    end_index: 5,
                }],
            ),
            (
                "......#...",
                vec![Part1SymbolHorizontalBlastRadius {
                    start_index: 5,
                    end_index: 8,
                }],
            ),
            (
                "617*......",
                vec![Part1SymbolHorizontalBlastRadius {
                    start_index: 2,
                    end_index: 5,
                }],
            ),
        ] {
            assert_eq!(part1_get_symbols_blast_radius_from_line(input), expected);
        }
    }

    #[test]
    fn part1_get_numbers_from_line_success() {
        for (input, expected) in [
            (
                Line {
                    previous: None,
                    current: "...*......".to_string(),
                    next: None,
                },
                vec![],
            ),
            (
                Line {
                    previous: None,
                    current: "467*.114..".to_string(),
                    next: None,
                },
                vec![
                    Part1Number {
                        start_index: 0,
                        end_index: 3,
                        value: 467,
                        is_closed_to_symbol: true,
                    },
                    Part1Number {
                        start_index: 5,
                        end_index: 8,
                        value: 114,
                        is_closed_to_symbol: false,
                    },
                ],
            ),
            (
                Line {
                    previous: None,
                    current: "..35..633.".to_string(),
                    next: Some("***.......".to_string()),
                },
                vec![
                    Part1Number {
                        start_index: 2,
                        end_index: 4,
                        value: 35,
                        is_closed_to_symbol: true,
                    },
                    Part1Number {
                        start_index: 6,
                        end_index: 9,
                        value: 633,
                        is_closed_to_symbol: false,
                    },
                ],
            ),
            (
                Line {
                    previous: Some("...*......".to_string()),
                    current: "617.......".to_string(),
                    next: None,
                },
                vec![Part1Number {
                    start_index: 0,
                    end_index: 3,
                    value: 617,
                    is_closed_to_symbol: true,
                }],
            ),
            (
                Line {
                    previous: Some("...709.....*...".to_string()),
                    current: ".......298..86.".to_string(),
                    next: Some("...............".to_string()),
                },
                vec![
                    Part1Number {
                        start_index: 7,
                        end_index: 10,
                        value: 298,
                        is_closed_to_symbol: false,
                    },
                    Part1Number {
                        start_index: 12,
                        end_index: 14,
                        value: 86,
                        is_closed_to_symbol: true,
                    },
                ],
            ),
        ] {
            assert_eq!(part1_get_numbers_from_line(input), expected);
        }
    }

    #[test]
    fn part2_example() {
        let lines = vec![
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(
            part2(lines),
            "467835" // (467 * 35) + (755 * 598)
        );
    }

    #[test]
    fn part2_more_cases() {
        let lines = vec![
            ".....$.*..........",
            "617*..............",
            "664.598...........",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(part2(lines), "0");
    }

    #[test]
    fn part2_more_cases_2() {
        let lines = vec![
            "...............",
            "...............",
            "...709.....*...",
            ".......298..86.",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(part2(lines), "0");
    }
}
