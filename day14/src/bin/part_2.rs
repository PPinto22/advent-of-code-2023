use std::{collections::HashMap, str::FromStr};

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve(input));
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Platform {
    rows: Vec<Vec<char>>,
    n_rows: usize,
    n_columns: usize,
}

fn solve(input: &str) -> u64 {
    let mut platform: Platform = input.parse().unwrap();
    platform.tilt_north();
    platform.calculate_load_after_n_cycles(1_000_000_000)
}

impl Platform {
    fn new(rows: Vec<Vec<char>>) -> Platform {
        let n_rows = rows.len();
        let n_columns = rows[0].len();
        Platform {
            rows,
            n_rows,
            n_columns,
        }
    }

    fn spin_cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    fn tilt_north(&mut self) {
        let mut solid_indexes: Vec<i32> = vec![-1; self.n_columns];
        for i in 0..self.n_rows {
            for j in 0..self.n_columns {
                match self.rows[i][j] {
                    'O' => {
                        self.rows[i][j] = '.';
                        self.rows[(solid_indexes[j] + 1) as usize][j] = 'O';
                        solid_indexes[j] += 1;
                    }
                    '#' => {
                        solid_indexes[j] = i as i32;
                    }
                    '.' => {}
                    _ => panic!(
                        "Unknown character at row {i}, column {j}: {}",
                        self.rows[i][j]
                    ),
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        let mut solid_indexes: Vec<usize> = vec![self.n_rows; self.n_columns];
        for i in (0..self.n_rows).rev() {
            for j in 0..self.n_columns {
                match self.rows[i][j] {
                    'O' => {
                        self.rows[i][j] = '.';
                        self.rows[(solid_indexes[j] - 1) as usize][j] = 'O';
                        solid_indexes[j] -= 1;
                    }
                    '#' => {
                        solid_indexes[j] = i;
                    }
                    '.' => {}
                    _ => panic!(
                        "Unknown character at row {i}, column {j}: {}",
                        self.rows[i][j]
                    ),
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        let mut solid_indexes: Vec<i32> = vec![-1; self.n_rows];
        for i in 0..self.n_rows {
            for j in 0..self.n_columns {
                match self.rows[i][j] {
                    'O' => {
                        self.rows[i][j] = '.';
                        self.rows[i][(solid_indexes[i] + 1) as usize] = 'O';
                        solid_indexes[i] += 1;
                    }
                    '#' => {
                        solid_indexes[i] = j as i32;
                    }
                    '.' => {}
                    _ => panic!(
                        "Unknown character at row {i}, column {j}: {}",
                        self.rows[i][j]
                    ),
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        let mut solid_indexes: Vec<usize> = vec![self.n_columns; self.n_rows];
        for i in 0..self.n_rows {
            for j in (0..self.n_columns).rev() {
                match self.rows[i][j] {
                    'O' => {
                        self.rows[i][j] = '.';
                        self.rows[i][(solid_indexes[i] - 1) as usize] = 'O';
                        solid_indexes[i] -= 1;
                    }
                    '#' => {
                        solid_indexes[i] = j;
                    }
                    '.' => {}
                    _ => panic!(
                        "Unknown character at row {i}, column {j}: {}",
                        self.rows[i][j]
                    ),
                }
            }
        }
    }

    fn calculate_load(&self) -> u64 {
        let mut load: u64 = 0;
        for (i, row) in self.rows.iter().enumerate() {
            for c in row.iter() {
                if *c == 'O' {
                    load += (self.n_rows - i) as u64
                }
            }
        }
        load
    }

    fn calculate_load_after_n_cycles(&self, n_cycles: usize) -> u64 {
        let mut cycles: Vec<u64> = Vec::new();
        let mut seen_states: HashMap<Vec<Vec<char>>, usize> = HashMap::new();

        let mut platform = self.clone();
        let mut cycle: usize = 0;

        while cycle < n_cycles {
            cycle += 1;
            platform.spin_cycle();

            if let Some(loop_start) = seen_states.get(&platform.rows) {
                let loop_length = cycle - loop_start;
                return cycles[(*loop_start + ((n_cycles - *loop_start) % loop_length)) - 1];
            }

            let load = platform.calculate_load();
            cycles.push(load);
            seen_states.insert(platform.rows.clone(), cycle);
        }

        *cycles.last().unwrap()
    }
}

impl FromStr for Platform {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();
        Ok(Platform::new(rows))
    }
}

#[cfg(test)]
mod test {
    use crate::Platform;

    use super::solve;
    use indoc::indoc;

    #[test]
    fn solves_sample() {
        let sample = indoc! {"
            O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#....
        "};

        let solution = solve(sample);

        assert_eq!(solution, 64);
    }

    #[test]
    fn runs_spin_cycle() {
        let mut plaform: Platform = (indoc! {"
            O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#....
        "})
        .parse()
        .unwrap();

        // cycle 1
        plaform.spin_cycle();
        let cycle_1_expected: Platform = (indoc! {"
            .....#....
            ....#...O#
            ...OO##...
            .OO#......
            .....OOO#.
            .O#...O#.#
            ....O#....
            ......OOOO
            #...O###..
            #..OO#....
        "})
        .parse()
        .unwrap();
        assert_eq!(plaform, cycle_1_expected);

        // cycle 2
        plaform.spin_cycle();
        let cycle_2_expected: Platform = (indoc! {"
            .....#....
            ....#...O#
            .....##...
            ..O#......
            .....OOO#.
            .O#...O#.#
            ....O#...O
            .......OOO
            #..OO###..
            #.OOO#...O
        "})
        .parse()
        .unwrap();
        assert_eq!(plaform, cycle_2_expected);

        // cycle 3
        plaform.spin_cycle();
        let cycle_3_expected: Platform = (indoc! {"
            .....#....
            ....#...O#
            .....##...
            ..O#......
            .....OOO#.
            .O#...O#.#
            ....O#...O
            .......OOO
            #...O###.O
            #.OOO#...O
        "})
        .parse()
        .unwrap();
        assert_eq!(plaform, cycle_3_expected);
    }
}
