fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve(input));
}

fn solve(input: &str) -> u64 {
    let steps = input.split(",");
    steps.map(hash).sum()
}

fn hash(s: &str) -> u64 {
    let mut result: u64 = 0;
    for c in s.bytes() {
        result += c as u64;
        result = (result * 17) % 256;
    }
    result
}

#[cfg(test)]
mod test {
    use super::solve;

    #[test]
    fn solves_sample_1() {
        let solution = solve("HASH");

        assert_eq!(solution, 52);
    }

    #[test]
    fn solves_sample_2() {
        let solution = solve("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");

        assert_eq!(solution, 1320);
    }
}
