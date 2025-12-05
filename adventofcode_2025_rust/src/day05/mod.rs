pub fn exec(lines: Vec<String>) {
    println!("[day05][part1] = {}", part1(lines.clone()));
    println!("[day05][part2] = {}", part2(lines));
}

pub fn part1(lines: Vec<String>) -> String {
    let mut ranges: Vec<(u128, u128)> = Vec::new();
    let mut numbers_to_check: Vec<u128> = Vec::new();
    let mut parsing_ranges = true;

    for line in lines {
        if line.is_empty() {
            parsing_ranges = false;
            continue;
        }

        if parsing_ranges {
            let parts: Vec<&str> = line.split('-').collect();
            let start: u128 = parts[0].parse().unwrap();
            let end: u128 = parts[1].parse().unwrap();
            ranges.push((start, end));
        } else {
            let number: u128 = line.parse().unwrap();
            numbers_to_check.push(number);
        }
    }

    let mut count: u64 = 0;
    for &number in &numbers_to_check {
        for &(start, end) in &ranges {
            if number >= start && number <= end {
                count += 1;
                break;
            }
        }
    }

    count.to_string()
}

pub fn part2(lines: Vec<String>) -> String {
    let mut how_many_numbers_are_in_one_or_more_ranges: u128 = 0;
    let mut ranges: Vec<(u128, u128)> = Vec::new();

    for line in lines {
        if line.is_empty() {
            break;
        }

        let parts: Vec<&str> = line.split('-').collect();
        let start: u128 = parts[0].parse().unwrap();
        let end: u128 = parts[1].parse().unwrap();
        ranges.push((start, end));
    }
    let mut merged_ranges: Vec<(u128, u128)> = Vec::new();
    ranges.sort_by_key(|k| k.0);
    for range in ranges {
        if merged_ranges.is_empty() {
            merged_ranges.push(range);
        } else {
            let last_range = merged_ranges.last_mut().unwrap();
            if range.0 <= last_range.1 + 1 {
                last_range.1 = last_range.1.max(range.1);
            } else {
                merged_ranges.push(range);
            }
        }
    }
    for (start, end) in merged_ranges {
        how_many_numbers_are_in_one_or_more_ranges += end - start + 1;
    }

    how_many_numbers_are_in_one_or_more_ranges.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let lines = vec![
            "3-5", "10-14", "16-20", "12-18", "", "1", "5", "8", "11", "17", "32",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        assert_eq!(part1(lines), "3".to_string());
    }

    #[test]
    fn part1_edge_cases() {
        let lines = vec![
            "555804456885533-560879776586056",
            "",
            "555804456885532",
            "555804456885533",
            "560000000000000",
            "560879776586056",
            "560879776586057",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        assert_eq!(part1(lines), "3".to_string());
    }

    #[test]
    fn part2_example() {
        let lines = vec!["3-5", "10-14", "16-20", "12-18"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        assert_eq!(part2(lines), "14".to_string());
    }
}
