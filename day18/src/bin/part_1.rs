use std::str::FromStr;

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve(input));
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Point(isize, isize);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Instruction {
    direction: Direction,
    meters: u32,
}

struct Polygon {
    vertices: Vec<Point>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    R,
    D,
    L,
    U,
}

fn solve(input: &str) -> u32 {
    let instructions: Vec<Instruction> = input.lines().map(|l| l.parse().unwrap()).collect();
    let dig_map = Polygon::from_instructions(&instructions);
    dig_map.calculate_area()
}

impl Polygon {
    fn from_instructions(instructions: &Vec<Instruction>) -> Self {
        let mut position = Point(0, 0);
        let mut vertices = vec![position.clone()];

        for instruction in instructions {
            position = Polygon::next_position(&position, instruction);
            vertices.push(position.clone());
        }

        Polygon { vertices }
    }

    fn calculate_area(&self) -> u32 {
        // shoelace formula
        let double_interior_area: u32 = self
            .vertices
            .windows(2)
            .map(|pair| (pair[0].0 * pair[1].1) as i32 - (pair[0].1 * pair[1].0) as i32)
            .sum::<i32>()
            .unsigned_abs();

        let mut double_area = double_interior_area;

        // edges
        self.vertices.windows(2).for_each(|pair| {
            double_area += (pair[0].0 as i32 - pair[1].0 as i32).unsigned_abs()
                + (pair[0].1 as i32 - pair[1].1 as i32).unsigned_abs()
        });

        double_area / 2 + 1
    }

    fn next_position(position: &Point, instruction: &Instruction) -> Point {
        let Point(i, j) = *position;

        match instruction.direction {
            Direction::R => Point(i, j + instruction.meters as isize),
            Direction::D => Point(i + instruction.meters as isize, j),
            Direction::L => Point(i, j - instruction.meters as isize),
            Direction::U => Point(i - instruction.meters as isize, j),
        }
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();

        Ok(Instruction {
            direction: parts[0].parse().unwrap(),
            meters: parts[1].parse().unwrap(),
        })
    }
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Direction::R),
            "D" => Ok(Direction::D),
            "L" => Ok(Direction::L),
            "U" => Ok(Direction::U),
            other => panic!("Unknown direction: {other}"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::solve;
    use indoc::indoc;

    #[test]
    fn solves_sample() {
        let sample = indoc! {"
            R 6 (#70c710)
            D 5 (#0dc571)
            L 2 (#5713f0)
            D 2 (#d2c081)
            R 2 (#59c680)
            D 2 (#411b91)
            L 5 (#8ceee2)
            U 2 (#caa173)
            L 1 (#1b58a2)
            U 2 (#caa171)
            R 2 (#7807d2)
            U 3 (#a77fa3)
            L 2 (#015232)
            U 2 (#7a21e3)
        "};

        let solution = solve(sample);

        assert_eq!(solution, 62);
    }
}
