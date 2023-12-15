pub mod task1 {
    use super::verification_number;

    pub fn ans() -> u128 {
        verification_number("resources/2023/day15/input")
    }
}

pub mod task2 {
    use super::run_operations;

    pub fn ans() -> u128 {
        run_operations("resources/2023/day15/input")
    }
}

fn verification_number(file: &str) -> u128 {
    let contents = std::fs::read_to_string(file).unwrap();

    contents.split(',').map(hash_chars).sum()
}

fn hash_chars(chars: &str) -> u128 {
    chars.chars().map(|c| c as u128).fold(0, |mut acc, char| {
        acc += char;
        acc *= 17;
        acc %= 256;
        acc
    })
}

fn run_operations(file: &str) -> u128 {
    let contents = std::fs::read_to_string(file).unwrap();

    contents
        .split(',')
        .map(parse_operation)
        .fold(Boxes::new(), |boxes, operation| {
            boxes.perform_operation(operation)
        })
        .focusing_power()
}

fn parse_operation(chars: &str) -> Operation {
    let regex = regex::Regex::new(r"(?<Label>[a-z]+)(?<Operation>[=-])(?<Lens>\d?)").unwrap();

    let capture = regex.captures_iter(chars).next().unwrap();
    let label = capture.name("Label").unwrap().as_str().to_string();
    let box_number = hash_chars(&label) as usize;
    let operation = capture.name("Operation").unwrap().as_str();
    match operation {
        "=" => {
            let lens = capture
                .name("Lens")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();
            Operation::Add(AddInstruction {
                label,
                box_number,
                focal_length: lens,
            })
        }
        "-" => Operation::Remove(RemoveInstruction { label, box_number }),
        _ => panic!("Invalid operation"),
    }
}

struct Boxes {
    boxes: Vec<Box>,
}
impl Boxes {
    fn new() -> Self {
        Self {
            boxes: vec![Box { lenses: vec![] }; 256],
        }
    }

    fn perform_operation(mut self, operation: Operation) -> Self {
        match operation {
            Operation::Add(instruction) => {
                let new_lens = Lens {
                    label: instruction.label,
                    focal_length: instruction.focal_length,
                };

                let box_to_update = &mut self.boxes[instruction.box_number];

                if let Some(existing_lens_index) = box_to_update
                    .lenses
                    .iter()
                    .position(|lens| lens.label == new_lens.label)
                {
                    box_to_update.lenses[existing_lens_index] = new_lens;
                } else {
                    box_to_update.lenses.push(new_lens);
                }
            }
            Operation::Remove(instruction) => {
                self.boxes[instruction.box_number]
                    .lenses
                    .retain(|lens| lens.label != instruction.label);
            }
        };
        self
    }

    fn focusing_power(self) -> u128 {
        self.boxes
            .into_iter()
            .enumerate()
            .map(|(index, box_)| (index + 1) * box_.focusing_power())
            .sum::<usize>() as u128
    }
}
#[derive(Clone, Debug)]
struct Box {
    lenses: Vec<Lens>,
}
impl Box {
    fn focusing_power(self) -> usize {
        self.lenses
            .into_iter()
            .enumerate()
            .map(|(index, lens)| (index + 1) * lens.focal_length)
            .sum()
    }
}
#[derive(Debug)]
struct RemoveInstruction {
    label: String,
    box_number: usize,
}
#[derive(Debug)]
struct AddInstruction {
    label: String,
    box_number: usize,
    focal_length: usize,
}
#[derive(Debug)]
enum Operation {
    Remove(RemoveInstruction),
    Add(AddInstruction),
}
#[derive(Clone, Debug)]
struct Lens {
    label: String,
    focal_length: usize,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_verification_number() {
        assert_eq!(verification_number("resources/2023/day15/test_input"), 1320);
    }

    #[test]
    fn test_perform_operations() {
        assert_eq!(run_operations("resources/2023/day15/test_input"), 145);
    }
}
