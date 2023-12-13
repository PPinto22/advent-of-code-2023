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

#[derive(Debug, Clone)]
struct NetworkState {
    node: String,
    step: usize,
}

fn solve(input: &str) -> u64 {
    let network = Network::parse(input);
    return network.count_steps_until_end();
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

    fn count_steps_until_end(&self) -> u64 {
        let final_states = self
            .states
            .iter()
            .map(|state| self.step_state_until_end(state));
        return final_states
            .map(|state| state.step as u64)
            .reduce(|acc, steps| num::integer::lcm(acc, steps))
            .unwrap();
    }

    fn step_state_until_end(&self, state: &NetworkState) -> NetworkState {
        let mut state = state.clone();
        while !state.node.ends_with("Z") {
            let instruction = self.instructions[state.step % self.instructions.len()];
            let next_node = self.connections[&state.node].get(instruction).to_owned();

            state = NetworkState {
                node: next_node,
                step: state.step + 1,
            }
        }
        return state.clone();
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
