use std::{collections::HashMap, str::FromStr};

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve(input));
}

#[derive(Debug, Clone)]
struct ConditionRecord {
    springs: Vec<char>,
    damaged_groups: Vec<u32>,
}

#[derive(Debug, PartialEq, Hash, Eq)]
struct CacheKey {
    spring_index: usize,
    damaged_count: u64,
    previous_is_damaged: bool,
}

fn solve(input: &str) -> u128 {
    let records: Vec<ConditionRecord> = input
        .lines()
        .map(|line| line.parse::<ConditionRecord>().unwrap())
        .map(|record| record.unfold())
        .collect();
    records
        .iter()
        .map(|record| record.count_combinations())
        .sum()
}

impl ConditionRecord {
    fn unfold(&self) -> ConditionRecord {
        let mut springs: Vec<char> = Vec::new();
        let mut damaged_groups: Vec<u32> = Vec::new();

        for _ in 0..5 {
            springs.append(&mut self.springs.clone());
            springs.push('?');
            damaged_groups.append(&mut self.damaged_groups.clone());
        }
        springs.pop();

        ConditionRecord {
            springs,
            damaged_groups,
        }
    }

    fn count_combinations(&self) -> u128 {
        let mut current_combination: Vec<char> = Vec::new();
        let mut current_groups: Vec<u32> = Vec::new();
        let mut cache: HashMap<CacheKey, u128> = HashMap::new();

        self.count_combinations_given_current(
            &mut current_combination,
            &mut current_groups,
            &mut cache,
            0,
        )
    }

    fn count_combinations_given_current(
        &self,
        current_combination: &mut Vec<char>,
        current_groups: &mut Vec<u32>,
        cache: &mut HashMap<CacheKey, u128>,
        damaged_count: u64,
    ) -> u128 {
        if current_groups.len() > self.damaged_groups.len() {
            return 0;
        }

        if let Some((combination_current_group, combination_previous_groups)) =
            current_groups.split_last()
        {
            let record_current_group = self.damaged_groups[current_groups.len() - 1];
            let record_previous_groups = &self.damaged_groups[0..combination_previous_groups.len()];
            if combination_current_group > &record_current_group
                || combination_previous_groups != record_previous_groups
            {
                return 0;
            }
        }

        if current_combination.len() == self.springs.len() {
            if current_groups != &self.damaged_groups {
                return 0;
            }
            return 1;
        }

        let previous_spring = current_combination.last();
        let previous_spring_is_damaged = previous_spring.is_some_and(|s| s == &'#');

        let cache_key = CacheKey {
            spring_index: current_combination.len(),
            damaged_count,
            previous_is_damaged: previous_spring_is_damaged,
        };
        if let Some(cached_result) = cache.get(&cache_key) {
            return *cached_result;
        }

        let spring: char = self.springs[current_combination.len()];

        let try_operational_spring = |current_combination: &mut Vec<char>,
                                      current_groups: &mut Vec<u32>,
                                      cache: &mut HashMap<CacheKey, u128>|
         -> u128 {
            current_combination.push('.');
            let combinations = &mut self.count_combinations_given_current(
                current_combination,
                current_groups,
                cache,
                damaged_count,
            );
            current_combination.pop();
            *combinations
        };

        let try_damaged_spring = |current_combination: &mut Vec<char>,
                                  current_groups: &mut Vec<u32>,
                                  cache: &mut HashMap<CacheKey, u128>|
         -> u128 {
            current_combination.push('#');
            if previous_spring_is_damaged {
                let damaged_count = current_groups.pop().unwrap_or(0);
                current_groups.push(damaged_count + 1);
            } else {
                current_groups.push(1);
            }
            let combinations = &mut self.count_combinations_given_current(
                current_combination,
                current_groups,
                cache,
                damaged_count + 1,
            );
            current_combination.pop();
            let damaged_count = current_groups.pop().unwrap();
            if damaged_count > 1 {
                current_groups.push(damaged_count - 1);
            }
            *combinations
        };

        let combinations = match spring {
            '#' => try_damaged_spring(current_combination, current_groups, cache),
            '.' => try_operational_spring(current_combination, current_groups, cache),
            '?' => {
                let combinations_if_damaged =
                    try_damaged_spring(current_combination, current_groups, cache);
                let combinations_if_operational =
                    try_operational_spring(current_combination, current_groups, cache);
                combinations_if_damaged + combinations_if_operational
            }
            _ => panic!("Unknown spring condition: {spring}"),
        };

        cache.insert(cache_key, combinations);
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
    use test_case::test_case;

    #[test_case("???.### 1,1,3", 1)]
    #[test_case(".??..??...?##. 1,1,3", 16384)]
    #[test_case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[test_case("????.#...#... 4,1,1", 16)]
    #[test_case("????.######..#####. 1,6,5", 2500)]
    #[test_case("?###???????? 3,2,1", 506250)]
    fn solves_sample_lines(input: &str, expected_result: u128) {
        let solution = solve(input);

        assert_eq!(solution, expected_result);
    }

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

        assert_eq!(solution, 525152);
    }
}
