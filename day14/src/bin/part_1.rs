use std::str::FromStr;

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve(input));
}

#[derive(Debug)]
struct Platform {
    rows: Vec<Vec<char>>,
    n_rows: usize,
    n_columns: usize,
}

fn solve(input: &str) -> u64 {
    let mut platform: Platform = input.parse().unwrap();
    platform.tilt_north();
    platform.calculate_load()
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

        assert_eq!(solution, 136);
    }
}
