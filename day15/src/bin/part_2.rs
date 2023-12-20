use std::str::FromStr;

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve(input));
}

#[derive(Debug)]
struct Step {
    label: String,
    operation: Operation,
}

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal_length: u8,
}

#[derive(Debug)]
enum Operation {
    ADD { focal_length: u8 },
    REMOVE,
}

fn solve(input: &str) -> u64 {
    let steps: Vec<Step> = input.split(",").map(|s| s.parse().unwrap()).collect();
    let mut boxes: Vec<Vec<Lens>> = vec![Vec::new(); 256];

    for step in steps.iter() {
        let i = hash(&step.label) as usize;
        let existing_lens = boxes[i]
            .iter()
            .enumerate()
            .find(|(_, lens)| lens.label == step.label);

        match step.operation {
            Operation::ADD { focal_length } => {
                let new_lens = Lens {
                    label: step.label.to_owned(),
                    focal_length,
                };

                if let Some((j, _)) = existing_lens {
                    boxes[i][j] = new_lens;
                } else {
                    boxes[i].push(new_lens);
                }
            }
            Operation::REMOVE => {
                if let Some((j, _)) = existing_lens {
                    boxes[i].remove(j);
                }
            }
        }
    }

    boxes
        .iter()
        .enumerate()
        .map(|(i, lenses)| -> u64 {
            lenses
                .iter()
                .enumerate()
                .map(|(j, lense)| (i as u64 + 1) * (j as u64 + 1) * lense.focal_length as u64)
                .sum()
        })
        .sum()
}

impl FromStr for Step {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(label) = s.strip_suffix('-') {
            return Ok(Step {
                label: label.to_owned(),
                operation: Operation::REMOVE,
            });
        }

        let key_value_split: Vec<&str> = s.split('=').collect();
        let label = key_value_split[0];
        let focal_length: u8 = key_value_split[1].parse().unwrap();
        Ok(Step {
            label: label.to_owned(),
            operation: Operation::ADD { focal_length },
        })
    }
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
    fn solves_sample() {
        let solution = solve("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");

        assert_eq!(solution, 145);
    }
}
