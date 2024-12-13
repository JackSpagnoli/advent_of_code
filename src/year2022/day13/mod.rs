use json::number::Number;
use json::{parse, JsonValue};
use std::cmp::Ordering::{Greater, Less};
use std::fs;

pub mod task1 {
    use super::check_file_sorting;

    pub fn ans() -> u128 {
        check_file_sorting("resources/2022/day13/input") as u128
    }
}

pub mod task2 {
    use super::decoder_key;

    pub fn ans() -> u128 {
        decoder_key("resources/2022/day13/input") as u128
    }
}

fn check_file_sorting(file: &str) -> usize {
    let contents = fs::read_to_string(file).expect("Error reading file");
    contents
        .split("\n\n")
        .enumerate()
        .filter(|(_, packets)| {
            let mut lines = packets.lines().map(parse);
            let line_1 = lines.next().unwrap().unwrap();
            let line_2 = lines.next().unwrap().unwrap();

            sorted_pair(line_1, line_2).unwrap_or(false)
        })
        .map(|(i, _)| i + 1)
        .sum()
}

fn sorted_pair(element_1: JsonValue, element_2: JsonValue) -> Option<bool> {
    match (element_1, element_2) {
        (JsonValue::Array(a), JsonValue::Array(b)) => {
            let a_len = a.len();
            let b_len = b.len();

            for (a, b) in a.into_iter().zip(b.into_iter()) {
                let sorted_pair = sorted_pair(a, b);
                if sorted_pair.is_some() {
                    return sorted_pair;
                }
            }
            if b_len < a_len {
                return Some(false);
            }
            return Some(true);
        }
        (JsonValue::Number(a), JsonValue::Number(b)) => {
            let a = unwrap_number(a);
            let b = unwrap_number(b);
            return match b.cmp(&a) {
                Less => Some(false),
                Greater => Some(true),
                _ => None,
            };
        }
        (a, JsonValue::Number(b)) => {
            let b = JsonValue::Array(vec![JsonValue::Number(b)]);
            return sorted_pair(a, b);
        }
        (JsonValue::Number(a), b) => {
            let a = JsonValue::Array(vec![JsonValue::Number(a)]);
            return sorted_pair(a, b);
        }
        (_, _) => panic!("Yikes"),
    }
}

fn unwrap_number(number: Number) -> usize {
    match usize::try_from(number) {
        Ok(a) => a,
        Err(_) => panic!("Yikes"),
    }
}

fn decoder_key(file: &str) -> usize {
    let contents = fs::read_to_string(file).expect("Error reading file");
    let mut lines = contents.lines();

    let mut parsed_lines: Vec<JsonValue> = vec![];

    let mut line1 = lines.next().unwrap();
    let mut line2 = lines.next().unwrap();
    lines.next();

    let mut break_after_next_pass = false;
    loop {
        parsed_lines.push(parse(line1).unwrap());
        parsed_lines.push(parse(line2).unwrap());

        if break_after_next_pass {
            break;
        }

        line1 = lines.next().unwrap();
        line2 = lines.next().unwrap();
        if lines.next().is_none() {
            break_after_next_pass = true;
        }
    }

    parsed_lines.push(parse("[[2]]").unwrap());
    parsed_lines.push(parse("[[6]]").unwrap());

    parsed_lines.sort_by(|a, b| {
        let s = sorted_pair(a.clone(), b.clone());
        match s {
            Some(true) => Less,
            Some(false) => Greater,
            _ => panic!("Yikes"),
        }
    });

    let mut decoder_key: usize = 1;
    for (i, parsed_line) in parsed_lines.iter().enumerate() {
        if parsed_line.dump() == *"[[2]]" {
            decoder_key *= i + 1;
        }
        if parsed_line.dump() == *"[[6]]" {
            decoder_key *= i + 1;
            return decoder_key;
        }
    }

    decoder_key
}
