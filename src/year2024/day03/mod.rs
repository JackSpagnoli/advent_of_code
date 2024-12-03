use std::fs;

use regex::Regex;

pub mod task1 {
    use super::sum_mult;

    pub fn ans() -> u128 {
        sum_mult("resources/2024/day03/input.txt")
    }
}

pub mod task2 {
    pub fn ans() -> u128 {
        0
    }
}

struct Instruction {
    a: u128,
    b: u128,
}

fn sum_mult(file: &str) -> u128 {
    fs::read_to_string(file)
        .expect("Error reading file")
        .lines()
        .flat_map(parse_line)
        .map(|instruction| instruction.a * instruction.b)
        .sum()
}

static REGEX: &str = r#"mul\((?<a>\d{1,3}),(?<b>\d{1,3})\)"#;

fn parse_line(line: &str) -> Vec<Instruction> {
    let regex = Regex::new(REGEX).unwrap();

    regex
        .captures_iter(line)
        .map(|capture| {
            let a = capture["a"].parse::<u128>().unwrap();
            let b = capture["b"].parse::<u128>().unwrap();

            Instruction { a, b }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::sum_mult;

    #[test]
    fn task1() {
        let res = sum_mult("resources/2024/day03/test.txt");
        assert_eq!(res, 161);
    }
}
