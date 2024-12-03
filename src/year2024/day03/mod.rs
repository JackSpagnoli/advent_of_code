use regex::{Captures, Regex};
use std::fs;

pub mod task1 {
    use super::sum_mult;

    pub fn ans() -> u128 {
        sum_mult("resources/2024/day03/input.txt")
    }
}

pub mod task2 {
    use super::sum_with_break;

    pub fn ans() -> u128 {
        sum_with_break("resources/2024/day03/input.txt")
    }
}

struct Instruction {
    a: u128,
    b: u128,
}

impl From<Captures<'_>> for Instruction {
    fn from(value: Captures) -> Self {
        let a = value["a"].parse::<u128>().unwrap();
        let b = value["b"].parse::<u128>().unwrap();

        Instruction { a, b }
    }
}

fn sum_mult(file: &str) -> u128 {
    let input = fs::read_to_string(file).expect("Error reading file");

    parse_line(&input)
        .into_iter()
        .map(|instruction| instruction.a * instruction.b)
        .sum()
}

fn sum_with_break(file: &str) -> u128 {
    let input = fs::read_to_string(file).expect("Error reading file");

    parse_line_with_toggle(&input)
        .into_iter()
        .map(|instruction| instruction.a * instruction.b)
        .sum()
}

static MUL_REGEX: &str = r#"mul\((?<a>\d{1,3}),(?<b>\d{1,3})\)"#;

fn parse_line(line: &str) -> Vec<Instruction> {
    let regex = Regex::new(MUL_REGEX).unwrap();

    regex.captures_iter(line).map(Instruction::from).collect()
}

fn parse_line_with_toggle(line: &str) -> Vec<Instruction> {
    if line.is_empty() {
        return Vec::new();
    }
    let pause_break = line.split_once("don't()").unwrap_or((line, ""));
    let (pre_break, post_break) = pause_break;

    let pre_instructions = parse_line(pre_break);

    let enable_break = post_break.split_once("do()").unwrap_or(("", ""));
    let (_pre_enable, post_enable) = enable_break;
    let post_instructions = parse_line_with_toggle(post_enable);

    pre_instructions
        .into_iter()
        .chain(post_instructions.into_iter())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::year2024::day03::sum_with_break;

    use super::sum_mult;

    #[test]
    fn task1() {
        let res = sum_mult("resources/2024/day03/task_1_test.txt");
        assert_eq!(res, 161);
    }
    #[test]
    fn task2() {
        let res = sum_with_break("resources/2024/day03/task_2_test.txt");
        assert_eq!(res, 48);
    }
}
