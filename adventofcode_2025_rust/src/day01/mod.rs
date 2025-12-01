pub fn exec(lines: Vec<String>) {
    println!("[day01][part1] = {}", part1(lines.clone()));
    println!("[day01][part2] = {}", part2(lines));
}

enum Direction {
    Left,
    Right,
}

fn get_direction(line: &String) -> Direction {
    if line.starts_with("L") {
        Direction::Left
    } else {
        Direction::Right
    }
}

fn get_value(line: &String) -> u32 {
    line[1..].parse().unwrap()
}

pub fn part1(lines: Vec<String>) -> String {
    let mut current_value: i32 = 50;
    let mut number_of_times_value_is_zero: u32 = 0;

    for line in lines {
        let direction = get_direction(&line);
        let value = get_value(&line);

        match direction {
            Direction::Left => {
                current_value -= value as i32;
            }
            Direction::Right => {
                current_value += value as i32;
            }
        }

        current_value = current_value % 100;
        if current_value < 0 {
            current_value = 100 - current_value.abs();
        }

        if current_value == 0 {
            number_of_times_value_is_zero += 1;
        }
    }

    return number_of_times_value_is_zero.to_string();
}

pub fn part2(lines: Vec<String>) -> String {
    let mut current_value: i32 = 50;
    let mut number_of_times_value_pass_by_zero: u32 = 0;

    for line in lines {
        let direction = get_direction(&line);
        let value = get_value(&line);

        if value >= 100 {
            number_of_times_value_pass_by_zero += value / 100;
            match direction {
                Direction::Left => {
                    if current_value != 0 && (current_value - (value % 100) as i32) <= 0 {
                        number_of_times_value_pass_by_zero += 1;
                    }
                }
                Direction::Right => {
                    if (current_value + (value % 100) as i32) >= 100 {
                        number_of_times_value_pass_by_zero += 1;
                    }
                }
            }
        } else if matches!(direction, Direction::Left)
            && current_value != 0
            && current_value - value as i32 <= 0
        {
            number_of_times_value_pass_by_zero += 1;
        } else if matches!(direction, Direction::Right) && current_value + value as i32 >= 100 {
            number_of_times_value_pass_by_zero += 1;
        }

        match direction {
            Direction::Left => {
                current_value -= value as i32;
            }
            Direction::Right => {
                current_value += value as i32;
            }
        }

        current_value = current_value % 100;
        if current_value < 0 {
            current_value = 100 - current_value.abs();
        }
    }

    return number_of_times_value_pass_by_zero.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let lines = vec![
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        assert_eq!(part1(lines), "3".to_string());
    }

    #[test]
    fn part2_example() {
        let lines = vec![
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        assert_eq!(part2(lines), "6".to_string());
    }

    #[test]
    fn part2_with_r50() {
        let lines = vec!["R50"].iter().map(|s| s.to_string()).collect();

        assert_eq!(part2(lines), "1".to_string());
    }

    #[test]
    fn part2_with_l50() {
        let lines = vec!["L50"].iter().map(|s| s.to_string()).collect();

        assert_eq!(part2(lines), "1".to_string());
    }

    #[test]
    fn part2_with_r1000() {
        let lines = vec!["R1000"].iter().map(|s| s.to_string()).collect();

        assert_eq!(part2(lines), "10".to_string());
    }

    #[test]
    fn part2_with_l1000() {
        let lines = vec!["L1000"].iter().map(|s| s.to_string()).collect();

        assert_eq!(part2(lines), "10".to_string());
    }

    #[test]
    fn part2_with_l190() {
        let lines = vec!["L190"].iter().map(|s| s.to_string()).collect();

        assert_eq!(part2(lines), "2".to_string());
    }

    #[test]
    fn part2_with_l50_l199() {
        let lines = vec!["L50", "L199"].iter().map(|s| s.to_string()).collect();

        assert_eq!(part2(lines), "2".to_string());
    }

    #[test]
    fn part2_with_l50_l200() {
        let lines = vec!["L50", "L200"].iter().map(|s| s.to_string()).collect();

        assert_eq!(part2(lines), "3".to_string());
    }

    #[test]
    fn part2_with_l50_l201() {
        let lines = vec!["L50", "L201"].iter().map(|s| s.to_string()).collect();

        assert_eq!(part2(lines), "3".to_string());
    }

    #[test]
    fn part2_with_r190() {
        let lines = vec!["R190"].iter().map(|s| s.to_string()).collect();

        assert_eq!(part2(lines), "2".to_string());
    }

    #[test]
    fn part2_with_r50_r199() {
        let lines = vec!["R50", "R199"].iter().map(|s| s.to_string()).collect();

        assert_eq!(part2(lines), "2".to_string());
    }

    #[test]
    fn part2_with_r50_r200() {
        let lines = vec!["R50", "R200"].iter().map(|s| s.to_string()).collect();

        assert_eq!(part2(lines), "3".to_string());
    }

    #[test]
    fn part2_with_r50_r201() {
        let lines = vec!["R50", "R201"].iter().map(|s| s.to_string()).collect();

        assert_eq!(part2(lines), "3".to_string());
    }

    #[test]
    fn part2_with_r43_l693() {
        let lines = vec!["R43", "L693"].iter().map(|s| s.to_string()).collect();

        assert_eq!(part2(lines), "7".to_string());
    }

    #[test]
    fn part2_with_l43_r693() {
        let lines = vec!["L43", "R693"].iter().map(|s| s.to_string()).collect();

        assert_eq!(part2(lines), "7".to_string());
    }
}
