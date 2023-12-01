use std::fmt::Display;

pub fn exec(lines: Vec<String>) {
    println!("[day3][part1] = {}", part1(lines.clone()));
    println!("[day3][part2] = {}", part2(lines));
}

fn part1(lines: Vec<String>) -> impl Display {
    let mut gamma_as_array: [&str; 12] =
        ["0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0"];
    for i in 0..=11 {
        if get_most_common_bit_in_column(&lines, i) == 1 {
            gamma_as_array[i] = "1";
        }
    }

    let gamma = i32::from_str_radix(gamma_as_array.join("").as_str(), 2).unwrap();
    let epsilon = gamma ^ 0b111111111111;

    format!("{}", gamma * epsilon)
}

fn part2(lines: Vec<String>) -> impl Display {
    format!(
        "{}",
        get_rating(lines.clone(), true) * get_rating(lines, false)
    )
}

fn get_rating(lines: Vec<String>, oxygen_rating: bool) -> i32 {
    let mut remaining_lines: Vec<String> = lines;

    for i in 0..=11 {
        let bit_to_keep = match oxygen_rating {
            true => get_most_common_bit_in_column(&remaining_lines, i),
            false => get_least_common_bit_in_column(&remaining_lines, i),
        };
        remaining_lines = keep_lines_if_column_contains_bit(&remaining_lines, i, bit_to_keep);

        if remaining_lines.len() == 1 {
            break;
        }
    }

    i32::from_str_radix(remaining_lines.first().unwrap(), 2).unwrap()
}

fn get_most_common_bit_in_column(lines: &Vec<String>, col_index: usize) -> i32 {
    let number_of_1_in_the_column =
        get_bits_in_column(&lines, col_index).iter().sum::<i32>() as f64;
    let half_lines_length = lines.len() as f64 / 2f64;

    if number_of_1_in_the_column >= half_lines_length {
        1
    } else {
        0
    }
}

fn get_least_common_bit_in_column(lines: &Vec<String>, col_index: usize) -> i32 {
    let number_of_1_in_the_column =
        get_bits_in_column(&lines, col_index).iter().sum::<i32>() as f64;
    let half_lines_length = lines.len() as f64 / 2f64;

    if number_of_1_in_the_column >= half_lines_length {
        0
    } else {
        1
    }
}

fn get_bits_in_column(lines: &Vec<String>, col_index: usize) -> Vec<i32> {
    lines
        .iter()
        .map(|l| l.chars().nth(col_index).unwrap().to_string())
        .map(|bit| bit.parse::<i32>().unwrap())
        .collect()
}

fn keep_lines_if_column_contains_bit(
    lines: &Vec<String>,
    col_index: usize,
    bit: i32,
) -> Vec<String> {
    let filtered_lines: Vec<String> = lines
        .iter()
        .filter(|l| {
            l.chars()
                .nth(col_index)
                .unwrap()
                .to_string()
                .parse::<i32>()
                .unwrap()
                == bit
        })
        .map(|l| l.to_owned())
        .collect();

    if filtered_lines.len() > 0 {
        filtered_lines
    } else {
        lines.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exec_part1_with_sample() {
        assert_eq!(
            format!("{}", 3601664),
            format!(
                "{}",
                part1(vec![
                    "001000000000".to_string(),
                    "111100000000".to_string(),
                    "101100000000".to_string(),
                    "101110000000".to_string(),
                    "101010000000".to_string(),
                    "011110000000".to_string(),
                    "001110000000".to_string(),
                    "111000000000".to_string(),
                    "100000000000".to_string(),
                    "110010000000".to_string(),
                    "000100000000".to_string(),
                    "010100000000".to_string(),
                ])
            )
        );
    }

    #[test]
    fn oxygen_rating_with_sample() {
        assert_eq!(
            0b111101000010,
            get_rating(
                vec![
                    "001000000010".to_string(),
                    "001001000000".to_string(),
                    "111101000000".to_string(),
                    "111101000010".to_string(),
                ],
                true
            )
        );
    }

    #[test]
    fn co2_rating_with_sample() {
        assert_eq!(
            0b001000000010,
            get_rating(
                vec![
                    "001000000010".to_string(),
                    "001001000000".to_string(),
                    "111101000000".to_string(),
                    "111101000000".to_string(),
                ],
                false
            )
        );
    }

    #[test]
    fn co2_rating_with_sample_2() {
        assert_eq!(
            0b110101001110,
            get_rating(
                vec![
                    "110101001001".to_string(),
                    "110101001011".to_string(),
                    "110101001110".to_string(),
                ],
                false
            )
        );
    }
}
