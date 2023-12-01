use regex::Regex;

pub fn exec(lines: Vec<String>) {
    println!("[day01][part1] = {}", part1(lines.clone()));
    println!("[day01][part2] = {}", part2(lines));
}

fn part1(lines: Vec<String>) -> String {
    lines
        .iter()
        .map(|line| {
            let first_number = get_first_numeric_number_from_line(line);
            let last_number = get_last_numeric_number_from_line(line);
            format!("{first_number}{last_number}")
                .parse::<u64>()
                .unwrap()
        })
        .sum::<u64>()
        .to_string()
}

fn part2(lines: Vec<String>) -> String {
    lines
        .iter()
        .map(|line| {
            let first_number = get_first_number_from_line(line).unwrap();
            let last_number = get_last_number_from_line(line).unwrap();
            format!("{}{}", first_number.value, last_number.value)
                .parse::<u64>()
                .unwrap()
        })
        .sum::<u64>()
        .to_string()
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct NumberInLine {
    index: u64,
    value: u32,
}

/// Only used by part1
fn get_first_numeric_number_from_line(line: &str) -> u32 {
    let first_letters = line
        .chars()
        .take_while(|c| c.is_alphabetic())
        .collect::<String>();
    line.chars()
        .nth(first_letters.len())
        .unwrap()
        .to_string()
        .parse::<u32>()
        .unwrap()
}

/// Only used by part1
fn get_last_numeric_number_from_line(line: &str) -> u32 {
    let last_letters = line
        .chars()
        .rev()
        .take_while(|c| c.is_alphabetic())
        .collect::<String>();
    line.chars()
        .rev()
        .nth(last_letters.len())
        .unwrap()
        .to_string()
        .parse::<u32>()
        .unwrap()
}

fn get_regex_for_digit_numbers() -> Regex {
    Regex::new(r"([0-9])?").unwrap()
}

fn get_regexes_for_alphabetical_numbers() -> Vec<Regex> {
    vec![
        Regex::new(r"(zero)?").unwrap(),
        Regex::new(r"(one)?").unwrap(),
        Regex::new(r"(two)?").unwrap(),
        Regex::new(r"(three)?").unwrap(),
        Regex::new(r"(four)?").unwrap(),
        Regex::new(r"(five)?").unwrap(),
        Regex::new(r"(six)?").unwrap(),
        Regex::new(r"(seven)?").unwrap(),
        Regex::new(r"(eight)?").unwrap(),
        Regex::new(r"(nine)?").unwrap(),
    ]
}

fn parse_number(number: &str) -> u32 {
    match number {
        "0" | "zero" => 0,
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => panic!("wrong usage of parse_number"),
    }
}

fn get_numbers_from_line(line: &str) -> Vec<NumberInLine> {
    let mut result = vec![];

    for captures in get_regexes_for_alphabetical_numbers()
        .iter()
        .flat_map(|regex| regex.captures_iter(line))
        .chain(get_regex_for_digit_numbers().captures_iter(line))
    {
        captures
            .iter()
            // the first item is the whole match, so we don't want it
            .skip(1)
            .filter(|m| m.is_some())
            .for_each(|m| {
                result.push(NumberInLine {
                    index: m.unwrap().start() as u64,
                    value: parse_number(m.unwrap().as_str()),
                });
            });
    }
    result
}

fn get_first_number_from_line(line: &str) -> Option<NumberInLine> {
    get_numbers_from_line(line)
        .iter()
        .fold(None, |index_of_first_number, current| {
            let Some(index_of_first_number) = index_of_first_number else {
                return Some(*current);
            };

            if current.index < index_of_first_number.index {
                Some(*current)
            } else {
                Some(index_of_first_number)
            }
        })
}

fn get_last_number_from_line(line: &str) -> Option<NumberInLine> {
    get_numbers_from_line(line)
        .iter()
        .fold(None, |index_of_last_number, current| {
            let Some(index_of_last_number) = index_of_last_number else {
                return Some(*current);
            };

            if current.index > index_of_last_number.index {
                Some(*current)
            } else {
                Some(index_of_last_number)
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let lines = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        assert_eq!(part1(lines), "142".to_string());
    }

    #[test]
    fn get_first_numeric_number_from_line_success() {
        for (line, expected) in [
            ("eight9fhstbssrplmdlncmmqqnklb39ninejz", 9),
            ("three656", 6),
            ("ppjvndvknbtpfsncplmhhrlh5", 5),
        ] {
            assert_eq!(get_first_numeric_number_from_line(line), expected);
        }
    }

    #[test]
    fn get_last_numeric_number_from_line_success() {
        for (line, expected) in [
            ("eight9fhstbssrplmdlncmmqqnklb39ninejz", 9),
            ("three656", 6),
            ("ppjvndvknbtpfsncplmhhrlh5", 5),
        ] {
            assert_eq!(get_last_numeric_number_from_line(line), expected);
        }
    }

    #[test]
    fn part2_example() {
        let lines = vec![
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        assert_eq!(part2(lines), "281".to_string());
    }

    #[test]
    fn part2_edge_case_not_covered_in_example() {
        let lines = vec!["eighthree", "sevenine"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        assert_eq!(part2(lines), (83 + 79).to_string());
    }

    #[test]
    fn get_first_alphabetical_number_from_line_success() {
        for (line, expected) in [
            (
                "eight9fhstbssrplmdlncmmqqnklb39ninejz",
                Some(NumberInLine { index: 0, value: 8 }),
            ),
            ("three656", Some(NumberInLine { index: 0, value: 3 })),
            (
                "ppjvndvknbtpfsncplmhhrlh5",
                Some(NumberInLine {
                    index: 24,
                    value: 5,
                }),
            ),
        ] {
            assert_eq!(get_first_number_from_line(line), expected);
        }
    }

    #[test]
    fn get_last_alphabetical_number_from_line_success() {
        for (line, expected) in [
            (
                "eight9fhstbssrplmdlncmmqqnklb39ninejz",
                Some(NumberInLine {
                    index: 31,
                    value: 9,
                }),
            ),
            ("three656", Some(NumberInLine { index: 7, value: 6 })),
            (
                "ppjvndvknbtpfsncplmhhrlh5",
                Some(NumberInLine {
                    index: 24,
                    value: 5,
                }),
            ),
        ] {
            assert_eq!(get_last_number_from_line(line), expected);
        }
    }
}
