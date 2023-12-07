use std::cmp::Ordering;

pub mod task1 {
    use super::total_winnings;

    pub fn ans() -> u128 {
        total_winnings("resources/2023/day07/input")
    }
}

pub mod task2 {
    use super::total_winnings_with_jokers;

    pub fn ans() -> u128 {
        total_winnings_with_jokers("resources/2023/day07/input")
    }
}

fn total_winnings(file: &str) -> u128 {
    let contents = std::fs::read_to_string(file).expect("Could not read file");

    let mut hands = contents.lines().map(Hand::from).collect::<Vec<_>>();

    hands.sort();

    hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i as u128 + 1))
        .sum()
}

fn total_winnings_with_jokers(file: &str) -> u128 {
    let contents = std::fs::read_to_string(file).expect("Could not read file");

    let mut hands = contents
        .lines()
        .map(Hand::from)
        .map(|hand| {
            let cards = hand
                .cards
                .into_iter()
                .map(|card| match card {
                    Card::Jack => Card::Joker,
                    _ => card,
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            Hand {
                cards,
                bid: hand.bid,
            }
        })
        .collect::<Vec<_>>();

    hands.sort();

    hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i as u128 + 1))
        .sum()
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
    Joker,
}

fn card_value(card: &Card) -> usize {
    match card {
        Card::Two => 2,
        Card::Three => 3,
        Card::Four => 4,
        Card::Five => 5,
        Card::Six => 6,
        Card::Seven => 7,
        Card::Eight => 8,
        Card::Nine => 9,
        Card::Ten => 10,
        Card::Jack => 11,
        Card::Queen => 12,
        Card::King => 13,
        Card::Ace => 14,
        Card::Joker => 1,
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_value = card_value(self);
        let other_value = card_value(other);
        self_value.cmp(&other_value)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    bid: u128,
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let regex = regex::Regex::new(r"(?P<hand>[^\s]+) (?P<bid>\d+)").unwrap();
        let captures = regex.captures(value).unwrap();

        let bid = captures
            .name("bid")
            .unwrap()
            .as_str()
            .parse::<u128>()
            .unwrap();

        let hand = captures.name("hand").unwrap().as_str();
        let cards: [Card; 5] = hand
            .chars()
            .map(|card| match card {
                '2' => Card::Two,
                '3' => Card::Three,
                '4' => Card::Four,
                '5' => Card::Five,
                '6' => Card::Six,
                '7' => Card::Seven,
                '8' => Card::Eight,
                '9' => Card::Nine,
                'T' => Card::Ten,
                'J' => Card::Jack,
                'Q' => Card::Queen,
                'K' => Card::King,
                'A' => Card::Ace,
                _ => panic!("Invalid card"),
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Hand { cards, bid }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_jokers = self.number_of_jokers();
        let self_value = joker_rank(&value_hand(self), self_jokers);
        let other_jokers = other.number_of_jokers();
        let other_value = joker_rank(&value_hand(other), other_jokers);

        if self_value != other_value {
            return value_rank(self_value).cmp(&value_rank(other_value));
        }

        let self_cards = self.cards;
        let other_cards = other.cards;

        let self_cards = self_cards.iter().map(card_value).collect::<Vec<_>>();
        let other_cards = other_cards.iter().map(card_value).collect::<Vec<_>>();

        self_cards.cmp(&other_cards)
    }
}

impl Hand {
    fn number_of_jokers(&self) -> usize {
        self.cards
            .iter()
            .filter(|card| **card == Card::Joker)
            .count()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum HandRank {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

fn joker_rank(hand_rank: &HandRank, number_of_jokers: usize) -> HandRank {
    if number_of_jokers == 0 {
        return *hand_rank;
    }
    match (hand_rank, number_of_jokers) {
        (HandRank::FiveOfAKind, _) => HandRank::FiveOfAKind,
        (HandRank::FourOfAKind, _) => HandRank::FiveOfAKind,
        (HandRank::FullHouse, 1) => HandRank::FourOfAKind,
        (HandRank::FullHouse, _) => HandRank::FiveOfAKind,
        (HandRank::ThreeOfAKind, 1) => HandRank::FourOfAKind,
        (HandRank::ThreeOfAKind, _) => HandRank::FiveOfAKind,
        (HandRank::TwoPair, 1) => HandRank::FullHouse,
        (HandRank::TwoPair, 2) => HandRank::FourOfAKind,
        (HandRank::TwoPair, _) => HandRank::FiveOfAKind,
        (HandRank::OnePair, 1) => HandRank::ThreeOfAKind,
        (HandRank::OnePair, 2) => HandRank::FourOfAKind,
        (HandRank::OnePair, _) => HandRank::FiveOfAKind,
        (HandRank::HighCard, 1) => HandRank::OnePair,
        (HandRank::HighCard, 2) => HandRank::ThreeOfAKind,
        (HandRank::HighCard, 3) => HandRank::FourOfAKind,
        (HandRank::HighCard, _) => HandRank::FiveOfAKind,
    }
}

fn value_hand(hand: &Hand) -> HandRank {
    let mut values = hand.cards;
    values.sort();

    let match_pattern = calc_match_pattern(&values);

    match match_pattern[..] {
        [5] => return HandRank::FiveOfAKind,
        [4, 1] => return HandRank::FourOfAKind,
        [1, 4] => return HandRank::FourOfAKind,
        [3, 2] => return HandRank::FullHouse,
        [2, 3] => return HandRank::FullHouse,
        [3, 1, 1] => return HandRank::ThreeOfAKind,
        [1, 3, 1] => return HandRank::ThreeOfAKind,
        [1, 1, 3] => return HandRank::ThreeOfAKind,
        [1, 2, 2] => return HandRank::TwoPair,
        [2, 1, 2] => return HandRank::TwoPair,
        [2, 2, 1] => return HandRank::TwoPair,
        _ => {}
    }

    if !match_pattern.contains(&2) {
        return HandRank::HighCard;
    }

    HandRank::OnePair
}

fn value_rank(hank_rank: HandRank) -> usize {
    match hank_rank {
        HandRank::FiveOfAKind => 7,
        HandRank::FourOfAKind => 6,
        HandRank::FullHouse => 5,
        HandRank::ThreeOfAKind => 4,
        HandRank::TwoPair => 3,
        HandRank::OnePair => 2,
        HandRank::HighCard => 1,
    }
}

fn calc_match_pattern(hand: &[Card; 5]) -> Vec<usize> {
    let hand_without_jokers = hand
        .iter()
        .filter(|card| **card != Card::Joker)
        .copied()
        .collect::<Vec<_>>();

    let (mut match_patterns, final_pattern) = hand_without_jokers.windows(2).fold(
        (vec![], 1usize),
        |(mut match_pattern, mut current_matches), cards| {
            if cards[0] == cards[1] {
                current_matches += 1;
            } else {
                match_pattern.push(current_matches);
                current_matches = 1;
            }
            (match_pattern, current_matches)
        },
    );

    match_patterns.push(final_pattern);

    let number_of_jokers = hand.len() - hand_without_jokers.len();
    match_patterns= [match_patterns, vec![1;number_of_jokers]].concat();

    match_patterns
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hand_parse() {
        let hands = [
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ];

        let expected_hands = [
            Hand {
                cards: [Card::Three, Card::Two, Card::Ten, Card::Three, Card::King],
                bid: 765,
            },
            Hand {
                cards: [Card::Ten, Card::Five, Card::Five, Card::Jack, Card::Five],
                bid: 684,
            },
            Hand {
                cards: [Card::King, Card::King, Card::Six, Card::Seven, Card::Seven],
                bid: 28,
            },
            Hand {
                cards: [Card::King, Card::Ten, Card::Jack, Card::Jack, Card::Ten],
                bid: 220,
            },
            Hand {
                cards: [Card::Queen, Card::Queen, Card::Queen, Card::Jack, Card::Ace],
                bid: 483,
            },
        ];

        let hands = hands.into_iter().map(Hand::from).collect::<Vec<_>>();

        assert_eq!(hands, expected_hands);
    }

    #[test]
    fn test_value_hand() {
        let hands = [
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ];

        let expected_values = [
            HandRank::OnePair,
            HandRank::ThreeOfAKind,
            HandRank::TwoPair,
            HandRank::TwoPair,
            HandRank::ThreeOfAKind,
        ];

        let hand_ranks = hands
            .into_iter()
            .map(Hand::from)
            .map(|hand| value_hand(&hand))
            .collect::<Vec<_>>();

        assert_eq!(hand_ranks, expected_values);
    }

    #[test]
    fn test_order_hands() {
        let mut hands = [
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ]
        .map(Hand::from);

        let expected_order = [
            "32T3K 765",
            "KTJJT 220",
            "KK677 28",
            "T55J5 684",
            "QQQJA 483",
        ]
        .map(Hand::from);

        hands.sort();

        assert_eq!(hands, expected_order);
    }

    #[test]
    fn test_total_winnings() {
        assert_eq!(total_winnings("resources/2023/day07/test_input"), 6440);
    }

    #[test]
    fn test_hand_with_jokers() {
        let hands = [
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ]
        .into_iter()
        .map(Hand::from)
        .map(|hand| {
            let cards = hand
                .cards
                .into_iter()
                .map(|card| match card {
                    Card::Jack => Card::Joker,
                    _ => card,
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            Hand {
                cards,
                bid: hand.bid,
            }
        })
        .collect::<Vec<_>>();

        let expected_hands = [
            Hand {
                cards: [Card::Three, Card::Two, Card::Ten, Card::Three, Card::King],
                bid: 765,
            },
            Hand {
                cards: [Card::Ten, Card::Five, Card::Five, Card::Joker, Card::Five],
                bid: 684,
            },
            Hand {
                cards: [Card::King, Card::King, Card::Six, Card::Seven, Card::Seven],
                bid: 28,
            },
            Hand {
                cards: [Card::King, Card::Ten, Card::Joker, Card::Joker, Card::Ten],
                bid: 220,
            },
            Hand {
                cards: [
                    Card::Queen,
                    Card::Queen,
                    Card::Queen,
                    Card::Joker,
                    Card::Ace,
                ],
                bid: 483,
            },
        ];

        assert_eq!(hands, expected_hands);
    }

    #[test]
    fn test_joker_hand_rank(){
        let hands: Vec<HandRank> = [
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ]
        .into_iter()
        .map(Hand::from)
        .map(|hand| {
            let cards = hand
                .cards
                .into_iter()
                .map(|card| match card {
                    Card::Jack => Card::Joker,
                    _ => card,
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            Hand {
                cards,
                bid: hand.bid,
            }
        })
        .map(|hand| {
            let hand_jokers = hand.number_of_jokers();
            joker_rank(&value_hand(&hand), hand_jokers)
        }).collect();

        let expected_ranks = [
            HandRank::OnePair,
            HandRank::FourOfAKind,
            HandRank::TwoPair,
            HandRank::FourOfAKind,
            HandRank::FourOfAKind,
        ];

        assert_eq!(hands, expected_ranks);

    }

    #[test]
    fn test_total_winnings_with_joker() {
        assert_eq!(
            total_winnings_with_jokers("resources/2023/day07/test_input"),
            5905
        );
    }
}
