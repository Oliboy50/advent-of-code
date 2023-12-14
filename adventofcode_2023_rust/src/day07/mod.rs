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

fn part2(_lines: Vec<String>) -> String {
    todo!()
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
        let hand = *self;
        if hand_is_five_of_a_kind(hand) {
            HandType::FiveOfAKind
        } else if hand_is_four_of_a_kind(hand) {
            HandType::FourOfAKind
        } else if hand_is_full_house(hand) {
            HandType::FullHouse
        } else if hand_is_three_of_a_kind(hand) {
            HandType::ThreeOfAKind
        } else if hand_is_two_pair(hand) {
            HandType::TwoPair
        } else if hand_is_one_pair(hand) {
            HandType::OnePair
        } else if hand_is_high_card(hand) {
            HandType::HighCard
        } else {
            panic!("Unsupported hand");
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

fn hand_is_five_of_a_kind(hand: Hand) -> bool {
    hand.first_card() == hand.second_card()
        && hand.second_card() == hand.third_card()
        && hand.third_card() == hand.fourth_card()
        && hand.fourth_card() == hand.fifth_card()
}

fn hand_is_four_of_a_kind(hand: Hand) -> bool {
    if hand_is_five_of_a_kind(hand) {
        return false;
    }

    (hand.first_card() == hand.second_card()
        && hand.second_card() == hand.third_card()
        && hand.third_card() == hand.fourth_card())
        || (hand.first_card() == hand.second_card()
            && hand.second_card() == hand.third_card()
            && hand.third_card() == hand.fifth_card())
        || (hand.first_card() == hand.second_card()
            && hand.second_card() == hand.fourth_card()
            && hand.fourth_card() == hand.fifth_card())
        || (hand.first_card() == hand.third_card()
            && hand.third_card() == hand.fourth_card()
            && hand.fourth_card() == hand.fifth_card())
        || (hand.second_card() == hand.third_card()
            && hand.third_card() == hand.fourth_card()
            && hand.fourth_card() == hand.fifth_card())
}

fn hand_is_full_house(hand: Hand) -> bool {
    if hand_is_five_of_a_kind(hand) {
        return false;
    }
    if hand_is_four_of_a_kind(hand) {
        return false;
    }

    (hand.first_card() == hand.second_card()
        && hand.second_card() == hand.third_card()
        && hand.fourth_card() == hand.fifth_card())
        || (hand.first_card() == hand.second_card()
            && hand.second_card() == hand.fourth_card()
            && hand.third_card() == hand.fifth_card())
        || (hand.first_card() == hand.second_card()
            && hand.second_card() == hand.fifth_card()
            && hand.third_card() == hand.fourth_card())
        || (hand.first_card() == hand.third_card()
            && hand.third_card() == hand.fourth_card()
            && hand.second_card() == hand.fifth_card())
        || (hand.first_card() == hand.third_card()
            && hand.third_card() == hand.fifth_card()
            && hand.second_card() == hand.fourth_card())
        || (hand.first_card() == hand.fourth_card()
            && hand.fourth_card() == hand.fifth_card()
            && hand.second_card() == hand.third_card())
        || (hand.second_card() == hand.third_card()
            && hand.third_card() == hand.fourth_card()
            && hand.first_card() == hand.fifth_card())
        || (hand.second_card() == hand.third_card()
            && hand.third_card() == hand.fifth_card()
            && hand.first_card() == hand.fourth_card())
        || (hand.second_card() == hand.fourth_card()
            && hand.fourth_card() == hand.fifth_card()
            && hand.first_card() == hand.third_card())
        || (hand.third_card() == hand.fourth_card()
            && hand.fourth_card() == hand.fifth_card()
            && hand.first_card() == hand.second_card())
}

fn hand_is_three_of_a_kind(hand: Hand) -> bool {
    if hand_is_five_of_a_kind(hand) {
        return false;
    }
    if hand_is_four_of_a_kind(hand) {
        return false;
    }
    if hand_is_full_house(hand) {
        return false;
    }

    (hand.first_card() == hand.second_card() && hand.second_card() == hand.third_card())
        || (hand.first_card() == hand.second_card() && hand.second_card() == hand.fourth_card())
        || (hand.first_card() == hand.second_card() && hand.second_card() == hand.fifth_card())
        || (hand.first_card() == hand.third_card() && hand.third_card() == hand.fourth_card())
        || (hand.first_card() == hand.third_card() && hand.third_card() == hand.fifth_card())
        || (hand.first_card() == hand.fourth_card() && hand.fourth_card() == hand.fifth_card())
        || (hand.second_card() == hand.third_card() && hand.third_card() == hand.fourth_card())
        || (hand.second_card() == hand.third_card() && hand.third_card() == hand.fifth_card())
        || (hand.second_card() == hand.fourth_card() && hand.fourth_card() == hand.fifth_card())
        || (hand.third_card() == hand.fourth_card() && hand.fourth_card() == hand.fifth_card())
}

fn hand_is_two_pair(hand: Hand) -> bool {
    if hand_is_five_of_a_kind(hand) {
        return false;
    }
    if hand_is_four_of_a_kind(hand) {
        return false;
    }
    if hand_is_full_house(hand) {
        return false;
    }
    if hand_is_three_of_a_kind(hand) {
        return false;
    }

    (hand.first_card() == hand.second_card() && hand.third_card() == hand.fourth_card())
        || (hand.first_card() == hand.second_card() && hand.third_card() == hand.fifth_card())
        || (hand.first_card() == hand.second_card() && hand.fourth_card() == hand.fifth_card())
        || (hand.first_card() == hand.third_card() && hand.second_card() == hand.fourth_card())
        || (hand.first_card() == hand.third_card() && hand.second_card() == hand.fifth_card())
        || (hand.first_card() == hand.third_card() && hand.fourth_card() == hand.fifth_card())
        || (hand.first_card() == hand.fourth_card() && hand.second_card() == hand.third_card())
        || (hand.first_card() == hand.fourth_card() && hand.second_card() == hand.fifth_card())
        || (hand.first_card() == hand.fourth_card() && hand.third_card() == hand.fifth_card())
        || (hand.first_card() == hand.fifth_card() && hand.second_card() == hand.third_card())
        || (hand.first_card() == hand.fifth_card() && hand.second_card() == hand.fourth_card())
        || (hand.first_card() == hand.fifth_card() && hand.third_card() == hand.fourth_card())
        || (hand.second_card() == hand.third_card() && hand.fourth_card() == hand.fifth_card())
        || (hand.second_card() == hand.fourth_card() && hand.third_card() == hand.fifth_card())
        || (hand.second_card() == hand.fifth_card() && hand.third_card() == hand.fourth_card())
}

fn hand_is_one_pair(hand: Hand) -> bool {
    if hand_is_five_of_a_kind(hand) {
        return false;
    }
    if hand_is_four_of_a_kind(hand) {
        return false;
    }
    if hand_is_full_house(hand) {
        return false;
    }
    if hand_is_three_of_a_kind(hand) {
        return false;
    }
    if hand_is_two_pair(hand) {
        return false;
    }

    (hand.first_card() == hand.second_card())
        || (hand.first_card() == hand.third_card())
        || (hand.first_card() == hand.fourth_card())
        || (hand.first_card() == hand.fifth_card())
        || (hand.second_card() == hand.third_card())
        || (hand.second_card() == hand.fourth_card())
        || (hand.second_card() == hand.fifth_card())
        || (hand.third_card() == hand.fourth_card())
        || (hand.third_card() == hand.fifth_card())
        || (hand.fourth_card() == hand.fifth_card())
}

fn hand_is_high_card(hand: Hand) -> bool {
    if hand_is_five_of_a_kind(hand) {
        return false;
    }
    if hand_is_four_of_a_kind(hand) {
        return false;
    }
    if hand_is_full_house(hand) {
        return false;
    }
    if hand_is_three_of_a_kind(hand) {
        return false;
    }
    if hand_is_two_pair(hand) {
        return false;
    }
    if hand_is_one_pair(hand) {
        return false;
    }

    true
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
            assert_eq!(hand_is_five_of_a_kind(hand), expected);
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
            (Hand((Three, Three, Three, Three, Three)), false),
        ] {
            assert_eq!(hand_is_four_of_a_kind(hand), expected);
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
            (Hand((Three, Four, Three, Three, Three)), false),
            (Hand((Three, Three, Three, Three, Three)), false),
        ] {
            assert_eq!(hand_is_full_house(hand), expected);
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
            (Hand((Three, King, Three, Three, King)), false),
            (Hand((Three, Four, Three, Three, Three)), false),
            (Hand((Three, Three, Three, Three, Three)), false),
        ] {
            assert_eq!(hand_is_three_of_a_kind(hand), expected);
        }
    }

    #[test]
    fn hand_is_two_pair_success() {
        for (hand, expected) in [
            (Hand((Three, Four, Three, Four, As)), true),
            (Hand((Three, Four, Three, King, As)), false),
            (Hand((Three, Four, Three, As, Three)), false),
            (Hand((Three, King, Three, Three, King)), false),
            (Hand((Three, Four, Three, Three, Three)), false),
            (Hand((Three, Three, Three, Three, Three)), false),
        ] {
            assert_eq!(hand_is_two_pair(hand), expected);
        }
    }

    #[test]
    fn hand_is_one_pair_success() {
        for (hand, expected) in [
            (Hand((Three, Four, Three, King, As)), true),
            (Hand((Three, King, Two, Queen, As)), false),
            (Hand((Three, Four, Three, Four, As)), false),
            (Hand((Three, Four, Three, As, Three)), false),
            (Hand((Three, King, Three, Three, King)), false),
            (Hand((Three, Four, Three, Three, Three)), false),
            (Hand((Three, Three, Three, Three, Three)), false),
        ] {
            assert_eq!(hand_is_one_pair(hand), expected);
        }
    }

    #[test]
    fn hand_is_high_card_success() {
        for (hand, expected) in [
            (Hand((Three, King, Two, Queen, As)), true),
            (Hand((Three, Four, Three, King, As)), false),
            (Hand((Three, Four, Three, Four, As)), false),
            (Hand((Three, Four, Three, As, Three)), false),
            (Hand((Three, King, Three, Three, King)), false),
            (Hand((Three, Four, Three, Three, Three)), false),
            (Hand((Three, Three, Three, Three, Three)), false),
        ] {
            assert_eq!(hand_is_high_card(hand), expected);
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
}
