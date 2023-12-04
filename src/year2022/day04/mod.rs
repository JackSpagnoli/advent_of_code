use std::fs;
use std::str::FromStr;

pub mod task1 {
    use super::overlapping_pairs;

    pub fn ans() -> u128 {
        overlapping_pairs("resources/2022/day04/input") as u128
    }
}

pub mod task2 {
    use super::strict_overlapping_pairs;

    pub fn ans() -> u128 {
        strict_overlapping_pairs("resources/2022/day04/input") as u128
    }
}

fn overlapping_pairs(file: &str) -> u32 {
    let contents = fs::read_to_string(file).expect("Error reading file");
    let split_contents = contents.lines();

    return split_contents.fold(0, |n, x| {
        let mut split_line = x.split(',');

        let mut left_range = split_line.next().unwrap().split('-');
        let left_a = <u32 as FromStr>::from_str(left_range.next().unwrap()).unwrap();
        let left_b = <u32 as FromStr>::from_str(left_range.next().unwrap()).unwrap();

        let mut right_range = split_line.next().unwrap().split('-');
        let right_a = <u32 as FromStr>::from_str(right_range.next().unwrap()).unwrap();
        let right_b = <u32 as FromStr>::from_str(right_range.next().unwrap()).unwrap();

        if (left_a >= right_a && left_b <= right_b) || (left_a <= right_a && left_b >= right_b) {
            n + 1
        } else {
            n
        }
    });
}

fn strict_overlapping_pairs(file: &str) -> u32 {
    let contents = fs::read_to_string(file).expect("Error reading file");
    let split_contents = contents.lines();

    return split_contents.fold(0, |n, x| {
        let mut split_line = x.split(',');

        let mut left_range = split_line.next().unwrap().split('-');
        let left_a = <u32 as FromStr>::from_str(left_range.next().unwrap()).unwrap();
        let left_b = <u32 as FromStr>::from_str(left_range.next().unwrap()).unwrap();

        let mut right_range = split_line.next().unwrap().split('-');
        let right_a = <u32 as FromStr>::from_str(right_range.next().unwrap()).unwrap();
        let right_b = <u32 as FromStr>::from_str(right_range.next().unwrap()).unwrap();

        if (left_a >= right_a && left_a <= right_b)
            || (left_b >= right_a && left_b <= right_b)
            || (right_a >= left_a && right_a <= left_b)
            || (right_b >= left_a && right_b <= left_b)
        {
            n + 1
        } else {
            n
        }
    });
}
