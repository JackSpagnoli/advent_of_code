use std::fs;

pub mod task1 {
    use super::max_calories;

    pub fn ans() -> u128 {
        max_calories("resources/2022/day01/input")
    }
}

pub mod task2 {
    use super::top_three_calories;

    pub fn ans() -> u128 {
        top_three_calories("resources/2022/day01/input")
    }
}

fn max_calories(file: &str) -> u128 {
    parse_calories(file)
        .into_iter()
        .max()
        .unwrap()
}

fn parse_calories(file: &str) -> Vec<u128> {
    let contents = fs::read_to_string(file).expect("Error reading file");
    contents
        .lines()
        .fold((vec![], 0), |(mut calories, current_pack), line| {
            if line.is_empty() {
                calories.push(current_pack);
                (calories, 0)
            } else {
                let line_calories = line.parse::<u128>().unwrap();
                (calories, current_pack + line_calories)
            }
        })
        .0
}

fn top_three_calories(file: &str) -> u128 {
    let mut calories = parse_calories(file);
    calories.sort();
    calories.into_iter().rev().take(3).sum()
}