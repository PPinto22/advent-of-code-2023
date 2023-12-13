fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve(input));
}

#[derive(Debug)]
struct RaceList {
    races: Vec<Race>,
}

#[derive(Debug)]
struct Race {
    time: u32,
    record: u32,
}

fn solve(input: &str) -> u64 {
    let race_list = RaceList::parse(input);
    return race_list.multiply_number_of_ways_to_beat_records();
}

impl RaceList {
    fn parse(document: &str) -> RaceList {
        let mut lines = document.lines();
        let times: Vec<u32> = lines.next().unwrap().split(": ").collect::<Vec<&str>>()[1]
            .split_whitespace()
            .map(|time_str| time_str.parse::<u32>().unwrap())
            .collect();
        let records: Vec<u32> = lines.next().unwrap().split(": ").collect::<Vec<&str>>()[1]
            .split_whitespace()
            .map(|record_str| record_str.parse::<u32>().unwrap())
            .collect();
        let races: Vec<Race> = times
            .iter()
            .zip(records.iter())
            .map(|(time, record)| Race {
                time: *time,
                record: *record,
            })
            .collect();
        return RaceList { races };
    }

    fn multiply_number_of_ways_to_beat_records(&self) -> u64 {
        return self
            .races
            .iter()
            .map(|race| race.count_ways_to_beat_record())
            .product();
    }
}

impl Race {
    fn count_ways_to_beat_record(&self) -> u64 {
        return (1..self.time)
            .map(|hold_duration| self.calculate_distance(hold_duration))
            .filter(|distance| *distance > self.record)
            .count() as u64;
    }

    fn calculate_distance(&self, hold_duration: u32) -> u32 {
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

        assert_eq!(solution, 288);
    }
}
