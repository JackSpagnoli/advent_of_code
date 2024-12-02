use std::{collections::HashMap, fs};

pub mod task1 {
    use super::{parse_pairs, sort_halves, Pair};

    pub fn ans() -> u128 {
        let pairs = parse_pairs("resources/2024/day01/input.txt");
        sort_halves(pairs).into_iter().map(Pair::distance).sum()
    }
}

pub mod task2 {
    use super::{count_occurances, parse_pairs};

    pub fn ans() -> u128 {
        let pairs = parse_pairs("resources/2024/day01/input.txt");
        let (left, right) = pairs.into_iter().fold((vec![], vec![]), |mut acc, pair| {
            acc.0.push(pair.left);
            acc.1.push(pair.right);
            acc
        });

        let occurances = count_occurances(right);

        left.into_iter()
            .map(|num| num as u128 * occurances.get(&num).unwrap_or(&0))
            .sum::<u128>()
    }
}

struct Pair {
    left: i128,
    right: i128,
}

impl From<&str> for Pair {
    fn from(line: &str) -> Self {
        let mut halves = line.split("   ");
        let left = i128::from_str_radix(halves.next().unwrap(), 10).unwrap();
        let right = i128::from_str_radix(halves.next().unwrap(), 10).unwrap();

        Pair { left, right }
    }
}

impl Pair {
    fn distance(self) -> u128 {
        self.left.abs_diff(self.right)
    }
}

fn parse_pairs(file_path: &str) -> Vec<Pair> {
    fs::read_to_string(file_path)
        .expect("Error reading file")
        .lines()
        .map(Pair::from)
        .collect()
}

fn sort_halves(pairs: Vec<Pair>) -> Vec<Pair> {
    let (mut left, mut right) = pairs.into_iter().fold((vec![], vec![]), |mut acc, pair| {
        acc.0.push(pair.left);
        acc.1.push(pair.right);
        acc
    });

    left.sort_unstable();
    right.sort_unstable();

    left.into_iter()
        .zip(right.into_iter())
        .map(|(left, right)| Pair { left, right })
        .collect()
}

fn count_occurances(nums: Vec<i128>) -> HashMap<i128, u128> {
    let mut map: HashMap<i128, u128> = HashMap::new();

    nums.into_iter().for_each(|num| {
        let current = map.get(&num).unwrap_or(&0);

        map.insert(num, current + 1);
    });

    map
}

#[cfg(test)]
mod tests {
    use crate::year2024::day01::count_occurances;

    use super::{parse_pairs, sort_halves, Pair};

    #[test]
    fn test_task_1() {
        let pairs = parse_pairs("resources/2024/day01/task_1_test.txt");
        let dist: u128 = sort_halves(pairs).into_iter().map(Pair::distance).sum();

        assert_eq!(dist, 11)
    }
    #[test]
    fn test_task_2() {
        let pairs = parse_pairs("resources/2024/day01/task_1_test.txt");
        let (left, right) = pairs.into_iter().fold((vec![], vec![]), |mut acc, pair| {
            acc.0.push(pair.left);
            acc.1.push(pair.right);
            acc
        });

        let occurances = count_occurances(right);

        let metric = left
            .into_iter()
            .map(|num| num as u128 * occurances.get(&num).unwrap_or(&0))
            .sum::<u128>();

        assert_eq!(metric, 31);
    }
}
