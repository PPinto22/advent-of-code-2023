use std::str::FromStr;

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve(input));
}

type Point = (usize, usize);

type UnboundedPoint = (i64, i64);

enum NavigationError {
    LoopNotFound,
}

struct ParsePipeError;

#[derive(Debug)]
struct Maze {
    tiles: Vec<Vec<TileType>>,
    n_rows: usize,
    n_columns: usize,
    start: Point,
}

#[derive(PartialEq, Eq, Debug)]
enum TileType {
    Pipe(PipeType),
    Ground,
    Start,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum PipeType {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    N,
    E,
    S,
    W,
}

fn solve(input: &str) -> u64 {
    let maze: Maze = input.parse().unwrap();
    let loop_path = maze.find_loop();
    loop_path.len() as u64 / 2
}

impl Maze {
    fn new(tiles: Vec<Vec<TileType>>) -> Self {
        let n_rows = tiles.len();
        let n_columns = tiles[0].len();
        let start = Self::find_start(&tiles);
        Maze {
            tiles,
            n_rows,
            n_columns,
            start,
        }
    }

    fn find_start(tiles: &Vec<Vec<TileType>>) -> Point {
        for (i, row) in tiles.iter().enumerate() {
            for (j, tile) in row.iter().enumerate() {
                if *tile == TileType::Start {
                    return (i, j);
                }
            }
        }
        panic!("Start not found");
    }

    fn find_loop(&self) -> Vec<Point> {
        for start_tile in PipeType::VALUES {
            for start_direction in Direction::VALUES {
                if let Ok(path) = self.find_loop_given_start(start_tile, start_direction) {
                    return path;
                }
            }
        }
        panic!("Loop not found");
    }

    fn find_loop_given_start(
        &self,
        start_tile: PipeType,
        start_direction: Direction,
    ) -> Result<Vec<Point>, NavigationError> {
        let mut path: Vec<Point> = Vec::new();
        let mut point = self.start;
        let mut direction = start_direction;

        loop {
            path.push(point);
            let next = self.step(start_tile, point, direction);
            if next.is_none() {
                return Err(NavigationError::LoopNotFound);
            }

            let (next_point, next_direction) = next.unwrap();
            if next_point == self.start {
                break;
            }

            point = next_point;
            direction = next_direction;
        }

        Ok(path)
    }

    fn step(
        &self,
        start_tile: PipeType,
        point: Point,
        direction: Direction,
    ) -> Option<(Point, Direction)> {
        match self.get_tile(point) {
            TileType::Pipe(pipe) => self.step_given_pipe(point, direction, &pipe),
            TileType::Start => self.step_given_pipe(point, direction, &start_tile),
            _ => None,
        }
    }

    fn step_given_pipe(
        &self,
        point: Point,
        direction: Direction,
        current_pipe: &PipeType,
    ) -> Option<(Point, Direction)> {
        let (i, j): UnboundedPoint = (point.0 as i64, point.1 as i64);
        let next_unbounded: Option<(UnboundedPoint, Direction)> = match current_pipe {
            PipeType::NS => match direction {
                Direction::S => Some(((i + 1, j), Direction::S)),
                Direction::N => Some(((i - 1, j), Direction::N)),
                _ => None,
            },
            PipeType::EW => match direction {
                Direction::E => Some(((i, j + 1), Direction::E)),
                Direction::W => Some(((i, j - 1), Direction::W)),
                _ => None,
            },
            PipeType::NE => match direction {
                Direction::S => Some(((i, j + 1), Direction::E)),
                Direction::W => Some(((i - 1, j), Direction::N)),
                _ => None,
            },
            PipeType::NW => match direction {
                Direction::S => Some(((i, j - 1), Direction::W)),
                Direction::E => Some(((i - 1, j), Direction::N)),
                _ => None,
            },
            PipeType::SW => match direction {
                Direction::E => Some(((i + 1, j), Direction::S)),
                Direction::N => Some(((i, j - 1), Direction::W)),
                _ => None,
            },
            PipeType::SE => match direction {
                Direction::W => Some(((i + 1, j), Direction::S)),
                Direction::N => Some(((i, j + 1), Direction::E)),
                _ => None,
            },
        };

        if let Some((point, direction)) = next_unbounded {
            if self.is_inbounds(point) {
                return Some((self.bound_point(point).unwrap(), direction));
            }
        }
        None
    }

    fn get_tile(&self, point: Point) -> &TileType {
        &self.tiles[point.0][point.1]
    }

    fn is_inbounds(&self, point: UnboundedPoint) -> bool {
        point.0 >= 0
            && point.0 < self.n_rows as i64
            && point.1 >= 0
            && point.1 < self.n_columns as i64
    }

    fn find_tile(&self, point: UnboundedPoint) -> Option<&TileType> {
        if !self.is_inbounds(point) {
            return None;
        }
        Some(self.get_tile(self.bound_point(point).unwrap()))
    }

    fn bound_point(&self, point: UnboundedPoint) -> Option<Point> {
        if !self.is_inbounds(point) {
            return None;
        }
        Some((point.0 as usize, point.1 as usize))
    }
}

impl PipeType {
    const VALUES: [Self; 6] = [Self::NS, Self::EW, Self::NE, Self::NW, Self::SE, Self::SW];
}

impl Direction {
    const VALUES: [Self; 4] = [Self::N, Self::E, Self::S, Self::W];
}

impl FromStr for Maze {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles: Vec<Vec<TileType>> = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse::<TileType>().unwrap())
                    .collect()
            })
            .collect();
        Ok(Maze::new(tiles))
    }
}

impl FromStr for TileType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(pipe_type) = s.parse::<PipeType>() {
            return Ok(Self::Pipe(pipe_type));
        }
        match s {
            "." => Ok(Self::Ground),
            "S" => Ok(Self::Start),
            _ => panic!("Unknown tile: {s}"),
        }
    }
}

impl FromStr for PipeType {
    type Err = ParsePipeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "|" => Ok(Self::NS),
            "-" => Ok(Self::EW),
            "L" => Ok(Self::NE),
            "J" => Ok(Self::NW),
            "7" => Ok(Self::SW),
            "F" => Ok(Self::SE),
            _ => Err(ParsePipeError),
        }
    }
}

#[cfg(test)]
mod test {
    use super::solve;
    use indoc::indoc;

    #[test]
    fn solves_sample_1() {
        let sample = indoc! {"
            -L|F7
            7S-7|
            L|7||
            -L-J|
            L|-JF
        "};

        let solution = solve(sample);

        assert_eq!(solution, 4);
    }

    #[test]
    fn solves_sample_2() {
        let sample = indoc! {"
            ..F7.
            .FJ|.
            SJ.L7
            |F--J
            LJ...
        "};

        let solution = solve(sample);

        assert_eq!(solution, 8);
    }
}
