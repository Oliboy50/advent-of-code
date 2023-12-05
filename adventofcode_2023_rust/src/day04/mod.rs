use regex::Regex;

pub fn exec(lines: Vec<String>) {
    println!("[day04][part1] = {}", part1(lines.clone()));
    println!("[day04][part2] = {}", part2(lines));
}

fn part1(lines: Vec<String>) -> String {
    lines
        .iter()
        .map(|line| get_card_from_line(line))
        .map(|card| get_numbers_that_are_in_both_lists(&card.winning_numbers, &card.player_numbers))
        .map(compute_points_for_numbers)
        .sum::<u64>()
        .to_string()
}

fn part2(lines: Vec<String>) -> String {
    let original_cards = lines
        .iter()
        .map(|line| get_card_from_line(line))
        .collect::<Vec<Card>>();

    part2_recursive_number_of_cards_won(&original_cards, original_cards.as_slice()).to_string()
}

fn part2_recursive_number_of_cards_won(original_cards: &[Card], cards: &[Card]) -> u64 {
    let mut number_of_cards = cards.len() as u64;

    for card in cards {
        let number_of_next_cards_to_win =
            get_numbers_that_are_in_both_lists(&card.winning_numbers, &card.player_numbers).len()
                as u16;

        for i in 1..=number_of_next_cards_to_win {
            let next_card_id = card.id + i;
            let Some(next_card) = original_cards.iter().find(|card| card.id == next_card_id) else {
                continue;
            };

            number_of_cards +=
                part2_recursive_number_of_cards_won(original_cards, &[next_card.clone()]);
        }
    }

    number_of_cards
}

#[derive(Debug, Clone, PartialEq)]
struct Card {
    id: u16,
    winning_numbers: Vec<u16>,
    player_numbers: Vec<u16>,
}

fn get_card_from_line(line: &str) -> Card {
    let mut winning_numbers = Vec::new();
    let mut player_numbers = Vec::new();

    let regex_for_line = Regex::new(
        r"^Card +(?<id>\d+): (?<winning_numbers>[ 0-9]+) \| (?<player_numbers>[ 0-9]+)$",
    )
    .unwrap();
    let regex_for_numbers = Regex::new(r" ?\d+").unwrap();

    let line_captures = regex_for_line.captures(line).unwrap();
    let id = line_captures
        .name("id")
        .unwrap()
        .as_str()
        .parse::<u16>()
        .unwrap();

    for captures in
        regex_for_numbers.captures_iter(line_captures.name("winning_numbers").unwrap().as_str())
    {
        captures.iter().for_each(|capture| {
            winning_numbers.push(capture.unwrap().as_str().trim().parse::<u16>().unwrap());
        });
    }

    for captures in
        regex_for_numbers.captures_iter(line_captures.name("player_numbers").unwrap().as_str())
    {
        captures.iter().for_each(|capture| {
            player_numbers.push(capture.unwrap().as_str().trim().parse::<u16>().unwrap());
        });
    }

    Card {
        id,
        winning_numbers,
        player_numbers,
    }
}

fn get_numbers_that_are_in_both_lists(list1: &[u16], list2: &[u16]) -> Vec<u16> {
    list2
        .iter()
        .filter(|n| list1.contains(n))
        .map(Clone::clone)
        .collect()
}

fn compute_points_for_numbers(numbers: Vec<u16>) -> u64 {
    numbers
        .iter()
        .enumerate()
        .fold(0, |acc, (i, _)| if i == 0 { acc + 1 } else { acc * 2 })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let lines = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(part1(lines), "13");
    }

    #[test]
    fn get_card_from_line_success() {
        assert_eq!(
            get_card_from_line("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            Card {
                id: 1,
                winning_numbers: vec![41, 48, 83, 86, 17],
                player_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
            }
        );
    }

    #[test]
    fn get_numbers_that_are_in_both_lists_success() {
        assert_eq!(
            get_numbers_that_are_in_both_lists(
                &[41, 48, 83, 86, 17],
                &[83, 86, 6, 31, 17, 9, 48, 53]
            ),
            vec![83, 86, 17, 48]
        );
    }

    #[test]
    fn compute_points_for_numbers_success() {
        assert_eq!(compute_points_for_numbers(vec![83, 86, 17, 48]), 8);
    }

    #[test]
    fn part2_example() {
        let lines = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(part2(lines), "30");
    }
}
