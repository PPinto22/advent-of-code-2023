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

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
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
        let joker_count = card_counts
            .iter()
            .find(|(card, _)| *card == Card::Jack)
            .map(|(_, count)| *count)
            .unwrap_or(0);
        let regular_card_counts: Vec<u32> = card_counts
            .iter()
            .filter(|(card, _)| *card != Card::Jack)
            .map(|(_, count)| *count)
            .collect();
        let highest_card_count = *regular_card_counts.get(0).unwrap_or(&0);
        let second_highest_card_count = *regular_card_counts.get(1).unwrap_or(&0);
        
        if highest_card_count + joker_count == 5 {
            return HandType::FiveOfAKind;
        }
        if highest_card_count + joker_count == 4 {
            return HandType::FourOfAKind;
        }
        if highest_card_count + joker_count == 3 && second_highest_card_count == 2 {
            return HandType::FullHouse;
        }
        if highest_card_count + joker_count == 3 {
            return HandType::ThreeOfAKind;
        }
        if highest_card_count == 2 && second_highest_card_count == 2 {
            return HandType::TwoPair;
        }
        if highest_card_count + joker_count == 2 {
            return HandType::OnePair;
        }
        return HandType::HighCard;
    }

    fn count_cards_descending(cards: &Vec<Card>) -> Vec<(Card, u32)> {
        let mut card_counts: Vec<(Card, u32)> = cards
            .iter()
            .fold(HashMap::new(), |mut acc, card| {
                *acc.entry(*card).or_insert(0 as u32) += 1;
                acc
            })
            .into_iter()
            .collect();
        card_counts.sort_by_key(|(_, count)| *count);
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

        assert_eq!(solution, 5905);
    }
}
