use std::{collections::HashSet, fs};

pub mod task1 {
    use super::length_to_packet_start;

    pub fn ans() -> u128 {
        length_to_packet_start("resources/2022/day06/input") as u128
    }
}

pub mod task2 {
    use super::length_to_distinct_sequence;

    pub fn ans() -> u128 {
        length_to_distinct_sequence("resources/2022/day06/input", 14) as u128
    }
}

fn length_to_distinct_sequence(file: &str, length: usize) -> u32 {
    let input_contents = fs::read_to_string(file).expect("Error reading file");
    let binding = input_contents.lines().collect::<Vec<&str>>();
    let mut line = binding.first().unwrap().chars();

    let mut i: u32 = length as u32;
    while line.size_hint().1.unwrap() >= length {
        let mut next_four = line.clone().take(length).collect::<Vec<char>>();

        let mut seen = HashSet::new();

        next_four.retain(|x: &char| {
            let is_seen = seen.contains(x);
            seen.insert(*x);
            !is_seen
        });
        if next_four.len() == length {
            return i;
        }

        i += 1;
        line.next();
    }
    0
}

fn length_to_packet_start(file: &str) -> u32 {
    length_to_distinct_sequence(file, 4)
}
