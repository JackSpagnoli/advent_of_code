use std::collections::HashMap;

pub mod task1 {
    use super::steps_to_reach_zzz;

    pub fn ans() -> u128 {
        steps_to_reach_zzz("resources/2023/day08/input")
    }
}

pub mod task2 {
    use super::steps_to_reach_zzz_ghost;

    pub fn ans() -> u128 {
        steps_to_reach_zzz_ghost("resources/2023/day08/input")
    }
}

fn steps_to_reach_zzz(file: &str) -> StepsToReachZZZ {
    parse_map(file).follow_map("AAA", "ZZZ")
}

fn steps_to_reach_zzz_ghost(file: &str) -> StepsToReachZZZ {
    parse_map(file).follow_ghost_map()
}

#[derive(PartialEq, Debug, Clone)]
enum Instruction {
    LeftTurn,
    RightTurn,
}
type NodeId = String;
#[derive(PartialEq, Debug, Clone)]
struct Node {
    left: NodeId,
    right: NodeId,
}
#[derive(PartialEq, Debug)]
struct Map {
    instructions: Vec<Instruction>,
    nodes: HashMap<String, Node>,
}

type StepsToReachZZZ = u128;
impl Map {
    fn follow_map(&self, start_node: &str, end_node_ending: &str) -> StepsToReachZZZ {
        let looping_instructions = get_looping_instructions(&self.instructions);

        let mut current_node = start_node.to_string();

        for (index, instruction) in looping_instructions {
            let node = self.nodes.get(&current_node).unwrap();
            current_node = match instruction {
                Instruction::LeftTurn => node.left.clone(),
                Instruction::RightTurn => node.right.clone(),
            };
            if current_node.ends_with(end_node_ending) {
                return index as StepsToReachZZZ;
            }
        }
        panic!("Bruh")
    }

    fn follow_ghost_map(self) -> StepsToReachZZZ {
        let times: Vec<u128> = self
            .nodes
            .keys()
            .filter(|index| index.ends_with('A'))
            .map(|index| index.to_string())
            .map(|node| self.follow_map(&node, "Z"))
            .collect();

        times.iter().fold(1, |a, b| num::integer::lcm(a, *b))
    }
}

fn get_looping_instructions(
    instructions: &[Instruction],
) -> impl Iterator<Item = (usize, &Instruction)> {
    instructions.clone()
        .iter()
        .cycle()
        .enumerate()
        .map(|(index, instruction)| (index + 1, instruction))
}

fn parse_map(file: &str) -> Map {
    let contents = std::fs::read_to_string(file).expect("Error reading file");

    let mut lines = contents.lines();

    let instructions = parse_instructions(lines.next().unwrap());

    let nodes = parse_nodes(&mut lines.skip(1));

    Map {
        instructions,
        nodes,
    }
}

fn parse_instructions(line: &str) -> Vec<Instruction> {
    line.chars()
        .map(|c| match c {
            'L' => Instruction::LeftTurn,
            'R' => Instruction::RightTurn,
            _ => panic!("Invalid instruction"),
        })
        .collect()
}

fn parse_nodes(lines: &mut dyn Iterator<Item = &str>) -> HashMap<String, Node> {
    lines
        .into_iter()
        .map(parse_line)
        .fold(HashMap::new(), |mut acc, (k, v)| {
            acc.insert(k, v);
            acc
        })
}

fn parse_line(line: &str) -> (NodeId, Node) {
    let regex = regex::Regex::new(
        r"(?P<node_index>.+) = \((?P<left_index>.+), (?P<right_index>.+)\)",
    )
    .unwrap();
    let captures = regex.captures(line).unwrap();
    let node_index = captures.name("node_index").unwrap().as_str().to_string();
    let left_index = captures.name("left_index").unwrap().as_str().to_string();
    let right_index = captures.name("right_index").unwrap().as_str().to_string();
    (
        node_index,
        Node {
            left: left_index,
            right: right_index,
        },
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_map() {
        let file = "resources/2023/day08/test_input";

        let map = parse_map(file);

        let expected_map = Map {
            instructions: vec![Instruction::RightTurn, Instruction::LeftTurn],
            nodes: HashMap::from([
                (
                    "AAA".to_string(),
                    Node {
                        left: "BBB".to_string(),
                        right: "CCC".to_string(),
                    },
                ),
                (
                    "BBB".to_string(),
                    Node {
                        left: "DDD".to_string(),
                        right: "EEE".to_string(),
                    },
                ),
                (
                    "CCC".to_string(),
                    Node {
                        left: "ZZZ".to_string(),
                        right: "GGG".to_string(),
                    },
                ),
                (
                    "DDD".to_string(),
                    Node {
                        left: "DDD".to_string(),
                        right: "DDD".to_string(),
                    },
                ),
                (
                    "EEE".to_string(),
                    Node {
                        left: "EEE".to_string(),
                        right: "EEE".to_string(),
                    },
                ),
                (
                    "GGG".to_string(),
                    Node {
                        left: "GGG".to_string(),
                        right: "GGG".to_string(),
                    },
                ),
                (
                    "ZZZ".to_string(),
                    Node {
                        left: "ZZZ".to_string(),
                        right: "ZZZ".to_string(),
                    },
                ),
            ]),
        };

        assert_eq!(map, expected_map);
    }

    #[test]
    fn test_follow_map() {
        let file = "resources/2023/day08/test_input";

        let steps = steps_to_reach_zzz(file);

        assert_eq!(steps, 2);
    }

    #[test]
    fn test_follow_ghost_map() {
        let file = "resources/2023/day08/test_input2";

        let steps = steps_to_reach_zzz_ghost(file);

        assert_eq!(steps, 6);
    }
}
