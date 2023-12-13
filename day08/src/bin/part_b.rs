use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve(input));
}

#[derive(Debug)]
struct Network {
    instructions: Vec<char>,
    connections: HashMap<String, NodePair>,
    states: Vec<NetworkState>,
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

// FIXME Takes too long
fn solve(input: &str) -> u64 {
    let mut network = Network::parse(input);
    network.step_until_end();
    return network.states[0].step as u64;
}

impl Network {
    fn new(instructions: Vec<char>, connections: HashMap<String, NodePair>) -> Self {
        let states: Vec<NetworkState> = connections
            .keys()
            .filter(|node| node.ends_with("A"))
            .map(|node| NetworkState {
                node: node.to_owned(),
                step: 0,
            })
            .collect();
        return Network {
            instructions,
            connections,
            states,
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
        self.states = self
            .states
            .iter()
            .map(|state| {
                let instruction = self.instructions[state.step % self.instructions.len()];
                let next_node = self.connections[&state.node].get(instruction).to_owned();
                return NetworkState {
                    node: next_node,
                    step: state.step + 1,
                };
            })
            .collect();
    }

    fn step_until_end(&mut self) {
        while self.states.iter().any(|state| !state.node.ends_with("Z")) {
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
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
        "};

        let solution = solve(sample);

        assert_eq!(solution, 6);
    }
}
