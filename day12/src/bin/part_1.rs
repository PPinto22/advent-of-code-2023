use std::str::FromStr;

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve(input));
}

#[derive(Debug)]
struct ConditionRecord {
    springs: Vec<char>,
    damaged_groups: Vec<u32>,
}

fn solve(input: &str) -> u64 {
    let records: Vec<ConditionRecord> = input.lines().map(|line| line.parse().unwrap()).collect();
    records
        .iter()
        .map(|record| record.generate_combinations().len() as u64)
        .sum()
}

impl ConditionRecord {
    fn generate_combinations(&self) -> Vec<Vec<char>> {
        let mut current_combination: Vec<char> = Vec::new();
        let mut current_groups: Vec<u32> = Vec::new();

        self.generate_combinations_given_current(&mut current_combination, &mut current_groups)
    }

    fn generate_combinations_given_current(
        &self,
        current_combination: &mut Vec<char>,
        current_groups: &mut Vec<u32>,
    ) -> Vec<Vec<char>> {
        if current_groups.len() > self.damaged_groups.len() {
            return Vec::new();
        }

        if let Some((combination_current_group, combination_previous_groups)) =
            current_groups.split_last()
        {
            let record_current_group = self.damaged_groups[current_groups.len() - 1];
            let record_previous_groups = &self.damaged_groups[0..combination_previous_groups.len()];
            if combination_current_group > &record_current_group
                || combination_previous_groups != record_previous_groups
            {
                return Vec::new();
            }
        }

        if current_combination.len() == self.springs.len() {
            if current_groups != &self.damaged_groups {
                return Vec::new();
            }
            return vec![current_combination.to_owned()];
        }

        let previous_spring = current_combination.last();
        let previous_spring_is_damaged = previous_spring.is_some_and(|s| s == &'#');
        let spring = self.springs[current_combination.len()];

        let mut combinations: Vec<Vec<char>> = Vec::new();

        let try_operational_spring =
            |current_combination: &mut Vec<char>,
             current_groups: &mut Vec<u32>,
             combinations_result: &mut Vec<Vec<char>>| {
                current_combination.push('.');
                combinations_result.append(
                    &mut self
                        .generate_combinations_given_current(current_combination, current_groups),
                );
                current_combination.pop();
            };

        let try_damaged_spring =
            |current_combination: &mut Vec<char>,
             current_groups: &mut Vec<u32>,
             combinations_result: &mut Vec<Vec<char>>| {
                current_combination.push('#');
                if previous_spring_is_damaged {
                    let damaged_count = current_groups.pop().unwrap_or(0);
                    current_groups.push(damaged_count + 1);
                } else {
                    current_groups.push(1);
                }
                combinations_result.append(
                    &mut self
                        .generate_combinations_given_current(current_combination, current_groups),
                );
                current_combination.pop();
                let damaged_count = current_groups.pop().unwrap();
                if damaged_count > 1 {
                    current_groups.push(damaged_count - 1);
                }
            };

        match spring {
            '#' => try_damaged_spring(current_combination, current_groups, &mut combinations),
            '.' => try_operational_spring(current_combination, current_groups, &mut combinations),
            '?' => {
                try_damaged_spring(current_combination, current_groups, &mut combinations);
                try_operational_spring(current_combination, current_groups, &mut combinations);
            }
            _ => panic!("Unknown spring condition: {spring}"),
        }

        combinations
    }
}

impl FromStr for ConditionRecord {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split_whitespace().collect();
        let springs: Vec<char> = split[0].chars().collect();
        let damaged_groups: Vec<u32> = split[1].split(",").map(|n| n.parse().unwrap()).collect();

        Ok(ConditionRecord {
            springs,
            damaged_groups,
        })
    }
}

#[cfg(test)]
mod test {
    use super::solve;
    use indoc::indoc;

    #[test]
    fn solves_sample() {
        let sample = indoc! {"
            ???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1
        "};

        let solution = solve(sample);

        assert_eq!(solution, 21);
    }
}
