use std::{cmp::max, fs};

pub mod task1 {
    use super::sum_points;

    pub fn ans() -> u128 {
        sum_points("resources/2023/day04/input")
    }
}

pub mod task2 {
    use super::sum_copies;

    pub fn ans() -> u128 {
        sum_copies("resources/2023/day04/input")
    }
}

fn sum_points(file: &str) -> u128 {
    let file = fs::read_to_string(file).expect("Could not read file");

    file.lines()
        .map(parse_line)
        .map(calc_matching_numbers_count)
        .map(get_line_score)
        .sum::<usize>() as u128
}

fn sum_copies(file: &str) -> u128 {
    let file = fs::read_to_string(file).expect("Could not read file");

    let mut lines = file
        .lines()
        .map(parse_line)
        .map(calc_matching_numbers_count)
        .collect::<Vec<Line>>();

    for line_number in 0..lines.len() {
        let current_line_wins = lines[line_number].matching_numbers_count;
        let current_line_copies = lines[line_number].copies;

        match current_line_wins {
            Some(0) | None => continue,
            Some(line_wins) => {
                lines[line_number + 1..=line_number + line_wins]
                    .iter_mut()
                    .for_each(|line| line.copies += current_line_copies);
            }
        }
    }

    lines.iter().map(|line| line.copies).sum::<usize>() as u128
}

#[derive(Default, PartialEq, Debug)]
struct Line {
    winning_numbers: Vec<usize>,
    card_numbers: Vec<usize>,
    matching_numbers_count: Option<usize>,
    copies: usize,
}

fn calc_matching_numbers_count(line: Line) -> Line {
    let matching_numbers_count = Some(
        line.winning_numbers
            .iter()
            .filter(|number| line.card_numbers.contains(number))
            .count(),
    );
    Line {
        matching_numbers_count,
        ..line
    }
}

fn get_line_score(line: Line) -> usize {
    match line.matching_numbers_count {
        Some(0) | None => 0,
        Some(matching_numbers_count) => 2usize.pow(max(matching_numbers_count - 1, 0) as u32),
    }
}

fn parse_line(line: &str) -> Line {
    let mut numbers = line.split(':').nth(1).unwrap().split('|');
    let winning_numbers = parse_numbers(numbers.next().unwrap());
    let card_numbers = parse_numbers(numbers.next().unwrap());
    let matching_numbers_count = None;
    let copies = 1;
    Line {
        winning_numbers,
        card_numbers,
        matching_numbers_count,
        copies,
    }
}

fn parse_numbers(numbers: &str) -> Vec<usize> {
    numbers
        .split(' ')
        .skip(1)
        .filter(|str| !str.is_empty())
        .map(|number| number.parse::<usize>().unwrap())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_line() {
        let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let parsed_line = parse_line(line);

        assert_eq!(parsed_line.winning_numbers, vec![41, 48, 83, 86, 17]);
        assert_eq!(parsed_line.card_numbers, vec![83, 86, 6, 31, 17, 9, 48, 53]);
    }

    #[test]
    fn test_parse_line_singles() {
        let line = "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1";
        let parsed_line = parse_line(line);

        assert_eq!(parsed_line.winning_numbers, vec![1, 21, 53, 59, 44]);
        assert_eq!(
            parsed_line.card_numbers,
            vec![69, 82, 63, 72, 16, 21, 14, 1]
        );
    }

    #[test]
    fn test_matching_numbers_count() {
        let lines = [
            ("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 4),
            ("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 1),
            ("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 0),
        ];

        lines.iter().for_each(|(line, expected)| {
            let parsed_line = calc_matching_numbers_count(parse_line(line));
            assert_eq!(parsed_line.matching_numbers_count.unwrap(), *expected);
        });
    }

    #[test]
    fn test_line_points() {
        let lines = [
            ("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 8),
            ("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 1),
            ("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 0),
        ];

        lines.iter().for_each(|(line, expected)| {
            let score = get_line_score(calc_matching_numbers_count(parse_line(line)));
            assert_eq!(score, *expected);
        });
    }

    #[test]
    fn test_sum_points() {
        assert_eq!(sum_points("resources/2023/day04/test_input"), 13);
    }

    #[test]
    fn test_sum_copies() {
        assert_eq!(sum_copies("resources/2023/day04/test_input"), 30);
    }
}
