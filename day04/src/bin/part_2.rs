use regex::Regex;
use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve(input));
}

#[derive(Debug)]
struct Card {
    id: u32,
    winning_numbers: HashSet<u32>,
    owned_numbers: HashSet<u32>,
    count: u32,
}

fn solve(input: &str) -> u32 {
    let mut cards: Vec<Card> = input.lines().map(Card::parse).collect();

    for i in 0..cards.len() {
        let i_count = (&cards[i]).count;
        let won_copies = (&cards[i]).count_winning_numbers();
        for j in 1..=won_copies as usize {
            let next_card = &mut cards[i+j];
            next_card.add_copies(i_count);
        }
    }

    return cards.iter().map(|c| c.count).sum();
}

impl Card {
    fn parse(card_str: &str) -> Card {
        let regex =
            Regex::new(r"^Card\s+(?<id>\d+): (?<winning_numbers>.*) \| (?<owned_numbers>.*)")
                .unwrap();
        let captures = regex.captures(card_str).unwrap();
        let id = captures["id"].parse::<u32>().unwrap();
        let winning_numbers: HashSet<u32> = captures["winning_numbers"]
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        let owned_numbers: HashSet<u32> = captures["owned_numbers"]
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        return Card {
            id,
            winning_numbers,
            owned_numbers,
            count: 1,
        };
    }

    fn count_winning_numbers(&self) -> u32 {
        return self
            .winning_numbers
            .intersection(&self.owned_numbers)
            .count() as u32;
    }

    fn add_copies(&mut self, copies: u32) {
        self.count += copies;
    }
}

#[cfg(test)]
mod test {
    use super::solve;
    use indoc::indoc;

    #[test]
    fn solves_sample() {
        let sample = indoc! {"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "};

        let solution = solve(sample);

        assert_eq!(solution, 30);
    }
}
