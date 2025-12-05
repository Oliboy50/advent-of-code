pub fn exec(lines: Vec<String>) {
    println!("[day03][part1] = {}", part1(lines.clone()));
    println!("[day03][part2] = {}", part2(lines));
}

pub fn part1(lines: Vec<String>) -> String {
    let mut total: u32 = 0;
    for line in lines {
        let digits: Vec<u8> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        let mut first_highest_digit = 0;
        let mut first_highest_digit_index = 0;
        let mut second_highest_digit = 0;
        // get first highest digit
        for (i, digit) in digits.iter().enumerate() {
            if first_highest_digit < *digit && i < digits.len() - 1 {
                first_highest_digit = *digit;
                first_highest_digit_index = i;
            }
        }
        // get second-highest digit
        for (i, digit) in digits.iter().enumerate() {
            if i <= first_highest_digit_index {
                continue;
            }
            if second_highest_digit < *digit {
                second_highest_digit = *digit;
            }
        }

        total += format!("{first_highest_digit}{second_highest_digit}")
            .parse::<u32>()
            .unwrap();
    }

    total.to_string()
}

pub fn part2(lines: Vec<String>) -> String {
    let mut total: u128 = 0;
    for line in lines {
        let digits: Vec<u8> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        let mut highest_digit_1 = 0;
        let mut highest_digit_1_index = 0;
        let mut highest_digit_2 = 0;
        let mut highest_digit_2_index = 0;
        let mut highest_digit_3 = 0;
        let mut highest_digit_3_index = 0;
        let mut highest_digit_4 = 0;
        let mut highest_digit_4_index = 0;
        let mut highest_digit_5 = 0;
        let mut highest_digit_5_index = 0;
        let mut highest_digit_6 = 0;
        let mut highest_digit_6_index = 0;
        let mut highest_digit_7 = 0;
        let mut highest_digit_7_index = 0;
        let mut highest_digit_8 = 0;
        let mut highest_digit_8_index = 0;
        let mut highest_digit_9 = 0;
        let mut highest_digit_9_index = 0;
        let mut highest_digit_10 = 0;
        let mut highest_digit_10_index = 0;
        let mut highest_digit_11 = 0;
        let mut highest_digit_11_index = 0;
        let mut highest_digit_12 = 0;
        for (i, digit) in digits.iter().enumerate() {
            if highest_digit_1 < *digit && i < digits.len() - 11 {
                highest_digit_1 = *digit;
                highest_digit_1_index = i;
            }
        }
        for (i, digit) in digits.iter().enumerate() {
            if i <= highest_digit_1_index {
                continue;
            }
            if highest_digit_2 < *digit && i < digits.len() - 10 {
                highest_digit_2 = *digit;
                highest_digit_2_index = i;
            }
        }
        for (i, digit) in digits.iter().enumerate() {
            if i <= highest_digit_2_index {
                continue;
            }
            if highest_digit_3 < *digit && i < digits.len() - 9 {
                highest_digit_3 = *digit;
                highest_digit_3_index = i;
            }
        }
        for (i, digit) in digits.iter().enumerate() {
            if i <= highest_digit_3_index {
                continue;
            }
            if highest_digit_4 < *digit && i < digits.len() - 8 {
                highest_digit_4 = *digit;
                highest_digit_4_index = i;
            }
        }
        for (i, digit) in digits.iter().enumerate() {
            if i <= highest_digit_4_index {
                continue;
            }
            if highest_digit_5 < *digit && i < digits.len() - 7 {
                highest_digit_5 = *digit;
                highest_digit_5_index = i;
            }
        }
        for (i, digit) in digits.iter().enumerate() {
            if i <= highest_digit_5_index {
                continue;
            }
            if highest_digit_6 < *digit && i < digits.len() - 6 {
                highest_digit_6 = *digit;
                highest_digit_6_index = i;
            }
        }
        for (i, digit) in digits.iter().enumerate() {
            if i <= highest_digit_6_index {
                continue;
            }
            if highest_digit_7 < *digit && i < digits.len() - 5 {
                highest_digit_7 = *digit;
                highest_digit_7_index = i;
            }
        }
        for (i, digit) in digits.iter().enumerate() {
            if i <= highest_digit_7_index {
                continue;
            }
            if highest_digit_8 < *digit && i < digits.len() - 4 {
                highest_digit_8 = *digit;
                highest_digit_8_index = i;
            }
        }
        for (i, digit) in digits.iter().enumerate() {
            if i <= highest_digit_8_index {
                continue;
            }
            if highest_digit_9 < *digit && i < digits.len() - 3 {
                highest_digit_9 = *digit;
                highest_digit_9_index = i;
            }
        }
        for (i, digit) in digits.iter().enumerate() {
            if i <= highest_digit_9_index {
                continue;
            }
            if highest_digit_10 < *digit && i < digits.len() - 2 {
                highest_digit_10 = *digit;
                highest_digit_10_index = i;
            }
        }
        for (i, digit) in digits.iter().enumerate() {
            if i <= highest_digit_10_index {
                continue;
            }
            if highest_digit_11 < *digit && i < digits.len() - 1 {
                highest_digit_11 = *digit;
                highest_digit_11_index = i;
            }
        }
        for (i, digit) in digits.iter().enumerate() {
            if i <= highest_digit_11_index {
                continue;
            }
            if highest_digit_12 < *digit {
                highest_digit_12 = *digit;
            }
        }

        total += format!("{highest_digit_1}{highest_digit_2}{highest_digit_3}{highest_digit_4}{highest_digit_5}{highest_digit_6}{highest_digit_7}{highest_digit_8}{highest_digit_9}{highest_digit_10}{highest_digit_11}{highest_digit_12}")
            .parse::<u128>()
            .unwrap();
    }

    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let lines = vec![
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        assert_eq!(part1(lines), "357".to_string());
    }

    #[test]
    fn part2_example() {
        let lines = vec![
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        assert_eq!(part2(lines), "3121910778619".to_string());
    }
}
