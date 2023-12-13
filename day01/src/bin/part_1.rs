fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve(input));
}

fn solve(input: &str) -> u64 {
    return input
        .lines()
        .map(|line| find_two_digit_number(line) as u64)
        .sum();
}

fn find_two_digit_number(line: &str) -> u32 {
    let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
    return digits.first().unwrap() * 10 + digits.last().unwrap();
}

#[cfg(test)]
mod test {
    use super::solve;
    use indoc::indoc;

    #[test]
    fn solves_sample() {
        let sample = indoc! {"
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "};

        let solution = solve(sample);

        assert_eq!(solution, 142);
    }
}
