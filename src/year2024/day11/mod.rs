use std::collections::HashMap;

pub mod task1 {
    pub fn ans() -> u128 {
        super::count_stones("resources/2024/day11/input.txt", 25)
    }
}

pub mod task2 {
    pub fn ans() -> u128 {
        super::count_stones("resources/2024/day11/input.txt", 75)
    }
}

type NumberIterationsPair = (u128, u128);
fn count_stones(file: &str, iterations: u128) -> u128 {
    let content = std::fs::read_to_string(file).unwrap();

    let starting_stones = content
        .split_ascii_whitespace()
        .map(|s| s.parse::<u128>().unwrap())
        .collect::<Vec<u128>>();

    let mut stone_sizes: HashMap<NumberIterationsPair, u128> = HashMap::new();

    starting_stones.into_iter().fold(0, |acc, stone| {
        acc + count_stones_recursive(&mut stone_sizes, stone, iterations)
    })
}

fn count_stones_recursive(
    stone_sizes: &mut HashMap<NumberIterationsPair, u128>,
    stone: u128,
    iterations: u128,
) -> u128 {
    if iterations == 0 {
        return 1;
    }

    if let Some(count) = stone_sizes.get(&(stone, iterations)) {
        return *count;
    }

    if stone == 0 {
        let count = count_stones_recursive(stone_sizes, 1, iterations - 1);
        stone_sizes.insert((stone, iterations), count);
        return count;
    }

    let stone_digits: Vec<char> = stone.to_string().chars().collect();
    if stone_digits.len() % 2 == 0 {
        let left = stone_digits[0..stone_digits.len() / 2]
            .iter()
            .collect::<String>()
            .parse::<u128>()
            .unwrap();
        let right = stone_digits[stone_digits.len() / 2..]
            .iter()
            .collect::<String>()
            .parse::<u128>()
            .unwrap();

        let left_count = count_stones_recursive(stone_sizes, left, iterations - 1);
        let right_count = count_stones_recursive(stone_sizes, right, iterations - 1);

        let count = left_count + right_count;
        stone_sizes.insert((stone, iterations), count);
        return count;
    }

    let count = count_stones_recursive(stone_sizes, stone * 2024, iterations - 1);
    stone_sizes.insert((stone, iterations), count);
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_stones_small() {
        assert_eq!(count_stones("resources/2024/day11/test_input.txt", 6), 22);
    }

    #[test]
    fn test_count_stones() {
        assert_eq!(
            count_stones("resources/2024/day11/test_input.txt", 25),
            55312
        );
    }
}
