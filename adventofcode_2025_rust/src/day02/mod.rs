pub fn exec(lines: Vec<String>) {
    println!("[day02][part1] = {}", part1(lines.clone()));
    println!("[day02][part2] = {}", part2(lines));
}

pub fn part1(lines: Vec<String>) -> String {
    let mut added_invalid_ids: u128 = 0;
    for line in lines {
        let parts: Vec<&str> = line.split('-').collect();
        let start: u128 = parts[0].parse().unwrap();
        let end: u128 = parts[1].parse().unwrap();

        for id in start..=end {
            let id_as_string = id.to_string();
            let how_many_digits_in_id = id_as_string.len();
            let first_half: String = id_as_string[..how_many_digits_in_id / 2].to_string();
            let second_half: String = id_as_string[how_many_digits_in_id / 2..].to_string();
            if first_half == second_half {
                added_invalid_ids += id;
            }
        }
    }

    added_invalid_ids.to_string()
}

pub fn part2(lines: Vec<String>) -> String {
    let mut added_invalid_ids: u128 = 0;
    for line in lines {
        let parts: Vec<&str> = line.split('-').collect();
        let start: u128 = parts[0].parse().unwrap();
        let end: u128 = parts[1].parse().unwrap();

        for id in start..=end {
            let id_as_string = id.to_string();
            let how_many_digits_in_id = id_as_string.len();

            // handle 2 repeat
            if how_many_digits_in_id < 2 {
                continue;
            }
            let first_half: String = id_as_string[..how_many_digits_in_id / 2].to_string();
            let second_half: String = id_as_string[how_many_digits_in_id / 2..].to_string();
            if first_half == second_half {
                added_invalid_ids += id;
                continue;
            }

            // handle 3 repeat
            if how_many_digits_in_id < 3 {
                continue;
            }
            let part_size = how_many_digits_in_id / 3;
            let first_part: String = id_as_string[..part_size].to_string();
            let second_part: String = id_as_string[part_size..part_size * 2].to_string();
            let third_part: String = id_as_string[part_size * 2..].to_string();
            if first_part == second_part && second_part == third_part {
                added_invalid_ids += id;
                continue;
            }

            // handle 4 repeat
            if how_many_digits_in_id < 4 {
                continue;
            }
            let part_size = how_many_digits_in_id / 4;
            let first_part: String = id_as_string[..part_size].to_string();
            let second_part: String = id_as_string[part_size..part_size * 2].to_string();
            let third_part: String = id_as_string[part_size * 2..part_size * 3].to_string();
            let fourth_part: String = id_as_string[part_size * 3..].to_string();
            if first_part == second_part && second_part == third_part && third_part == fourth_part {
                added_invalid_ids += id;
                continue;
            }

            // handle 5 repeat
            if how_many_digits_in_id < 5 {
                continue;
            }
            let part_size = how_many_digits_in_id / 5;
            let first_part: String = id_as_string[..part_size].to_string();
            let second_part: String = id_as_string[part_size..part_size * 2].to_string();
            let third_part: String = id_as_string[part_size * 2..part_size * 3].to_string();
            let fourth_part: String = id_as_string[part_size * 3..part_size * 4].to_string();
            let fifth_part: String = id_as_string[part_size * 4..].to_string();
            if first_part == second_part
                && second_part == third_part
                && third_part == fourth_part
                && fourth_part == fifth_part
            {
                added_invalid_ids += id;
                continue;
            }

            // handle 6 repeat
            if how_many_digits_in_id < 6 {
                continue;
            }
            let part_size = how_many_digits_in_id / 6;
            let first_part: String = id_as_string[..part_size].to_string();
            let second_part: String = id_as_string[part_size..part_size * 2].to_string();
            let third_part: String = id_as_string[part_size * 2..part_size * 3].to_string();
            let fourth_part: String = id_as_string[part_size * 3..part_size * 4].to_string();
            let fifth_part: String = id_as_string[part_size * 4..part_size * 5].to_string();
            let sixth_part: String = id_as_string[part_size * 5..].to_string();
            if first_part == second_part
                && second_part == third_part
                && third_part == fourth_part
                && fourth_part == fifth_part
                && fifth_part == sixth_part
            {
                added_invalid_ids += id;
                continue;
            }

            // handle 7 repeat
            if how_many_digits_in_id < 7 {
                continue;
            }
            let part_size = how_many_digits_in_id / 7;
            let first_part: String = id_as_string[..part_size].to_string();
            let second_part: String = id_as_string[part_size..part_size * 2].to_string();
            let third_part: String = id_as_string[part_size * 2..part_size * 3].to_string();
            let fourth_part: String = id_as_string[part_size * 3..part_size * 4].to_string();
            let fifth_part: String = id_as_string[part_size * 4..part_size * 5].to_string();
            let sixth_part: String = id_as_string[part_size * 5..part_size * 6].to_string();
            let seventh_part: String = id_as_string[part_size * 6..].to_string();
            if first_part == second_part
                && second_part == third_part
                && third_part == fourth_part
                && fourth_part == fifth_part
                && fifth_part == sixth_part
                && sixth_part == seventh_part
            {
                added_invalid_ids += id;
                continue;
            }

            // handle 8 repeat
            if how_many_digits_in_id < 8 {
                continue;
            }
            let part_size = how_many_digits_in_id / 8;
            let first_part: String = id_as_string[..part_size].to_string();
            let second_part: String = id_as_string[part_size..part_size * 2].to_string();
            let third_part: String = id_as_string[part_size * 2..part_size * 3].to_string();
            let fourth_part: String = id_as_string[part_size * 3..part_size * 4].to_string();
            let fifth_part: String = id_as_string[part_size * 4..part_size * 5].to_string();
            let sixth_part: String = id_as_string[part_size * 5..part_size * 6].to_string();
            let seventh_part: String = id_as_string[part_size * 6..part_size * 7].to_string();
            let eighth_part: String = id_as_string[part_size * 7..].to_string();
            if first_part == second_part
                && second_part == third_part
                && third_part == fourth_part
                && fourth_part == fifth_part
                && fifth_part == sixth_part
                && sixth_part == seventh_part
                && seventh_part == eighth_part
            {
                added_invalid_ids += id;
                continue;
            }

            // handle 9 repeat
            if how_many_digits_in_id < 9 {
                continue;
            }
            let part_size = how_many_digits_in_id / 9;
            let first_part: String = id_as_string[..part_size].to_string();
            let second_part: String = id_as_string[part_size..part_size * 2].to_string();
            let third_part: String = id_as_string[part_size * 2..part_size * 3].to_string();
            let fourth_part: String = id_as_string[part_size * 3..part_size * 4].to_string();
            let fifth_part: String = id_as_string[part_size * 4..part_size * 5].to_string();
            let sixth_part: String = id_as_string[part_size * 5..part_size * 6].to_string();
            let seventh_part: String = id_as_string[part_size * 6..part_size * 7].to_string();
            let eighth_part: String = id_as_string[part_size * 7..part_size * 8].to_string();
            let ninth_part: String = id_as_string[part_size * 8..].to_string();
            if first_part == second_part
                && second_part == third_part
                && third_part == fourth_part
                && fourth_part == fifth_part
                && fifth_part == sixth_part
                && sixth_part == seventh_part
                && seventh_part == eighth_part
                && eighth_part == ninth_part
            {
                added_invalid_ids += id;
                continue;
            }

            // handle 10 repeat
            if how_many_digits_in_id < 10 {
                continue;
            }
            let part_size = how_many_digits_in_id / 10;
            let first_part: String = id_as_string[..part_size].to_string();
            let second_part: String = id_as_string[part_size..part_size * 2].to_string();
            let third_part: String = id_as_string[part_size * 2..part_size * 3].to_string();
            let fourth_part: String = id_as_string[part_size * 3..part_size * 4].to_string();
            let fifth_part: String = id_as_string[part_size * 4..part_size * 5].to_string();
            let sixth_part: String = id_as_string[part_size * 5..part_size * 6].to_string();
            let seventh_part: String = id_as_string[part_size * 6..part_size * 7].to_string();
            let eighth_part: String = id_as_string[part_size * 7..part_size * 8].to_string();
            let ninth_part: String = id_as_string[part_size * 8..part_size * 9].to_string();
            let tenth_part: String = id_as_string[part_size * 9..].to_string();
            if first_part == second_part
                && second_part == third_part
                && third_part == fourth_part
                && fourth_part == fifth_part
                && fifth_part == sixth_part
                && sixth_part == seventh_part
                && seventh_part == eighth_part
                && eighth_part == ninth_part
                && ninth_part == tenth_part
            {
                added_invalid_ids += id;
                continue;
            }
        }
    }

    added_invalid_ids.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let lines = vec![
            "11-22",
            "95-115",
            "998-1012",
            "1188511880-1188511890",
            "222220-222224",
            "1698522-1698528",
            "446443-446449",
            "38593856-38593862",
            "565653-565659",
            "824824821-824824827",
            "2121212118-2121212124",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        assert_eq!(part1(lines), "1227775554".to_string());
    }

    #[test]
    fn part2_example() {
        let lines = vec![
            "11-22",
            "95-115",
            "998-1012",
            "1188511880-1188511890",
            "222220-222224",
            "1698522-1698528",
            "446443-446449",
            "38593856-38593862",
            "565653-565659",
            "824824821-824824827",
            "2121212118-2121212124",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        assert_eq!(part2(lines), "4174379265".to_string());
    }
}
