use std::cmp::Ordering;
use std::fmt::Debug;

pub fn exec(lines: Vec<String>) {
    println!("[day07][part1] = {}", part1(lines.clone()));
    println!("[day07][part2] = {}", part2(lines));
}

fn part1(lines: Vec<String>) -> String {
    let mut hands_with_bids = get_hands_with_bids(&lines);
    hands_with_bids.sort();

    hands_with_bids
        .into_iter()
        .enumerate()
        .map(|(i, hand_with_bid)| (i + 1) as u64 * hand_with_bid.get_bid() as u64)
        .sum::<u64>()
        .to_string()
}

fn part2(lines: Vec<String>) -> String {
    let mut hands_with_bids = get_hands_with_bids(&lines);
    hands_with_bids.sort_by(sort_hands_with_bids_for_part2);

    hands_with_bids
        .into_iter()
        .enumerate()
        .map(|(i, hand_with_bid)| (i + 1) as u64 * hand_with_bid.get_bid() as u64)
        .sum::<u64>()
        .to_string()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord)]
enum Card {
    As,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_card_value = self.get_weight_for_sorting();
        let other_card_value = other.get_weight_for_sorting();

        if self_card_value > other_card_value {
            Some(Ordering::Greater)
        } else if self_card_value < other_card_value {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl Card {
    fn get_weight_for_sorting(&self) -> u8 {
        match self {
            Card::As => 14,
            Card::King => 13,
            Card::Queen => 12,
            Card::Jack => 11,
            Card::Ten => 10,
            Card::Nine => 9,
            Card::Eight => 8,
            Card::Seven => 7,
            Card::Six => 6,
            Card::Five => 5,
            Card::Four => 4,
            Card::Three => 3,
            Card::Two => 2,
        }
    }

    fn get_weight_for_sorting_for_part2(&self) -> u8 {
        match self {
            Card::As => 14,
            Card::King => 13,
            Card::Queen => 12,
            Card::Ten => 10,
            Card::Nine => 9,
            Card::Eight => 8,
            Card::Seven => 7,
            Card::Six => 6,
            Card::Five => 5,
            Card::Four => 4,
            Card::Three => 3,
            Card::Two => 2,
            Card::Jack => 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_value = self.get_weight_for_sorting();
        let other_value = other.get_weight_for_sorting();

        if self_value > other_value {
            Some(Ordering::Greater)
        } else if self_value < other_value {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl HandType {
    fn get_weight_for_sorting(&self) -> u8 {
        match self {
            HandType::FiveOfAKind => 7,
            HandType::FourOfAKind => 6,
            HandType::FullHouse => 5,
            HandType::ThreeOfAKind => 4,
            HandType::TwoPair => 3,
            HandType::OnePair => 2,
            HandType::HighCard => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord)]
struct Hand((Card, Card, Card, Card, Card));

impl FromIterator<Card> for Hand {
    fn from_iter<T: IntoIterator<Item = Card>>(iter: T) -> Self {
        let mut cards = iter.into_iter();
        Hand((
            cards.next().unwrap(),
            cards.next().unwrap(),
            cards.next().unwrap(),
            cards.next().unwrap(),
            cards.next().unwrap(),
        ))
    }
}

impl IntoIterator for Hand {
    type Item = Card;
    type IntoIter = std::array::IntoIter<Self::Item, 5>;

    fn into_iter(self) -> Self::IntoIter {
        [self.0 .0, self.0 .1, self.0 .2, self.0 .3, self.0 .4].into_iter()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_type_value = self.get_type().get_weight_for_sorting();
        let other_type_value = other.get_type().get_weight_for_sorting();

        if self_type_value > other_type_value {
            return Some(Ordering::Greater);
        } else if self_type_value < other_type_value {
            return Some(Ordering::Less);
        }

        let self_cards = self.into_iter();
        let other_cards = other.into_iter();

        for (self_card, other_card) in self_cards.zip(other_cards) {
            if self_card > other_card {
                return Some(Ordering::Greater);
            } else if self_card < other_card {
                return Some(Ordering::Less);
            }
        }

        Some(Ordering::Equal)
    }
}

impl Hand {
    fn first_card(&self) -> Card {
        self.0 .0
    }
    fn second_card(&self) -> Card {
        self.0 .1
    }
    fn third_card(&self) -> Card {
        self.0 .2
    }
    fn fourth_card(&self) -> Card {
        self.0 .3
    }
    fn fifth_card(&self) -> Card {
        self.0 .4
    }

    fn get_type(&self) -> HandType {
        if self.is_five_of_a_kind() {
            HandType::FiveOfAKind
        } else if self.is_four_of_a_kind() {
            HandType::FourOfAKind
        } else if self.is_full_house() {
            HandType::FullHouse
        } else if self.is_three_of_a_kind() {
            HandType::ThreeOfAKind
        } else if self.is_two_pair() {
            HandType::TwoPair
        } else if self.is_one_pair() {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }

    fn get_type_for_part2(&self) -> HandType {
        if self.is_five_of_a_kind() || self.is_five_of_a_kind_with_joker() {
            HandType::FiveOfAKind
        } else if self.is_four_of_a_kind() || self.is_four_of_a_kind_with_joker() {
            HandType::FourOfAKind
        } else if self.is_full_house() || self.is_full_house_with_joker() {
            HandType::FullHouse
        } else if self.is_three_of_a_kind() || self.is_three_of_a_kind_with_joker() {
            HandType::ThreeOfAKind
        } else if self.is_two_pair() {
            HandType::TwoPair
        } else if self.is_one_pair() || self.is_one_pair_with_joker() {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }

    fn get_number_of_jacks(&self) -> u8 {
        self.into_iter().fold(0, |acc, c| {
            if matches!(c, Card::Jack) {
                acc + 1
            } else {
                acc
            }
        })
    }

    fn is_five_of_a_kind(&self) -> bool {
        self.first_card() == self.second_card()
            && self.second_card() == self.third_card()
            && self.third_card() == self.fourth_card()
            && self.fourth_card() == self.fifth_card()
    }

    fn is_four_of_a_kind(&self) -> bool {
        (self.first_card() == self.second_card()
            && self.second_card() == self.third_card()
            && self.third_card() == self.fourth_card())
            || (self.first_card() == self.second_card()
                && self.second_card() == self.third_card()
                && self.third_card() == self.fifth_card())
            || (self.first_card() == self.second_card()
                && self.second_card() == self.fourth_card()
                && self.fourth_card() == self.fifth_card())
            || (self.first_card() == self.third_card()
                && self.third_card() == self.fourth_card()
                && self.fourth_card() == self.fifth_card())
            || (self.second_card() == self.third_card()
                && self.third_card() == self.fourth_card()
                && self.fourth_card() == self.fifth_card())
    }

    fn is_full_house(&self) -> bool {
        (self.first_card() == self.second_card()
            && self.second_card() == self.third_card()
            && self.fourth_card() == self.fifth_card())
            || (self.first_card() == self.second_card()
                && self.second_card() == self.fourth_card()
                && self.third_card() == self.fifth_card())
            || (self.first_card() == self.second_card()
                && self.second_card() == self.fifth_card()
                && self.third_card() == self.fourth_card())
            || (self.first_card() == self.third_card()
                && self.third_card() == self.fourth_card()
                && self.second_card() == self.fifth_card())
            || (self.first_card() == self.third_card()
                && self.third_card() == self.fifth_card()
                && self.second_card() == self.fourth_card())
            || (self.first_card() == self.fourth_card()
                && self.fourth_card() == self.fifth_card()
                && self.second_card() == self.third_card())
            || (self.second_card() == self.third_card()
                && self.third_card() == self.fourth_card()
                && self.first_card() == self.fifth_card())
            || (self.second_card() == self.third_card()
                && self.third_card() == self.fifth_card()
                && self.first_card() == self.fourth_card())
            || (self.second_card() == self.fourth_card()
                && self.fourth_card() == self.fifth_card()
                && self.first_card() == self.third_card())
            || (self.third_card() == self.fourth_card()
                && self.fourth_card() == self.fifth_card()
                && self.first_card() == self.second_card())
    }

    fn is_three_of_a_kind(&self) -> bool {
        (self.first_card() == self.second_card() && self.second_card() == self.third_card())
            || (self.first_card() == self.second_card() && self.second_card() == self.fourth_card())
            || (self.first_card() == self.second_card() && self.second_card() == self.fifth_card())
            || (self.first_card() == self.third_card() && self.third_card() == self.fourth_card())
            || (self.first_card() == self.third_card() && self.third_card() == self.fifth_card())
            || (self.first_card() == self.fourth_card() && self.fourth_card() == self.fifth_card())
            || (self.second_card() == self.third_card() && self.third_card() == self.fourth_card())
            || (self.second_card() == self.third_card() && self.third_card() == self.fifth_card())
            || (self.second_card() == self.fourth_card() && self.fourth_card() == self.fifth_card())
            || (self.third_card() == self.fourth_card() && self.fourth_card() == self.fifth_card())
    }

    fn is_two_pair(&self) -> bool {
        (self.first_card() == self.second_card() && self.third_card() == self.fourth_card())
            || (self.first_card() == self.second_card() && self.third_card() == self.fifth_card())
            || (self.first_card() == self.second_card() && self.fourth_card() == self.fifth_card())
            || (self.first_card() == self.third_card() && self.second_card() == self.fourth_card())
            || (self.first_card() == self.third_card() && self.second_card() == self.fifth_card())
            || (self.first_card() == self.third_card() && self.fourth_card() == self.fifth_card())
            || (self.first_card() == self.fourth_card() && self.second_card() == self.third_card())
            || (self.first_card() == self.fourth_card() && self.second_card() == self.fifth_card())
            || (self.first_card() == self.fourth_card() && self.third_card() == self.fifth_card())
            || (self.first_card() == self.fifth_card() && self.second_card() == self.third_card())
            || (self.first_card() == self.fifth_card() && self.second_card() == self.fourth_card())
            || (self.first_card() == self.fifth_card() && self.third_card() == self.fourth_card())
            || (self.second_card() == self.third_card() && self.fourth_card() == self.fifth_card())
            || (self.second_card() == self.fourth_card() && self.third_card() == self.fifth_card())
            || (self.second_card() == self.fifth_card() && self.third_card() == self.fourth_card())
    }

    fn is_one_pair(&self) -> bool {
        (self.first_card() == self.second_card())
            || (self.first_card() == self.third_card())
            || (self.first_card() == self.fourth_card())
            || (self.first_card() == self.fifth_card())
            || (self.second_card() == self.third_card())
            || (self.second_card() == self.fourth_card())
            || (self.second_card() == self.fifth_card())
            || (self.third_card() == self.fourth_card())
            || (self.third_card() == self.fifth_card())
            || (self.fourth_card() == self.fifth_card())
    }

    fn is_five_of_a_kind_with_joker(&self) -> bool {
        match self.get_number_of_jacks() {
            1 => self.is_four_of_a_kind(),
            2 | 3 => self.is_full_house(),
            4 => true,
            _ => false,
        }
    }

    fn is_four_of_a_kind_with_joker(&self) -> bool {
        match self.get_number_of_jacks() {
            1 => self.is_three_of_a_kind(),
            2 => self.is_two_pair(),
            3 => true,
            _ => false,
        }
    }

    fn is_full_house_with_joker(&self) -> bool {
        match self.get_number_of_jacks() {
            1 => self.is_two_pair(),
            _ => false,
        }
    }

    fn is_three_of_a_kind_with_joker(&self) -> bool {
        match self.get_number_of_jacks() {
            2 | 1 => self.is_one_pair(),
            _ => false,
        }
    }

    fn is_one_pair_with_joker(&self) -> bool {
        match self.get_number_of_jacks() {
            1 => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Ord)]
struct HandWithBid {
    hand: Hand,
    bid: u16,
}

impl PartialOrd for HandWithBid {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_hand = self.get_hand();
        let other_hand = other.get_hand();
        if self_hand > other_hand {
            Some(Ordering::Greater)
        } else if self_hand < other_hand {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl HandWithBid {
    fn get_hand(&self) -> Hand {
        self.hand
    }
    fn get_bid(&self) -> u16 {
        self.bid
    }
}

fn get_hands_with_bids(lines: &[String]) -> Vec<HandWithBid> {
    lines
        .iter()
        .map(|line| {
            let mut split = line.split_whitespace();
            let hand = split
                .next()
                .unwrap()
                .chars()
                .map(|c| match c {
                    'A' => Card::As,
                    'K' => Card::King,
                    'Q' => Card::Queen,
                    'J' => Card::Jack,
                    'T' => Card::Ten,
                    '9' => Card::Nine,
                    '8' => Card::Eight,
                    '7' => Card::Seven,
                    '6' => Card::Six,
                    '5' => Card::Five,
                    '4' => Card::Four,
                    '3' => Card::Three,
                    '2' => Card::Two,
                    _ => panic!("Unsupported card"),
                })
                .collect::<Hand>();
            let bid = split.next().unwrap().parse::<u16>().unwrap();

            HandWithBid { hand, bid }
        })
        .collect()
}

fn sort_hands_with_bids_for_part2(hand_with_bid: &HandWithBid, other: &HandWithBid) -> Ordering {
    let self_hand = hand_with_bid.get_hand();
    let other_hand = other.get_hand();

    let self_type_value = self_hand.get_type_for_part2().get_weight_for_sorting();
    let other_type_value = other_hand.get_type_for_part2().get_weight_for_sorting();

    if self_type_value > other_type_value {
        return Ordering::Greater;
    } else if self_type_value < other_type_value {
        return Ordering::Less;
    }

    let self_cards = self_hand.into_iter();
    let other_cards = other_hand.into_iter();

    for (self_card, other_card) in self_cards.zip(other_cards) {
        let self_card = self_card.get_weight_for_sorting_for_part2();
        let other_card = other_card.get_weight_for_sorting_for_part2();
        if self_card > other_card {
            return Ordering::Greater;
        } else if self_card < other_card {
            return Ordering::Less;
        }
    }

    Ordering::Equal
}

#[cfg(test)]
mod tests {
    use super::*;
    use Card::*;
    use HandType::*;

    #[test]
    fn part1_example() {
        let lines = vec![
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(part1(lines), "6440");
    }

    #[test]
    fn part1_more_cases_1() {
        let lines = vec![
            "88444 389", // full house
            "86866 534", // full house
            "29929 966", // full house
            "2JJJ5 280", // three kind
            "Q6996 762", // two pairs
            "7Q7J6 253", // one pair
            "QT2A4 841", // high card
            "7J9AT 722", // high card
            "69753 514", // high card
            "675KT 785", // high card
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(
            part1(lines),
            (785 * 1
                + 514 * 2
                + 722 * 3
                + 841 * 4
                + 253 * 5
                + 762 * 6
                + 280 * 7
                + 966 * 8
                + 534 * 9
                + 389 * 10)
                .to_string()
        );
    }

    #[test]
    fn get_hands_with_bids_success() {
        let lines = vec![
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
        assert_eq!(
            get_hands_with_bids(&lines),
            vec![
                HandWithBid {
                    hand: Hand((Three, Two, Ten, Three, King)),
                    bid: 765,
                },
                HandWithBid {
                    hand: Hand((Ten, Five, Five, Jack, Five)),
                    bid: 684,
                },
                HandWithBid {
                    hand: Hand((King, King, Six, Seven, Seven)),
                    bid: 28,
                },
                HandWithBid {
                    hand: Hand((King, Ten, Jack, Jack, Ten)),
                    bid: 220,
                },
                HandWithBid {
                    hand: Hand((Queen, Queen, Queen, Jack, As)),
                    bid: 483,
                },
            ]
        );
    }

    #[test]
    fn hand_get_type_success() {
        for (hand, expected) in [
            (Hand((Three, Two, Ten, Three, King)), OnePair),
            (Hand((Ten, Five, Five, Jack, Five)), ThreeOfAKind),
            (Hand((King, King, Six, Seven, Seven)), TwoPair),
            (Hand((King, Ten, Jack, Jack, Ten)), TwoPair),
            (Hand((Queen, Queen, Queen, Jack, As)), ThreeOfAKind),
        ] {
            assert_eq!(hand.get_type(), expected);
        }
    }

    #[test]
    fn hand_is_five_of_a_kind_success() {
        for (hand, expected) in [
            (Hand((Three, Three, Three, Three, Three)), true),
            (Hand((Three, Three, King, Three, Three)), false),
        ] {
            assert_eq!(hand.is_five_of_a_kind(), expected);
        }
    }

    #[test]
    fn hand_is_four_of_a_kind_success() {
        for (hand, expected) in [
            (Hand((Four, Three, Three, Three, Three)), true),
            (Hand((Three, Four, Three, Three, Three)), true),
            (Hand((Three, Three, Four, Three, Three)), true),
            (Hand((Three, Three, Three, Four, Three)), true),
            (Hand((Three, Three, Three, Three, Four)), true),
            (Hand((Three, Two, Three, Three, King)), false),
        ] {
            assert_eq!(hand.is_four_of_a_kind(), expected);
        }
    }

    #[test]
    fn hand_is_full_house_success() {
        for (hand, expected) in [
            (Hand((Eight, Eight, Eight, Four, Four)), true),
            (Hand((Eight, Eight, Four, Eight, Four)), true),
            (Hand((Eight, Eight, Four, Four, Eight)), true),
            (Hand((Eight, Four, Eight, Eight, Four)), true),
            (Hand((Eight, Four, Eight, Four, Eight)), true),
            (Hand((Eight, Four, Four, Eight, Eight)), true),
            (Hand((Four, Eight, Eight, Eight, Four)), true),
            (Hand((Four, Eight, Eight, Four, Eight)), true),
            (Hand((Four, Eight, Four, Eight, Eight)), true),
            (Hand((Four, Four, Eight, Eight, Eight)), true),
            (Hand((Three, King, Three, Three, As)), false),
        ] {
            assert_eq!(hand.is_full_house(), expected);
        }
    }

    #[test]
    fn hand_is_three_of_a_kind_success() {
        for (hand, expected) in [
            (Hand((Three, Three, Three, Four, As)), true),
            (Hand((Three, Three, Four, Three, As)), true),
            (Hand((Three, Three, Four, As, Three)), true),
            (Hand((Three, Four, Three, Three, As)), true),
            (Hand((Three, Four, Three, As, Three)), true),
            (Hand((Three, Four, As, Three, Three)), true),
            (Hand((Four, Three, As, Three, Three)), true),
            (Hand((Four, As, Three, Three, Three)), true),
            (Hand((Three, King, Three, King, As)), false),
        ] {
            assert_eq!(hand.is_three_of_a_kind(), expected);
        }
    }

    #[test]
    fn hand_is_two_pair_success() {
        for (hand, expected) in [
            (Hand((Three, Four, Three, Four, As)), true),
            (Hand((Three, Four, Three, King, As)), false),
        ] {
            assert_eq!(hand.is_two_pair(), expected);
        }
    }

    #[test]
    fn hand_is_one_pair_success() {
        for (hand, expected) in [
            (Hand((Three, Four, Three, King, As)), true),
            (Hand((Three, King, Two, Queen, As)), false),
        ] {
            assert_eq!(hand.is_one_pair(), expected);
        }
    }

    #[test]
    fn part2_example() {
        let lines = vec![
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(part2(lines), "5905");
    }

    #[test]
    fn part2_more_cases_1() {
        let lines = vec![
            "2JJJ5 280", // four kind
            "88444 389", // full house
            "86866 534", // full house
            "29929 966", // full house
            "7Q7J6 253", // three kind
            "Q6996 762", // two pairs
            "7292T 98",  // one pair
            "7J9AT 722", // one pair
            "QT2A4 841", // high card
            "69753 514", // high card
            "675KT 785", // high card
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(
            part2(lines),
            (785 * 1
                + 514 * 2
                + 841 * 3
                + 722 * 4
                + 98 * 5
                + 762 * 6
                + 253 * 7
                + 966 * 8
                + 534 * 9
                + 389 * 10
                + 280 * 11)
                .to_string()
        );
    }

    #[test]
    fn hand_is_five_of_a_kind_with_joker_success() {
        for (hand, expected) in [
            (Hand((Jack, Jack, Jack, Jack, Three)), true),
            (Hand((Three, Three, Three, Three, Jack)), true),
            (Hand((Three, Three, Jack, Jack, Three)), true),
            (Hand((Three, Three, Jack, Jack, Jack)), true),
            (Hand((Three, Two, Jack, Jack, Jack)), false),
        ] {
            assert_eq!(hand.is_five_of_a_kind_with_joker(), expected);
        }
    }

    #[test]
    fn hand_is_four_of_a_kind_with_joker_success() {
        for (hand, expected) in [
            (Hand((Four, Three, Three, Three, Jack)), true),
            (Hand((Three, Four, Three, Jack, Jack)), true),
            (Hand((Three, Jack, Four, Jack, Jack)), true),
            (Hand((Three, Jack, Jack, Four, King)), false),
        ] {
            assert_eq!(hand.is_four_of_a_kind_with_joker(), expected);
        }
    }

    #[test]
    fn hand_is_full_house_with_joker_success() {
        for (hand, expected) in [
            (Hand((Eight, Eight, Jack, Four, Four)), true),
            (Hand((Three, King, Jack, Three, As)), false),
        ] {
            assert_eq!(hand.is_full_house_with_joker(), expected);
        }
    }

    #[test]
    fn hand_is_three_of_a_kind_with_joker_success() {
        for (hand, expected) in [
            (Hand((Three, Three, Jack, Four, As)), true),
            (Hand((Three, Jack, Four, Jack, As)), true),
            (Hand((Three, King, Two, Jack, As)), false),
        ] {
            assert_eq!(hand.is_three_of_a_kind_with_joker(), expected);
        }
    }

    #[test]
    fn hand_is_one_pair_with_joker_success() {
        for (hand, expected) in [
            (Hand((Three, Four, Jack, King, As)), true),
            (Hand((Three, King, Two, Queen, As)), false),
        ] {
            assert_eq!(hand.is_one_pair_with_joker(), expected);
        }
    }
}
