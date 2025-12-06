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
        let (first_highest_digit_index, first_highest_digit) = digits
            .iter()
            .enumerate()
            .filter(|(i, _)| *i < digits.len() - 1)
            .fold((0, 0), fold_first_highest_digit);
        let (_, second_highest_digit) = digits
            .iter()
            .enumerate()
            .filter(|(i, _)| *i > first_highest_digit_index)
            .fold((0, 0), fold_first_highest_digit);

        total += format!("{first_highest_digit}{second_highest_digit}")
            .parse::<u32>()
            .unwrap();
    }

    total.to_string()
}

fn fold_first_highest_digit(
    (acc_index, acc_digit): (usize, u8),
    (i, digit): (usize, &u8),
) -> (usize, u8) {
    if *digit > acc_digit {
        (i, *digit)
    } else {
        (acc_index, acc_digit)
    }
}

pub fn part2(lines: Vec<String>) -> String {
    let mut total: u128 = 0;
    for line in lines {
        let digits: Vec<u8> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        let (highest_digit_1_index, highest_digit_1) = digits
            .iter()
            .enumerate()
            .filter(|&(i, _)| i < digits.len() - 11)
            .fold((0, 0), fold_first_highest_digit);
        let (highest_digit_2_index, highest_digit_2) = digits
            .iter()
            .enumerate()
            .filter(|&(i, _)| i > highest_digit_1_index && i < digits.len() - 10)
            .fold((0, 0), fold_first_highest_digit);
        let (highest_digit_3_index, highest_digit_3) = digits
            .iter()
            .enumerate()
            .filter(|&(i, _)| i > highest_digit_2_index && i < digits.len() - 9)
            .fold((0, 0), fold_first_highest_digit);
        let (highest_digit_4_index, highest_digit_4) = digits
            .iter()
            .enumerate()
            .filter(|&(i, _)| i > highest_digit_3_index && i < digits.len() - 8)
            .fold((0, 0), fold_first_highest_digit);
        let (highest_digit_5_index, highest_digit_5) = digits
            .iter()
            .enumerate()
            .filter(|&(i, _)| i > highest_digit_4_index && i < digits.len() - 7)
            .fold((0, 0), fold_first_highest_digit);
        let (highest_digit_6_index, highest_digit_6) = digits
            .iter()
            .enumerate()
            .filter(|&(i, _)| i > highest_digit_5_index && i < digits.len() - 6)
            .fold((0, 0), fold_first_highest_digit);
        let (highest_digit_7_index, highest_digit_7) = digits
            .iter()
            .enumerate()
            .filter(|&(i, _)| i > highest_digit_6_index && i < digits.len() - 5)
            .fold((0, 0), fold_first_highest_digit);
        let (highest_digit_8_index, highest_digit_8) = digits
            .iter()
            .enumerate()
            .filter(|&(i, _)| i > highest_digit_7_index && i < digits.len() - 4)
            .fold((0, 0), fold_first_highest_digit);
        let (highest_digit_9_index, highest_digit_9) = digits
            .iter()
            .enumerate()
            .filter(|&(i, _)| i > highest_digit_8_index && i < digits.len() - 3)
            .fold((0, 0), fold_first_highest_digit);
        let (highest_digit_10_index, highest_digit_10) = digits
            .iter()
            .enumerate()
            .filter(|&(i, _)| i > highest_digit_9_index && i < digits.len() - 2)
            .fold((0, 0), fold_first_highest_digit);
        let (highest_digit_11_index, highest_digit_11) = digits
            .iter()
            .enumerate()
            .filter(|&(i, _)| i > highest_digit_10_index && i < digits.len() - 1)
            .fold((0, 0), fold_first_highest_digit);
        let (_, highest_digit_12) = digits
            .iter()
            .enumerate()
            .filter(|&(i, _)| i > highest_digit_11_index)
            .fold((0, 0), fold_first_highest_digit);

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
