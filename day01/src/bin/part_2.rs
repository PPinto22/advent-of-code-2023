fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve(input));
}

const DIGIT_WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn solve(input: &str) -> u64 {
    return input
        .lines()
        .map(|line| find_two_digit_number(line) as u64)
        .sum();
}

fn find_two_digit_number(line: &str) -> u32 {
    let digits: Vec<u32> = line
        .char_indices()
        .filter_map(|(i, _)| find_starting_digit(&line[i..]))
        .collect();
    return digits.first().unwrap() * 10 + digits.last().unwrap();
}

fn find_starting_digit(slice: &str) -> Option<u32> {
    let first_char = slice.chars().next().unwrap();
    if first_char.is_digit(10) {
        return first_char.to_digit(10);
    }
    return map_text_to_number(slice);
}

fn map_text_to_number(slice: &str) -> Option<u32> {
    for (i, digit_word) in DIGIT_WORDS.iter().enumerate() {
        if slice.starts_with(digit_word) {
            return Some(i as u32 + 1);
        }
    }
    return None;
}

#[cfg(test)]
mod test {
    use super::solve;
    use indoc::indoc;

    #[test]
    fn solves_sample() {
        let sample = indoc! {"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        "};

        let solution = solve(sample);

        assert_eq!(solution, 281);
    }
}
