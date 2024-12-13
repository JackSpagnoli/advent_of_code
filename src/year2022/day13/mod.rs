use itertools::Itertools;
use json::number::Number;
use json::{parse, JsonValue};
use std::cmp::Ordering::{self, Equal, Greater, Less};
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

            let sort = sorted_pair(line_1.clone(), line_2.clone());

            sort == Less
        })
        .map(|(i, _)| i + 1)
        .sum()
}

fn sorted_pair(element_1: JsonValue, element_2: JsonValue) -> Ordering {
    match (element_1, element_2) {
        (JsonValue::Array(a), JsonValue::Array(b)) => {
            let a_len = a.len();
            let b_len = b.len();

            for (a, b) in a.into_iter().zip(b.into_iter()) {
                let sorted_pair = sorted_pair(a, b);
                if sorted_pair != Equal {
                    return sorted_pair;
                }
            }
            if b_len < a_len {
                return Greater;
            } else if b_len > a_len {
                return Less;
            }
            return Equal;
        }
        (JsonValue::Number(a), JsonValue::Number(b)) => {
            let a = unwrap_number(a);
            let b = unwrap_number(b);
            return a.cmp(&b);
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
    let dividers = vec![json::parse("[[2]]").unwrap(), json::parse("[[6]]").unwrap()];

    let contents = fs::read_to_string(file).expect("Error reading file");
    contents
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| parse(line).unwrap())
        .chain(dividers.clone().into_iter())
        .sorted_by(|a, b| sorted_pair(a.clone(), b.clone()))
        .enumerate()
        .filter(|(_, line)| dividers.contains(line))
        .map(|(i, _)| i + 1)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_file_sorting() {
        assert_eq!(
            check_file_sorting("resources/2022/day13/test_input.txt"),
            13
        );
    }

    #[test]
    fn test_decoder_key() {
        assert_eq!(decoder_key("resources/2022/day13/test_input.txt"), 140);
    }

    #[test]
    fn test_sort() {
        let a = json::parse("[[1],4]").unwrap();
        let b = json::parse("[1,1,3,1,1]").unwrap();

        assert_eq!(sorted_pair(a, b), Greater);
    }
}
