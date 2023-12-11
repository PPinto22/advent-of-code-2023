use core::cmp::Ordering::Equal;
use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve(input));
}

#[derive(Debug)]
struct HandBidList {
    hands: Vec<HandBid>,
}

#[derive(Debug)]
struct HandBid {
    hand: Hand,
    bid: u64,
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Hand {
    cards: Vec<Card>,
    type_: HandType,
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum Card {
    Ace,
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

#[derive(PartialEq, Eq, Hash, Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

fn solve(input: &str) -> u64 {
    let mut hands = HandBidList::parse(input);
    return hands.calculate_winnings();
}

impl HandBidList {
    fn parse(hand_bid_list_str: &str) -> HandBidList {
        let hands: Vec<HandBid> = hand_bid_list_str.lines().map(HandBid::parse).collect();
        return HandBidList { hands };
    }

    fn calculate_winnings(&mut self) -> u64 {
        self.hands.sort_by(|h1, h2| h1.hand.cmp(&h2.hand));

        return self
            .hands
            .iter()
            .enumerate()
            .map(|(i, hand)| hand.bid * (i as u64 + 1))
            .sum();
    }
}

impl HandBid {
    fn parse(hand_bid: &str) -> HandBid {
        let parts = hand_bid.split_whitespace().collect::<Vec<&str>>();
        let cards: Vec<Card> = parts[0].chars().map(Card::parse).collect();
        let bid = parts[1].parse::<u64>().unwrap();

        let hand = Hand::new(cards);
        return HandBid { hand, bid };
    }
}

impl Hand {
    fn new(cards: Vec<Card>) -> Self {
        let type_ = HandType::infer(&cards);
        return Hand { cards, type_ };
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        return match self.type_.cmp(&other.type_) {
            Equal => self.cards.cmp(&other.cards),
            ord => return ord,
        };
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl Card {
    fn parse(card: char) -> Card {
        match card {
            'A' => Card::Ace,
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
            _ => panic!("Unknown card: {card}"),
        }
    }

    fn strength(&self) -> u32 {
        return match self {
            Card::Ace => 13,
            Card::King => 12,
            Card::Queen => 11,
            Card::Jack => 10,
            Card::Ten => 9,
            Card::Nine => 8,
            Card::Eight => 7,
            Card::Seven => 6,
            Card::Six => 5,
            Card::Five => 4,
            Card::Four => 3,
            Card::Three => 2,
            Card::Two => 1,
        };
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.strength().cmp(&other.strength());
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl HandType {
    fn infer(cards: &Vec<Card>) -> HandType {
        if cards.len() != 5 {
            panic!("Invalid hand, must have exactly 5 cards: {cards:?}")
        }
        let card_counts = HandType::count_cards_descending(cards);

        if card_counts[0] == 5 {
            return HandType::FiveOfAKind;
        }
        if card_counts[0] == 4 {
            return HandType::FourOfAKind;
        }
        if card_counts[0] == 3 && card_counts[1] == 2 {
            return HandType::FullHouse;
        }
        if card_counts[0] == 3 {
            return HandType::ThreeOfAKind;
        }
        if card_counts[0] == 2 && card_counts[1] == 2 {
            return HandType::TwoPair;
        }
        if card_counts[0] == 2 {
            return HandType::OnePair;
        }
        if card_counts[0] == 1 {
            return HandType::HighCard;
        }
        panic!("Cannot infer type of hand: {cards:?}")
    }

    fn count_cards_descending(cards: &Vec<Card>) -> Vec<u32> {
        let mut card_counts = cards
            .iter()
            .fold(HashMap::new(), |mut acc, card| {
                *acc.entry(card).or_insert(0 as u32) += 1;
                acc
            })
            .into_values()
            .collect::<Vec<u32>>();
        card_counts.sort();
        card_counts.reverse();
        return card_counts;
    }

    fn strength(&self) -> u32 {
        return match self {
            HandType::FiveOfAKind => 7,
            HandType::FourOfAKind => 6,
            HandType::FullHouse => 5,
            HandType::ThreeOfAKind => 4,
            HandType::TwoPair => 3,
            HandType::OnePair => 2,
            HandType::HighCard => 1,
        };
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.strength().cmp(&other.strength());
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

#[cfg(test)]
mod test {
    use super::solve;
    use indoc::indoc;

    #[test]
    fn solves_sample() {
        let sample = indoc! {"
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        "};

        let solution = solve(sample);

        assert_eq!(solution, 6440);
    }
}
