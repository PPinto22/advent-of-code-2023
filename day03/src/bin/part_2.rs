fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve(input));
}

type Point = (usize, usize);

struct NumberPosition {
    number: u32,
    start: Point,
    end: Point,
}

impl NumberPosition {
    fn intersects(&self, (i, j): Point) -> bool {
        return self.row() == i && j >= self.start.1 && j <= self.end.1;
    }

    fn intersects_any(&self, points: &Vec<Point>) -> bool {
        return points.iter().any(|p| self.intersects(*p));
    }

    fn row(&self) -> usize {
        return self.start.0;
    }
}

fn solve(input: &str) -> u32 {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut numbers_adjacent_to_gears: Vec<NumberPosition> = Vec::new();
    let mut gears: Vec<Point> = Vec::new();

    for (i, line) in matrix.iter().enumerate() {
        let mut j = 0;
        while j < line.len() {
            let mut k = j;
            let mut number = 0;
            let mut found_adjacent_gear = false;

            while k < line.len() && matrix[i][k].is_digit(10) {
                found_adjacent_gear |= is_adjacent_to_gear(&matrix, (i, k));
                number = (number * 10) + matrix[i][k].to_digit(10).unwrap();
                k += 1
            }

            if k < line.len() && matrix[i][k] == '*' {
                gears.push((i, k));
            }

            if found_adjacent_gear {
                numbers_adjacent_to_gears.push(NumberPosition {
                    number,
                    start: (i, j),
                    end: (i, k - 1),
                });
            }
            j = k + 1;
        }
    }

    let mut gear_ratio = 0;

    for gear in gears {
        let gear_adjacent_indexes = adjacent_indexes(&matrix, gear);
        let adjacent_numbers: Vec<u32> = numbers_adjacent_to_gears
            .iter()
            .filter(|number| number.intersects_any(&gear_adjacent_indexes))
            .map(|np| np.number)
            .collect();
        if adjacent_numbers.len() == 2 {
            gear_ratio += adjacent_numbers[0] * adjacent_numbers[1];
        }
    }

    return gear_ratio;
}

fn is_adjacent_to_gear(matrix: &Vec<Vec<char>>, (i, j): Point) -> bool {
    return adjacent_indexes(&matrix, (i, j))
        .iter()
        .any(|(adj_i, adj_j)| matrix[*adj_i][*adj_j] == '*');
}

fn adjacent_indexes(matrix: &Vec<Vec<char>>, (i, j): Point) -> Vec<Point> {
    let n = matrix.len();
    let m = matrix[0].len();

    let mut result: Vec<Point> = Vec::new();

    if i > 0 && j > 0 {
        result.push((i - 1, j - 1))
    };
    if i > 0 {
        result.push((i - 1, j))
    };
    if i > 0 && j < m - 1 {
        result.push((i - 1, j + 1))
    };
    if j > 0 {
        result.push((i, j - 1))
    };
    if j < m - 1 {
        result.push((i, j + 1))
    };
    if i < n - 1 && j > 0 {
        result.push((i + 1, j - 1))
    };
    if i < n - 1 {
        result.push((i + 1, j))
    };
    if i < n - 1 && j < m - 1 {
        result.push((i + 1, j + 1))
    };

    return result;
}

#[cfg(test)]
mod test {
    use super::solve;
    use indoc::indoc;

    #[test]
    fn solves_sample() {
        let sample = indoc! {"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "};

        let solution = solve(sample);

        assert_eq!(solution, 467835);
    }
}
