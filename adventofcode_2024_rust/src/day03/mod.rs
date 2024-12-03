use regex::Regex;

pub fn exec(lines: Vec<String>) {
    println!("[day03][part1] = {}", part1(lines.clone()));
    println!("[day03][part2] = {}", part2(lines));
}

fn part1(lines: Vec<String>) -> String {
    let muls: Vec<Mul> = lines
        .iter()
        .flat_map(|line| extract_mul_from_line(line))
        .collect();

    muls.iter()
        .fold(0, |acc, mul| acc + (mul.x * mul.y))
        .to_string()
}

fn part2(lines: Vec<String>) -> String {
    let lines = lines
        .iter()
        .flat_map(|line| split_line_by_do_or_dont(line))
        .filter(|line| line.starts_with("do()"))
        .collect();

    part1(lines)
}

#[derive(Debug, Clone)]
struct Mul {
    x: u64,
    y: u64,
}

fn extract_mul_from_line(line: &str) -> Vec<Mul> {
    let mut muls = Vec::new();
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    for cap in re.captures_iter(line) {
        let x = cap[1].parse::<u64>().unwrap();
        let y = cap[2].parse::<u64>().unwrap();
        muls.push(Mul { x, y });
    }

    muls
}

fn split_line_by_do_or_dont(line: &str) -> Vec<String> {
    let mut lines = Vec::new();
    let re = Regex::new(r"do\(\)|don't\(\)").unwrap();

    let mut last_match_cursor = 0;
    for cap in re.captures_iter(&line) {
        if let Some(matched) = cap.get(0) {
            // skip empty matches
            if last_match_cursor == matched.start() {
                continue;
            }

            // keep the characters between the last match and the current match
            let match_cursor = matched.start();
            lines.push(line[last_match_cursor..match_cursor].to_string());
            last_match_cursor = match_cursor;
        }
    }

    // keep the remaining characters of the line after the last match
    if last_match_cursor < line.len() {
        lines.push(line[last_match_cursor..].to_string());
    }

    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let lines = vec!["xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        assert_eq!(part1(lines), "161".to_string());

        let lines = vec![
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)",
            "+mul(32,64]then(mul(11,8)mul(8,5))",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        assert_eq!(part1(lines), "161".to_string());
    }

    #[test]
    fn part2_example() {
        let lines =
            vec!["do()xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"]
                .iter()
                .map(|s| s.to_string())
                .collect();

        assert_eq!(part2(lines), "48".to_string());

        let lines = vec![
            "do()xmul(2,4)&mul[3,7]!^don't()_mul(5,5)",
            "don't()+mul(32,64](mul(11,8)undo()?mul(8,5))",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        assert_eq!(part2(lines), "48".to_string());
    }

    #[test]
    fn split_line_by_do_or_dont_when_start_with_do() {
        let line = "do()xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        assert_eq!(
            split_line_by_do_or_dont(line),
            vec![
                "do()xmul(2,4)&mul[3,7]!^".to_string(),
                "don't()_mul(5,5)+mul(32,64](mul(11,8)un".to_string(),
                "do()?mul(8,5))".to_string()
            ]
        );
    }

    #[test]
    fn split_line_by_do_or_dont_when_start_with_dont() {
        let line =
            "don't()xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        assert_eq!(
            split_line_by_do_or_dont(line),
            vec![
                "don't()xmul(2,4)&mul[3,7]!^".to_string(),
                "don't()_mul(5,5)+mul(32,64](mul(11,8)un".to_string(),
                "do()?mul(8,5))".to_string()
            ]
        );
    }
}
