fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve(input));
}

#[derive(Debug)]
struct Race {
    time: u64,
    record: u64,
}

fn solve(input: &str) -> u64 {
    let race = Race::parse(input);
    return race.count_ways_to_beat_record();
}

impl Race {
    fn parse(document: &str) -> Race {
        let mut lines = document.lines();
        let time: u64 = lines.next().unwrap().split(": ").collect::<Vec<&str>>()[1]
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join("")
            .parse::<u64>()
            .unwrap();
        let record: u64 = lines.next().unwrap().split(": ").collect::<Vec<&str>>()[1]
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join("")
            .parse::<u64>()
            .unwrap();
        return Race { time, record };
    }

    fn count_ways_to_beat_record(&self) -> u64 {
        return (1..self.time)
            .map(|hold_duration| self.calculate_distance(hold_duration))
            .filter(|distance| *distance > self.record)
            .count() as u64;
    }

    fn calculate_distance(&self, hold_duration: u64) -> u64 {
        let travel_duration = self.time - hold_duration;
        let speed = hold_duration;
        return travel_duration * speed;
    }
}

#[cfg(test)]
mod test {
    use super::solve;
    use indoc::indoc;

    #[test]
    fn solves_sample() {
        let sample = indoc! {"
            Time:      7  15   30
            Distance:  9  40  200
        "};

        let solution = solve(sample);

        assert_eq!(solution, 71503);
    }
}
