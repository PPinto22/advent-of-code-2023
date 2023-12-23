use std::{cmp::max, collections::HashSet, str::FromStr};

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve(input));
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Point(usize, usize);

#[derive(Debug)]
struct Grid {
    rows: Vec<Vec<char>>,
    n_rows: usize,
    n_columns: usize,
    beams: Vec<Beam>,
    energised_tiles: HashSet<Point>,
    visited: HashSet<Beam>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Beam {
    position: Point,
    direction: Direction,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Direction {
    N,
    E,
    S,
    W,
}

fn solve(input: &str) -> u64 {
    let mut grid: Grid = input.parse().unwrap();
    let n_rows = grid.n_rows;
    let n_columns = grid.n_columns;

    let mut max_energised: u64 = 0;
    let mut try_configuration = |grid: &mut Grid, position: Point, direction: Direction| {
        grid.reset();
        grid.set_initial_beam(Beam {
            position,
            direction,
        });
        grid.shine_beam();
        max_energised = max(max_energised, grid.energised_tiles.len() as u64);
    };

    for i in 0..n_rows {
        try_configuration(&mut grid, Point(i, 0), Direction::E);
        try_configuration(&mut grid, Point(i, n_columns - 1), Direction::W);
    }

    for j in 0..n_columns {
        try_configuration(&mut grid, Point(0, j), Direction::S);
        try_configuration(&mut grid, Point(n_rows - 1, j), Direction::N);
    }

    try_configuration(&mut grid, Point(0, 0), Direction::E);
    try_configuration(&mut grid, Point(0, 0), Direction::S);

    try_configuration(&mut grid, Point(0, n_columns - 1), Direction::S);
    try_configuration(&mut grid, Point(0, n_columns - 1), Direction::W);

    try_configuration(&mut grid, Point(n_rows - 1, n_columns - 1), Direction::W);
    try_configuration(&mut grid, Point(n_rows - 1, n_columns - 1), Direction::N);

    try_configuration(&mut grid, Point(n_rows - 1, 0), Direction::N);
    try_configuration(&mut grid, Point(n_rows - 1, 0), Direction::E);

    max_energised
}

impl Grid {
    fn new(rows: Vec<Vec<char>>) -> Grid {
        let n_rows = rows.len();
        let n_columns = rows[0].len();

        // 1 cell padding to make it easier to move within the grid without going out of bounds
        let mut padded_rows = rows.clone();
        for row in padded_rows.iter_mut() {
            row.insert(0, '.');
            row.push('.');
        }
        padded_rows.insert(0, vec!['.'; n_columns + 2]);
        padded_rows.push(vec!['.'; n_columns + 2]);

        let initial_beam = Beam {
            position: Point(1, 1),
            direction: Direction::E,
        };

        Grid {
            rows: padded_rows,
            n_rows,
            n_columns,
            beams: vec![initial_beam.clone()],
            energised_tiles: vec![initial_beam.position.clone()].into_iter().collect(),
            visited: vec![initial_beam.clone()].into_iter().collect(),
        }
    }

    fn reset(&mut self) {
        self.beams.clear();
        self.energised_tiles.clear();
        self.visited.clear();
    }

    fn set_initial_beam(&mut self, beam: Beam) {
        let beam = Beam {
            position: Point(beam.position.0 + 1, beam.position.1 + 1),
            direction: beam.direction,
        };
        self.beams = vec![beam.clone()];
        self.energised_tiles = vec![beam.position.clone()].into_iter().collect();
        self.visited = vec![beam.clone()].into_iter().collect();
    }

    fn print_energised_diagram(&self) {
        for i in 0..self.n_rows {
            for j in 0..self.n_columns {
                let point = Point(i + 1, j + 1);
                let symbol = if self.energised_tiles.contains(&point) {
                    '#'
                } else {
                    '.'
                };
                print!("{symbol}")
            }
            println!("")
        }
    }

    fn shine_beam(&mut self) {
        while !self.beams.is_empty() {
            self.advance_all_beams();
        }
    }

    fn advance_all_beams(&mut self) {
        for _ in 0..self.beams.len() {
            self.advance_beam(0);
        }
    }

    fn advance_beam(&mut self, beam_i: usize) {
        let beam = &self.beams[beam_i];

        let advanced_beams: Vec<Beam> = match self.get(&beam.position) {
            '.' => self
                .advance_beam_in_current_direction(beam)
                .into_iter()
                .collect(),
            '/' => self.reflect_on_forward_mirror(beam).into_iter().collect(),
            '\\' => self.reflect_on_backward_mirror(beam).into_iter().collect(),
            '|' => self.split_on_vertical_splitter(beam),
            '-' => self.split_on_horizontal_splitter(beam),
            other => panic!("Unknown grid character: {other}"),
        };
        let mut advanced_beams: Vec<Beam> = advanced_beams
            .into_iter()
            .filter(|b| !self.visited.contains(b))
            .collect();

        advanced_beams.iter().for_each(|b| {
            self.energised_tiles.insert(b.position.clone());
            if !self.visited.contains(&b) {
                self.visited.insert(b.clone());
            }
        });

        self.beams.remove(beam_i);
        self.beams.append(&mut advanced_beams);
    }

    fn advance_beam_in_current_direction(&self, beam: &Beam) -> Option<Beam> {
        let Point(i, j) = beam.position;
        let advanced_beam = match beam.direction {
            Direction::N => Beam {
                position: Point(i - 1, j),
                direction: Direction::N,
            },
            Direction::E => Beam {
                position: Point(i, j + 1),
                direction: Direction::E,
            },
            Direction::S => Beam {
                position: Point(i + 1, j),
                direction: Direction::S,
            },
            Direction::W => Beam {
                position: Point(i, j - 1),
                direction: Direction::W,
            },
        };
        Some(advanced_beam).filter(|b| !self.is_out_of_bounds(&b.position))
    }

    fn reflect_on_forward_mirror(&self, beam: &Beam) -> Option<Beam> {
        assert!(self.get(&beam.position) == '/');

        let Point(i, j) = beam.position;
        let reflected_beam = match beam.direction {
            Direction::N => Beam {
                position: Point(i, j + 1),
                direction: Direction::E,
            },
            Direction::E => Beam {
                position: Point(i - 1, j),
                direction: Direction::N,
            },
            Direction::S => Beam {
                position: Point(i, j - 1),
                direction: Direction::W,
            },
            Direction::W => Beam {
                position: Point(i + 1, j),
                direction: Direction::S,
            },
        };
        Some(reflected_beam).filter(|b| !self.is_out_of_bounds(&b.position))
    }

    fn reflect_on_backward_mirror(&self, beam: &Beam) -> Option<Beam> {
        assert!(self.get(&beam.position) == '\\');

        let Point(i, j) = beam.position;
        let reflected_beam = match beam.direction {
            Direction::N => Beam {
                position: Point(i, j - 1),
                direction: Direction::W,
            },
            Direction::E => Beam {
                position: Point(i + 1, j),
                direction: Direction::S,
            },
            Direction::S => Beam {
                position: Point(i, j + 1),
                direction: Direction::E,
            },
            Direction::W => Beam {
                position: Point(i - 1, j),
                direction: Direction::N,
            },
        };
        Some(reflected_beam).filter(|b| !self.is_out_of_bounds(&b.position))
    }

    fn split_on_horizontal_splitter(&self, beam: &Beam) -> Vec<Beam> {
        assert!(self.get(&beam.position) == '-');

        let Point(i, j) = beam.position;
        let reflected_beams = match beam.direction {
            Direction::N => vec![
                Beam {
                    position: Point(i, j - 1),
                    direction: Direction::W,
                },
                Beam {
                    position: Point(i, j + 1),
                    direction: Direction::E,
                },
            ],
            Direction::E => vec![Beam {
                position: Point(i, j + 1),
                direction: Direction::E,
            }],
            Direction::S => vec![
                Beam {
                    position: Point(i, j - 1),
                    direction: Direction::W,
                },
                Beam {
                    position: Point(i, j + 1),
                    direction: Direction::E,
                },
            ],
            Direction::W => vec![Beam {
                position: Point(i, j - 1),
                direction: Direction::W,
            }],
        };
        reflected_beams
            .into_iter()
            .filter(|b| !self.is_out_of_bounds(&b.position))
            .collect()
    }

    fn split_on_vertical_splitter(&self, beam: &Beam) -> Vec<Beam> {
        assert!(self.get(&beam.position) == '|');

        let Point(i, j) = beam.position;
        let reflected_beams = match beam.direction {
            Direction::N => vec![Beam {
                position: Point(i - 1, j),
                direction: Direction::N,
            }],
            Direction::E => vec![
                Beam {
                    position: Point(i - 1, j),
                    direction: Direction::N,
                },
                Beam {
                    position: Point(i + 1, j),
                    direction: Direction::S,
                },
            ],
            Direction::S => vec![Beam {
                position: Point(i + 1, j),
                direction: Direction::S,
            }],
            Direction::W => vec![
                Beam {
                    position: Point(i - 1, j),
                    direction: Direction::N,
                },
                Beam {
                    position: Point(i + 1, j),
                    direction: Direction::S,
                },
            ],
        };
        reflected_beams
            .into_iter()
            .filter(|b| !self.is_out_of_bounds(&b.position))
            .collect()
    }

    fn get(&self, point: &Point) -> char {
        self.rows[point.0][point.1]
    }

    fn is_out_of_bounds(&self, point: &Point) -> bool {
        let Point(i, j) = *point;
        i == 0 || i >= (self.n_rows + 1) || j == 0 || j >= (self.n_columns + 1)
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();
        Ok(Grid::new(rows))
    }
}

#[cfg(test)]
mod test {
    use super::solve;
    use indoc::indoc;

    #[test]
    fn solves_sample() {
        let sample = indoc! {r"
            .|...\....
            |.-.\.....
            .....|-...
            ........|.
            ..........
            .........\
            ..../.\\..
            .-.-/..|..
            .|....-|.\
            ..//.|....
        "};

        let solution = solve(sample);

        assert_eq!(solution, 51);
    }
}
