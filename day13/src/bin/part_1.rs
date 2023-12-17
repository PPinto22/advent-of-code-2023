use std::{cmp::min, str::FromStr};

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve(input));
}

#[derive(Debug)]
struct PatternList {
    patterns: Vec<Pattern>,
}

#[derive(Debug)]
struct Pattern {
    rows: Vec<Vec<char>>,
    n_rows: usize,
    n_columns: usize,
}

fn solve(input: &str) -> u64 {
    let pattern_list: PatternList = input.parse().unwrap();
    pattern_list
        .patterns
        .iter()
        .map(|pattern| {
            let horizontal_mirror = pattern.find_horizontal_mirror().unwrap_or(0);
            let vertical_mirror = pattern.find_vertical_mirror().unwrap_or(0);
            ((horizontal_mirror * 100) + vertical_mirror) as u64
        })
        .sum()
}

impl Pattern {
    fn new(rows: Vec<Vec<char>>) -> Pattern {
        let n_rows = rows.len();
        let n_columns = rows[0].len();
        Pattern {
            rows,
            n_rows,
            n_columns,
        }
    }

    fn find_horizontal_mirror(&self) -> Option<usize> {
        (1..self.n_rows).find(|i| self.has_horizontal_mirrow_on_row(*i))
    }

    fn has_horizontal_mirrow_on_row(&self, i: usize) -> bool {
        let simmetry_half_size = min(i, self.n_rows - i);
        for di in 0..simmetry_half_size {
            if self.rows[i - di - 1] != self.rows[i + di] {
                return false;
            }
        }
        true
    }

    fn find_vertical_mirror(&self) -> Option<usize> {
        (1..self.n_columns).find(|j| self.has_vertical_mirrow_on_column(*j))
    }

    fn has_vertical_mirrow_on_column(&self, j: usize) -> bool {
        let simmetry_half_size = min(j, self.n_columns - j);
        for dj in 0..simmetry_half_size {
            if self.rows.iter().any(|row| row[j - dj - 1] != row[j + dj]) {
                return false;
            }
        }
        true
    }
}

impl FromStr for PatternList {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().peekable();
        let mut patterns: Vec<Pattern> = Vec::new();

        while lines.peek().is_some() {
            let pattern_rows: Vec<Vec<char>> = lines
                .by_ref()
                .take_while(|l| !l.is_empty())
                .map(|row| row.chars().collect())
                .collect();
            patterns.push(Pattern::new(pattern_rows));
        }

        Ok(PatternList { patterns })
    }
}

#[cfg(test)]
mod test {
    use super::solve;
    use indoc::indoc;

    #[test]
    fn solves_sample() {
        let sample = indoc! {"
            #.##..##.
            ..#.##.#.
            ##......#
            ##......#
            ..#.##.#.
            ..##..##.
            #.#.##.#.
            
            #...##..#
            #....#..#
            ..##..###
            #####.##.
            #####.##.
            ..##..###
            #....#..#
        "};

        let solution = solve(sample);

        assert_eq!(solution, 405);
    }
}
