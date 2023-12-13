use std::str::FromStr;

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve(input));
}

#[derive(Debug)]
struct Oasis {
    histories: Vec<ValueHistory>,
}

#[derive(Debug)]
struct ValueHistory {
    values: Vec<i64>,
}

fn solve(input: &str) -> i64 {
    let oasis: Oasis = input.parse().unwrap();
    return oasis.histories.iter().map(|h| h.extrapolate_next()).sum();
}

impl ValueHistory {
    fn extrapolate_next(&self) -> i64 {
        let sequences: Vec<Vec<i64>> = self.generate_sequences_until_zeroes();

        let mut next: i64 = 0;
        for sequence in sequences[0..(sequences.len() - 1)].iter().rev() {
            next = next + sequence.last().unwrap()
        }

        return next;
    }

    fn generate_sequences_until_zeroes(&self) -> Vec<Vec<i64>> {
        let mut sequences: Vec<Vec<i64>> = Vec::new();
        sequences.push(self.values.to_owned());

        let mut last_sequence = sequences.last().unwrap();
        while !last_sequence.iter().all(|v| *v == 0) {
            let mut next: Vec<i64> = Vec::new();
            for i in 0..(last_sequence.len() - 1) {
                next.push(last_sequence[i + 1] - last_sequence[i])
            }

            sequences.push(next.to_owned());
            last_sequence = &sequences.last().unwrap();
        }

        return sequences;
    }
}

impl FromStr for Oasis {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let histories: Vec<ValueHistory> = s
            .lines()
            .map(|line| line.parse::<ValueHistory>().unwrap())
            .collect();
        return Ok(Oasis { histories });
    }
}

impl FromStr for ValueHistory {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<i64> = s
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        return Ok(ValueHistory { values });
    }
}

#[cfg(test)]
mod test {
    use super::solve;
    use indoc::indoc;

    #[test]
    fn solves_sample() {
        let sample = indoc! {"
            0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45
        "};

        let solution = solve(sample);

        assert_eq!(solution, 114);
    }
}
