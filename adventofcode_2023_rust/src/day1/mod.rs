pub fn exec(lines: Vec<String>) {
    println!("[day1][part1] = {:?}", part1(lines.clone()));
    println!("[day1][part2] = {:?}", part2(lines));
}

fn part1(lines: Vec<String>) -> String {
    lines
        .iter()
        .map(|line| {
            let first_number = part1_get_first_number_from_line(line);
            let last_number = part1_get_last_number_from_line(line);
            format!("{first_number}{last_number}")
                .parse::<u32>()
                .unwrap()
        })
        .sum::<u32>()
        .to_string()
}

fn part2(_lines: Vec<String>) -> String {
    todo!()
}

fn part1_get_first_number_from_line(line: &str) -> u32 {
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

fn part1_get_last_number_from_line(line: &str) -> u32 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_get_first_number_from_line_success() {
        for (line, expected_number) in [
            ("eight9fhstbssrplmdlncmmqqnklb39ninejz", 9),
            ("three656", 6),
            ("ppjvndvknbtpfsncplmhhrlh5", 5),
        ] {
            assert_eq!(part1_get_first_number_from_line(line), expected_number);
        }
    }

    #[test]
    fn part1_get_last_number_from_line_success() {
        for (line, expected_number) in [
            ("eight9fhstbssrplmdlncmmqqnklb39ninejz", 9),
            ("three656", 6),
            ("ppjvndvknbtpfsncplmhhrlh5", 5),
        ] {
            assert_eq!(part1_get_last_number_from_line(line), expected_number);
        }
    }

    #[test]
    fn part1_3_lines() {
        let lines = vec![
            "eight9fhstbssrplmdlncmmqqnklb39ninejz",
            "three656",
            "ppjvndvknbtpfsncplmhhrlh5",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        assert_eq!(part1(lines), (99 + 66 + 55).to_string());
    }

    #[test]
    fn part2_get_first_number_from_line_success() {
        for (line, expected_number) in [
            ("eight9fhstbssrplmdlncmmqqnklb39ninejz", 8),
            ("three656", 3),
            ("ppjvndvknbtpfsncplmhhrlh5", 5),
        ] {
            assert_eq!(part1_get_first_number_from_line(line), expected_number);
        }
    }

    #[test]
    fn part2_get_last_number_from_line_success() {
        for (line, expected_number) in [
            ("eight9fhstbssrplmdlncmmqqnklb39ninejz", 9),
            ("three656", 6),
            ("ppjvndvknbtpfsncplmhhrlh5", 5),
        ] {
            assert_eq!(part1_get_last_number_from_line(line), expected_number);
        }
    }

    #[test]
    fn part2_3_lines() {
        let lines = vec![
            "eight9fhstbssrplmdlncmmqqnklb39ninejz",
            "three656",
            "ppjvndvknbtpfsncplmhhrlh5",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        assert_eq!(part1(lines), (89 + 36 + 55).to_string());
    }
}
