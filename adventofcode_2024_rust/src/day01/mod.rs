pub fn exec(lines: Vec<String>) {
    println!("[day01][part1] = {}", part1(lines.clone()));
    println!("[day01][part2] = {}", part2(lines));
}

fn part1(lines: Vec<String>) -> String {
    let first_column_numbers = get_left_column_numbers_sorted_asc(lines.clone());
    let last_column_numbers = get_right_column_numbers_sorted_asc(lines);

    let mut sum = 0;
    for (i, number) in first_column_numbers.iter().enumerate() {
        if let Some(second_number) = last_column_numbers.get(i) {
            sum += number.abs_diff(*second_number);
        }
    }

    sum.to_string()
}

fn part2(lines: Vec<String>) -> String {
    let first_column_numbers = get_left_column_numbers_sorted_asc(lines.clone());
    let last_column_numbers = get_right_column_numbers_sorted_asc(lines);

    let mut last_computed_key_value = (0, 0);
    let mut sum = 0;
    for number in first_column_numbers {
        // the list is sorted by ascending order,
        // so if the number is the same as the last one,
        // we can reuse the result
        if number == last_computed_key_value.0 {
            sum += last_computed_key_value.1;
            continue;
        }

        let number_of_occurrences = get_number_of_times_the_number_is_present_in_the_list(number, &last_column_numbers);
        let result = number * number_of_occurrences;
        last_computed_key_value = (number, result);
        sum += result;
    }

    sum.to_string()
}

fn get_number_of_times_the_number_is_present_in_the_list(number: u64, list: &[u64]) -> u64 {
    list.iter().filter(|&n| *n == number).count() as u64
}

fn get_left_column_numbers_sorted_asc(lines: Vec<String>) -> Vec<u64> {
    let numbers: Vec<u64> = lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .next()
                .unwrap()
                .parse::<u64>()
                .unwrap()
        })
        .collect();

    let mut numbers_sorted = numbers.clone();
    numbers_sorted.sort();
    numbers_sorted
}

fn get_right_column_numbers_sorted_asc(lines: Vec<String>) -> Vec<u64> {
    let numbers: Vec<u64> = lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .last()
                .unwrap()
                .parse::<u64>()
                .unwrap()
        })
        .collect();

    let mut numbers_sorted = numbers.clone();
    numbers_sorted.sort();
    numbers_sorted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let lines = vec![
            "3   4",
            "4   3",
            "2   5",
            "1   3",
            "3   9",
            "3   3",
        ]
            .iter()
            .map(|s| s.to_string())
            .collect();

        assert_eq!(part1(lines), "11".to_string());
    }

    #[test]
    fn part2_example() {
        let lines = vec![
            "3   4",
            "4   3",
            "2   5",
            "1   3",
            "3   9",
            "3   3",
        ]
            .iter()
            .map(|s| s.to_string())
            .collect();

        assert_eq!(part2(lines), "31".to_string());
    }
}
