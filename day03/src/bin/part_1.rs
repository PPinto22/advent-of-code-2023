fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve(input));
}

fn solve(input: &str) -> u32 {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut sum = 0;
    for (i, line) in matrix.iter().enumerate() {
        let mut j = 0;
        while j < line.len() {
            let mut number = 0;
            let mut found_adjacent_symbol = false;

            while j < line.len() && matrix[i][j].is_digit(10) {
                found_adjacent_symbol |= is_adjacent_to_symbol(&matrix, i, j);
                number = (number * 10) + matrix[i][j].to_digit(10).unwrap();
                j += 1
            }

            if found_adjacent_symbol {
                sum += number;   
            }
            j += 1
        }
    }
    return sum;
}

fn is_adjacent_to_symbol(matrix: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let n = matrix.len();
    let m = matrix[0].len();
    return (i > 0 && j > 0 && is_symbol(matrix[i - 1][j - 1]))
        || (i > 0 && is_symbol(matrix[i - 1][j]))
        || (i > 0 && j < m - 1 && is_symbol(matrix[i - 1][j + 1]))
        || (j > 0 && is_symbol(matrix[i][j - 1]))
        || (j < m - 1 && is_symbol(matrix[i][j + 1]))
        || (i < n - 1 && j > 0 && is_symbol(matrix[i + 1][j - 1]))
        || (i < n - 1 && is_symbol(matrix[i + 1][j]))
        || (i < n - 1 && j < m - 1 && is_symbol(matrix[i + 1][j + 1]));
}

fn is_symbol(c: char) -> bool {
    return c != '.' && !c.is_alphanumeric();
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

        assert_eq!(solution, 4361);
    }
}
