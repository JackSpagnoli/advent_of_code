use std::{cmp::Ordering, ops::RangeInclusive};

pub mod task1 {
    use super::margin_of_error;

    pub fn ans() -> u128 {
        margin_of_error("resources/2023/day06/input")
    }
}

pub mod task2 {
    use super::margin_of_error_part_2;

    pub fn ans() -> u128 {
        margin_of_error_part_2("resources/2023/day06/input")
    }
}

fn margin_of_error(file: &str) -> u128 {
    let input = std::fs::read_to_string(file).expect("Could not read file");
    let mut lines = input.lines();
    let times_str = lines.next().expect("No times");
    let distances_str = lines.next().expect("No distances");

    let regex = regex::Regex::new(r"\b(\d+)\b").unwrap();
    let times: Vec<u128> = regex
        .captures_iter(times_str)
        .map(|cap| cap[0].parse::<u128>().unwrap())
        .collect::<Vec<_>>();
    let distances: Vec<u128> = regex
        .captures_iter(distances_str)
        .map(|cap| cap[0].parse::<u128>().unwrap())
        .collect::<Vec<_>>();

    get_margin_of_error(times, distances)
}

fn margin_of_error_part_2(file: &str) -> u128 {
    let input = std::fs::read_to_string(file).expect("Could not read file");
    let mut lines = input.lines();
    let times_str = lines.next().expect("No times");
    let distances_str = lines.next().expect("No distances");

    let regex = regex::Regex::new(r"\b(\d+)\b").unwrap();
    let time: u128 = regex
        .captures_iter(times_str)
        .fold("".to_string(), |acc, cap| {
            format!("{}{}", acc, &cap[0]).to_string()
        })
        .parse::<u128>()
        .unwrap();
    let distance: u128 = regex
        .captures_iter(distances_str)
        .fold("".to_string(), |acc, cap| {
            format!("{}{}", acc, &cap[0]).to_string()
        })
        .parse::<u128>()
        .unwrap();

    ways_of_winning((time, distance))
}

fn get_margin_of_error(times: Vec<u128>, distances: Vec<u128>) -> u128 {
    let ways_of_winning = times
        .into_iter()
        .zip(distances)
        .map(ways_of_winning)
        .collect::<Vec<_>>();

    ways_of_winning.into_iter().product()
}

fn ways_of_winning((time, distance): (u128, u128)) -> u128 {
    let time_range = 0..=time;

    let predicate: &Predicate =
        &move |charging_time| find_distance(charging_time, time).cmp(&distance);
    let lower_limit = match binary_search(time_range, predicate) {
        Ok(x) => x + 1,
        Err(x) => x,
    };

    time - 2 * (lower_limit) + 1
}

fn find_distance(charging_time: u128, time: u128) -> u128 {
    charging_time * (time - charging_time)
}

type Predicate = dyn Fn(u128) -> Ordering;
fn binary_search(range: RangeInclusive<u128>, predicate: &Predicate) -> Result<u128, u128> {
    let mut range = range;
    while range.start() != range.end() {
        let mid = (range.start() + range.end()) / 2;
        match predicate(mid) {
            Ordering::Less => range = mid + 1..=*range.end(),
            Ordering::Greater => range = *range.start()..=mid,
            Ordering::Equal => return Ok(mid),
        }
    }
    Err(*range.start())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_distance() {
        let time = 7;
        let charging_times = [0, 1, 2, 3, 4, 5, 6, 7];

        let expected_distances = vec![0, 6, 10, 12, 12, 10, 6, 0];

        let distances = charging_times
            .into_iter()
            .map(|charging_time| find_distance(charging_time, time))
            .collect::<Vec<_>>();

        assert_eq!(distances, expected_distances);
    }

    #[test]
    fn test_ways_of_winning() {
        let time = 7;
        let distance = 9;

        let expected_ways_of_winning = 4;

        let ways_of_winning = ways_of_winning((time, distance));

        assert_eq!(ways_of_winning, expected_ways_of_winning);
    }

    #[test]
    fn test_margin_of_error() {
        let file = "resources/2023/day06/test_input";

        let expected_margin_of_error = 288;

        let margin_of_error = margin_of_error(file);

        assert_eq!(margin_of_error, expected_margin_of_error);
    }

    #[test]
    fn test_margin_of_error_part_2() {
        let file = "resources/2023/day06/test_input";

        let expected_margin_of_error = 71503;

        let margin_of_error = margin_of_error_part_2(file);

        assert_eq!(margin_of_error, expected_margin_of_error);
    }
}
