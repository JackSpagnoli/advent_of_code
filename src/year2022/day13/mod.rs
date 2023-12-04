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
    let mut lines = contents.lines();

    let mut sorted_index_sum: usize = 0;

    let mut pair: usize = 1;
    let mut line1 = lines.next().unwrap();
    let mut line2 = lines.next().unwrap();
    lines.next();

    let mut break_after_next_pass = false;
    loop {
        let parsed_line_1 = parse(line1).unwrap();
        let parsed_line_2 = parse(line2).unwrap();

        if sorted_pair(&parsed_line_1, &parsed_line_2) == 1 {
            sorted_index_sum += pair;
        }
        if break_after_next_pass {
            break;
        }

        line1 = lines.next().unwrap();
        line2 = lines.next().unwrap();
        pair += 1;
        if lines.next() == None {
            break_after_next_pass = true;
        }
    }
    return sorted_index_sum;
}

fn sorted_pair(element_1: &JsonValue, element_2: &JsonValue) -> isize {
    if element_1.is_array() && element_2.is_array() {
        let array_1: Vec<&JsonValue> = element_1.members().collect();
        let array_2: Vec<&JsonValue> = element_2.members().collect();
        for i in 0..array_1.len() {
            if i >= array_2.len() {
                return -1;
            }
            let sorted_pair = sorted_pair(array_1[i], array_2[i]);
            if sorted_pair == -1 || sorted_pair == 1 {
                return sorted_pair;
            }
        }
        return 1;
    }
    if element_1.is_number() && element_2.is_number() {
        if element_1.as_usize().unwrap() < element_2.as_usize().unwrap() {
            return 1;
        } else if element_1.as_usize().unwrap() == element_2.as_usize().unwrap() {
            return 0;
        } else {
            return -1;
        }
    }
    if !element_1.is_array() {
        let mut array_1 = JsonValue::new_array();
        array_1.push(element_1.clone()).expect("Shitters");
        return sorted_pair(&array_1, element_2);
    }
    if !element_2.is_array() {
        let mut array_2 = JsonValue::new_array();
        array_2.push(element_2.clone()).expect("Shitters");
        return sorted_pair(element_1, &array_2);
    }
    return -1;
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
        if lines.next() == None {
            break_after_next_pass = true;
        }
    }

    parsed_lines.push(parse("[[2]]").unwrap());
    parsed_lines.push(parse("[[6]]").unwrap());

    parsed_lines.sort_by(|a, b| {
        let s = sorted_pair(a, b);
        match s {
            1 => {
                return Less;
            }
            -1 => {
                return Greater;
            }
            _ => panic!("Yikes"),
        }
    });

    let mut decoder_key: usize = 1;
    for i in 0..parsed_lines.len() {
        if parsed_lines[i].dump() == "[[2]]".to_owned() {
            decoder_key *= i + 1;
        }
        if parsed_lines[i].dump() == "[[6]]".to_owned() {
            decoder_key *= i + 1;
            return decoder_key;
        }
    }

    return decoder_key;
}
