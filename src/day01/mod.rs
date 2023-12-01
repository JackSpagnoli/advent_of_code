use std::{fs, vec};

pub mod task1 {
    use super::calc_calibration_values;

    pub fn ans() -> u128 {
        calc_calibration_values("resources/day01/input.txt", false)
    }
}

pub mod task2 {
    use super::calc_calibration_values;

    pub fn ans() -> u128 {
        calc_calibration_values("resources/day01/input.txt", true)
    }
}

pub fn calc_calibration_values(file: &str, include_words: bool) -> u128 {
    fs::read_to_string(file)
        .expect("Error reading file")
        .lines()
        .map(|line| calc_calibration_value(line, include_words))
        .sum()
}

fn calc_calibration_value(line: &str, include_words: bool) -> u128 {
    let mut digits = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let mut words = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    if include_words {
        digits.append(&mut words);
    }

    let first = parse_digit(
        digits
            .clone()
            .iter()
            .min_by_key(|digit| line.find(*digit).unwrap_or(usize::MAX))
            .unwrap(),
    );
    let last = parse_digit(
        digits
            .clone()
            .iter()
            .max_by_key(|digit| {match line.rfind(*digit){
                Some(index) => index+1,
                None => 0
            }})
            .unwrap(),
    );
    
    10 * first + last
}

fn parse_digit(digit: &str) -> u128 {
    match digit {
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => panic!("Invalid digit"),
    }
}

#[cfg(test)]
mod test;
