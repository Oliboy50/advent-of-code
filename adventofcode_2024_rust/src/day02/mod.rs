use std::collections::HashMap;

pub fn exec(lines: Vec<String>) {
    println!("[day02][part1] = {}", part1(lines.clone()));
    println!("[day02][part2] = {}", part2(lines));
}

fn part1(lines: Vec<String>) -> String {
    let all_increasing_or_decreasing_lines =
        keep_only_lines_with_all_increasing_or_decreasing_numbers(lines);
    let all_increasing_or_decreasing_lines_with_max_3_diff =
        keep_only_lines_with_max_3_absolute_diff_between_each_number(
            all_increasing_or_decreasing_lines,
        );

    all_increasing_or_decreasing_lines_with_max_3_diff
        .iter()
        .count()
        .to_string()
}

fn part2(lines: Vec<String>) -> String {
    // lines from part 1 are still valid
    let all_increasing_or_decreasing_lines =
        keep_only_lines_with_all_increasing_or_decreasing_numbers(lines.clone());
    let all_increasing_or_decreasing_lines_with_max_3_diff =
        keep_only_lines_with_max_3_absolute_diff_between_each_number(
            all_increasing_or_decreasing_lines,
        );

    // but remaining lines can also be valid now
    let remaining_lines: Vec<_> = lines
        .iter()
        .filter(|line| !all_increasing_or_decreasing_lines_with_max_3_diff.contains(line))
        .map(|s| s.to_string())
        .collect();
    let remaining_lines = transform_lines_to_enumerated_variants(remaining_lines);
    let all_increasing_or_decreasing_lines_if_we_remove_a_wrong_number =
        keep_only_variants_with_increasing_or_decreasing_numbers(remaining_lines);
    let all_increasing_or_decreasing_lines_with_max_3_diff_if_we_remove_a_wrong_number =
        keep_only_variants_with_max_3_absolute_diff_between_each_number(
            all_increasing_or_decreasing_lines_if_we_remove_a_wrong_number,
        );

    // combine both results
    let total = all_increasing_or_decreasing_lines_with_max_3_diff
        .iter()
        .count()
        + all_increasing_or_decreasing_lines_with_max_3_diff_if_we_remove_a_wrong_number
            .iter()
            .count();
    total.to_string()
}

fn is_all_increasing_or_decreasing(numbers: &[u64]) -> bool {
    let mut is_all_increasing = true;
    let mut is_all_decreasing = true;
    for i in 1..numbers.len() {
        if numbers[i] > numbers[i - 1] {
            is_all_decreasing = false;
        } else if numbers[i] < numbers[i - 1] {
            is_all_increasing = false;
        }
    }

    is_all_increasing || is_all_decreasing
}

fn is_max_3_diff_between_each_number(numbers: &[u64]) -> bool {
    let mut is_max_3_diff = true;
    for i in 1..numbers.len() {
        let diff = numbers[i].abs_diff(numbers[i - 1]);
        if diff <= 0 || diff > 3 {
            is_max_3_diff = false;
        }
    }

    is_max_3_diff
}

fn keep_only_lines_with_all_increasing_or_decreasing_numbers(lines: Vec<String>) -> Vec<String> {
    lines
        .iter()
        .filter(|line| {
            let numbers: Vec<u64> = line
                .split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect();

            is_all_increasing_or_decreasing(&numbers)
        })
        .map(|s| s.to_string())
        .collect()
}

fn keep_only_lines_with_max_3_absolute_diff_between_each_number(lines: Vec<String>) -> Vec<String> {
    lines
        .iter()
        .filter(|line| {
            let numbers: Vec<u64> = line
                .split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect();

            is_max_3_diff_between_each_number(&numbers)
        })
        .map(|s| s.to_string())
        .collect()
}

fn transform_lines_to_enumerated_variants(lines: Vec<String>) -> Vec<HashMap<usize, Vec<u64>>> {
    lines
        .iter()
        .map(|line| {
            let numbers: Vec<u64> = line
                .split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect();

            remove_each_number_from_vec_and_return_all_enumerated_variants(numbers)
        })
        .collect()
}

fn remove_each_number_from_vec_and_return_all_enumerated_variants(
    numbers: Vec<u64>,
) -> HashMap<usize, Vec<u64>> {
    let mut numbers_variants = HashMap::new();
    for i in 0..numbers.len() {
        let mut numbers_variant = numbers.clone();
        numbers_variant.remove(i);
        numbers_variants.insert(i, numbers_variant);
    }

    numbers_variants
}

fn keep_only_variants_with_increasing_or_decreasing_numbers(
    lines: Vec<HashMap<usize, Vec<u64>>>,
) -> Vec<HashMap<usize, Vec<u64>>> {
    let mut new_lines = Vec::with_capacity(lines.len());
    for line in lines {
        let mut new_variants = HashMap::new();
        for (i, numbers) in line {
            if is_all_increasing_or_decreasing(&numbers) {
                new_variants.insert(i, numbers);
            }
        }
        if new_variants.len() > 0 {
            new_lines.push(new_variants);
        }
    }
    new_lines
}

fn keep_only_variants_with_max_3_absolute_diff_between_each_number(
    lines: Vec<HashMap<usize, Vec<u64>>>,
) -> Vec<HashMap<usize, Vec<u64>>> {
    let mut new_lines = Vec::with_capacity(lines.len());
    for line in lines {
        let mut new_variants = HashMap::new();
        for (i, numbers) in line {
            if is_max_3_diff_between_each_number(&numbers) {
                new_variants.insert(i, numbers);
            }
        }
        if new_variants.len() > 0 {
            new_lines.push(new_variants);
        }
    }
    new_lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let lines = vec![
            "7 6 4 2 1",
            "1 2 7 8 9",
            "9 7 6 2 1",
            "1 3 2 4 5",
            "8 6 4 4 1",
            "1 3 6 7 9",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        assert_eq!(part1(lines), "2".to_string());
    }

    #[test]
    fn part2_example() {
        let lines = vec![
            "7 6 4 2 1",
            "1 2 7 8 9",
            "9 7 6 2 1",
            "1 3 2 4 5",
            "8 6 4 4 1",
            "1 3 6 7 9",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        assert_eq!(part2(lines), "4".to_string());
    }
}
