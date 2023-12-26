use std::str::FromStr;

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve(input));
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Point(i128, i128);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Instruction {
    direction: Direction,
    meters: u128,
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

fn solve(input: &str) -> u128 {
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

    fn calculate_area(&self) -> u128 {
        // shoelace formula
        let double_interior_area: u128 = self
            .vertices
            .windows(2)
            .map(|pair| (pair[0].0 * pair[1].1) - (pair[0].1 * pair[1].0))
            .sum::<i128>()
            .unsigned_abs();

        let mut double_area = double_interior_area;

        // edges
        self.vertices.windows(2).for_each(|pair| {
            double_area +=
                (pair[0].0 - pair[1].0).unsigned_abs() + (pair[0].1 - pair[1].1).unsigned_abs()
        });

        double_area / 2 + 1
    }

    fn next_position(position: &Point, instruction: &Instruction) -> Point {
        let Point(i, j) = *position;

        match instruction.direction {
            Direction::R => Point(i, j + instruction.meters as i128),
            Direction::D => Point(i + instruction.meters as i128, j),
            Direction::L => Point(i, j - instruction.meters as i128),
            Direction::U => Point(i - instruction.meters as i128, j),
        }
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let hex_digits = parts[2]
            .strip_prefix("(#")
            .unwrap()
            .strip_suffix(")")
            .unwrap();

        let direction = match hex_digits.chars().last() {
            Some('0') => Direction::R,
            Some('1') => Direction::D,
            Some('2') => Direction::L,
            Some('3') => Direction::U,
            _ => panic!("Unknown direction"),
        };
        let meters = u128::from_str_radix(&hex_digits[0..hex_digits.len() - 1], 16).unwrap();

        Ok(Instruction { direction, meters })
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

        assert_eq!(solution, 952408144115);
    }
}
