use std::{collections::HashMap, str::Lines};

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve(input));
}

#[derive(Debug)]
struct Network {
    instructions: Vec<char>,
    connections: HashMap<String, NodePair>,
    state: NetworkState,
}

#[derive(Debug)]
struct NodePair {
    left: String,
    right: String,
}

#[derive(Debug)]
struct NetworkState {
    node: String,
    step: usize,
}

fn solve(input: &str) -> u64 {
    let mut network = Network::parse(input);
    network.step_until_end();
    return network.state.step as u64;
}

impl Network {
    fn new(instructions: Vec<char>, connections: HashMap<String, NodePair>) -> Self {
        let state = NetworkState {
            node: "AAA".to_owned(),
            step: 0,
        };
        return Network {
            instructions,
            connections,
            state,
        };
    }

    fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        let instructions: Vec<char> = lines.next().unwrap().chars().collect();
        let _blank_line = lines.next();

        let connections: HashMap<String, NodePair> = lines
            .map(|line| {
                let parts: Vec<&str> = line.split(" = ").collect();
                let source = parts[0].to_string();
                let destinations: Vec<String> = parts[1]
                    .split(", ")
                    .map(|dest| {
                        dest.chars()
                            .filter(|c| c.is_alphanumeric())
                            .collect::<String>()
                    })
                    .collect();
                let destinations_pair = NodePair {
                    left: destinations[0].clone(),
                    right: destinations[1].clone(),
                };
                return (source, destinations_pair);
            })
            .collect();
        return Network::new(instructions, connections);
    }

    fn step(&mut self) {
        let instruction = self.instructions[self.state.step % self.instructions.len()];
        let next_node = self.connections[&self.state.node]
            .get(instruction)
            .to_owned();
        self.state = NetworkState {
            node: next_node,
            step: self.state.step + 1,
        }
    }

    fn step_until_end(&mut self) {
        while self.state.node != "ZZZ" {
            self.step();
        }
    }
}

impl NodePair {
    fn get(&self, instruction: char) -> &String {
        match instruction {
            'L' => &self.left,
            'R' => &self.right,
            _ => panic!("Invalid instruction: {instruction}"),
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
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)
        "};

        let solution = solve(sample);

        assert_eq!(solution, 6);
    }
}
