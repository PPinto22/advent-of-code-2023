use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashSet},
    rc::Rc,
    str::FromStr,
};

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve(input));
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Point(i64, i64);

struct Solver {
    city: City,
    candidates: BinaryHeap<Reverse<PathNode>>,
    visited: HashSet<VisitedNode>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct PathNode {
    previous: Option<Rc<PathNode>>,
    total_heat_loss: u64,
    position: Point,
    direction: Direction,
    consecutive_staight_moves: u8,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct VisitedNode {
    position: Point,
    direction: Direction,
    consecutive_straight_moves: u8,
}

#[derive(Debug)]
struct City {
    rows: Vec<Vec<u8>>,
    n_rows: usize,
    n_columns: usize,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Direction {
    N,
    E,
    S,
    W,
}

fn solve(input: &str) -> u64 {
    let city: City = input.parse().unwrap();
    let mut solver = Solver::new(city);
    let path_tail = solver.find_min_path();

    solver.print_path(&path_tail);

    path_tail.total_heat_loss
}

impl Solver {
    fn new(city: City) -> Solver {
        let start = PathNode {
            previous: None,
            total_heat_loss: 0,
            position: Point(0, 0),
            direction: Direction::E,
            consecutive_staight_moves: 0,
        };

        Solver {
            city,
            candidates: vec![Reverse(start)].into_iter().collect(),
            visited: HashSet::new(),
        }
    }

    fn print_path(&self, path_tail: &PathNode) {
        let mut visited_positions: HashSet<Point> = HashSet::new();
        let mut node: Option<Rc<PathNode>> = Some(Rc::new(path_tail.clone()));
        while node.is_some() {
            visited_positions.insert(node.as_ref().unwrap().position.clone());
            node = node.unwrap().previous.clone();
        }

        for i in 0..self.city.n_rows {
            for j in 0..self.city.n_columns {
                let position = Point(i as i64, j as i64);
                let symbol = match visited_positions.contains(&position) {
                    true => '.',
                    false => char::from_digit(self.city.get(&position) as u32, 10).unwrap(),
                };
                print!("{symbol}")
            }
            println!("")
        }
    }

    fn find_min_path(&mut self) -> PathNode {
        while !self.candidates.is_empty() {
            let node = self.candidates.pop().unwrap().0;
            if self.is_at_destination(&node) {
                return node;
            }

            let visited_node = VisitedNode {
                position: node.position.clone(),
                direction: node.direction.clone(),
                consecutive_straight_moves: node.consecutive_staight_moves,
            };
            if self.visited.contains(&visited_node) {
                continue;
            }
            self.visited.insert(visited_node);

            let mut next_candidates: BinaryHeap<Reverse<PathNode>> = vec![
                self.turn_left(&node),
                self.turn_right(&node),
                self.continue_straight(&node),
            ]
            .into_iter()
            .flatten()
            .map(|candidate| Reverse(candidate))
            .collect();

            self.candidates.append(&mut next_candidates);
        }
        panic!("Did not reach destination")
    }

    fn turn_left(&self, node: &PathNode) -> Option<PathNode> {
        let Point(i, j) = node.position;
        let (next_position, next_direction) = match node.direction {
            Direction::N => (Point(i, j - 1), Direction::W),
            Direction::E => (Point(i - 1, j), Direction::N),
            Direction::S => (Point(i, j + 1), Direction::E),
            Direction::W => (Point(i + 1, j), Direction::S),
        };

        if self.city.is_out_of_bounds(&next_position) {
            return None;
        }

        Some(PathNode {
            previous: Some(Rc::new(node.clone())),
            total_heat_loss: node.total_heat_loss + self.city.get(&next_position) as u64,
            position: next_position,
            direction: next_direction,
            consecutive_staight_moves: 1,
        })
    }

    fn turn_right(&self, node: &PathNode) -> Option<PathNode> {
        let Point(i, j) = node.position;
        let (next_position, next_direction) = match node.direction {
            Direction::N => (Point(i, j + 1), Direction::E),
            Direction::E => (Point(i + 1, j), Direction::S),
            Direction::S => (Point(i, j - 1), Direction::W),
            Direction::W => (Point(i - 1, j), Direction::N),
        };

        if self.city.is_out_of_bounds(&next_position) {
            return None;
        }

        Some(PathNode {
            previous: Some(Rc::new(node.clone())),
            total_heat_loss: node.total_heat_loss + self.city.get(&next_position) as u64,
            position: next_position,
            direction: next_direction,
            consecutive_staight_moves: 1,
        })
    }

    fn continue_straight(&self, node: &PathNode) -> Option<PathNode> {
        if node.consecutive_staight_moves == 3 {
            return None;
        }

        let Point(i, j) = node.position;
        let next_position = match node.direction {
            Direction::N => Point(i - 1, j),
            Direction::E => Point(i, j + 1),
            Direction::S => Point(i + 1, j),
            Direction::W => Point(i, j - 1),
        };

        if self.city.is_out_of_bounds(&next_position) {
            return None;
        }

        Some(PathNode {
            previous: Some(Rc::new(node.clone())),
            total_heat_loss: node.total_heat_loss + self.city.get(&next_position) as u64,
            position: next_position,
            direction: node.direction.clone(),
            consecutive_staight_moves: node.consecutive_staight_moves + 1,
        })
    }

    fn is_at_destination(&self, node: &PathNode) -> bool {
        node.position.0 == self.city.n_rows as i64 - 1
            && node.position.1 == self.city.n_columns as i64 - 1
    }
}

impl City {
    fn new(rows: Vec<Vec<u8>>) -> City {
        let n_rows = rows.len();
        let n_columns = rows[0].len();

        City {
            rows,
            n_rows,
            n_columns,
        }
    }

    fn get(&self, point: &Point) -> u8 {
        self.rows[point.0 as usize][point.1 as usize]
    }

    fn is_out_of_bounds(&self, point: &Point) -> bool {
        let Point(i, j) = *point;
        i < 0 || i >= self.n_rows as i64 || j < 0 || j >= self.n_columns as i64
    }
}

impl FromStr for City {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows: Vec<Vec<u8>> = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect();
        Ok(City::new(rows))
    }
}

impl PartialOrd for PathNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.total_heat_loss, self.consecutive_staight_moves)
            .cmp(&(other.total_heat_loss, other.consecutive_staight_moves))
    }
}

#[cfg(test)]
mod test {
    use super::solve;
    use indoc::indoc;

    #[test]
    fn solves_sample() {
        let sample = indoc! {"
            2413432311323
            3215453535623
            3255245654254
            3446585845452
            4546657867536
            1438598798454
            4457876987766
            3637877979653
            4654967986887
            4564679986453
            1224686865563
            2546548887735
            4322674655533
        "};

        let solution = solve(sample);

        assert_eq!(solution, 102);
    }
}
