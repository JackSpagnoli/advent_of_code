use std::fs;

use itertools::Itertools;

pub mod task1 {
    use super::pair_distances;

    pub fn ans() -> u128 {
        pair_distances("resources/2023/day11/input", 2)
    }
}

pub mod task2 {
    use super::pair_distances;

    pub fn ans() -> u128 {
        pair_distances("resources/2023/day11/input", 1_000_000)
    }
}

fn pair_distances(file:&str, distance_factor: usize) -> u128{
    parse_file(file, distance_factor).sum_distance_pairs() as u128
}

type Coordinate = (usize, usize);
struct Galaxies {
    galaxies: Vec<Coordinate>,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
    distance_factor: usize,
}

impl Galaxies {
    fn distance(&self, a: Coordinate, b: Coordinate) -> usize {
        let y_diff = if a.0 < b.0 { a.0..b.0 } else { b.0..a.0 };
        let x_diff = if a.1 < b.1 { a.1..b.1 } else { b.1..a.1 };

        let empty_cols = self
            .empty_cols
            .iter()
            .filter(|c| x_diff.contains(c))
            .count();
        let empty_rows = self
            .empty_rows
            .iter()
            .filter(|c| y_diff.contains(c))
            .count();

        x_diff.count() + y_diff.count() + (empty_cols + empty_rows) * (self.distance_factor-1)
    }

    fn sum_distance_pairs(&self) -> usize {
        self.galaxies
            .iter()
            .combinations(2)
            .map(|c| {
                let a = c[0];
                let b = c[1];
                self.distance(*a, *b)
            })
            .sum()
    }
}

fn parse_file(file: &str, distance_factor: usize) -> Galaxies {
    let contents = fs::read_to_string(file).expect("Error reading the file");

    let mut galaxies: Vec<Coordinate> = Vec::new();
    let mut empty_rows: Vec<usize> = Vec::new();
    let mut empty_cols: Vec<usize> = Vec::new();

    contents.lines().enumerate().for_each(|(row_index, line)| {
        let empty = line.clone().chars().all(|c| c == '.');
        if empty {
            empty_rows.push(row_index);
            return;
        }
        line.chars()
            .enumerate()
            .filter(|(_, c)| *c == '#')
            .for_each(|(col_index, _)| {
                galaxies.push((row_index, col_index));
            });
    });

    let line_length = contents.lines().next().unwrap().len();
    (0..line_length).for_each(|col_index| {
        let empty = contents
            .lines()
            .clone()
            .map(|line| line.chars().nth(col_index).unwrap())
            .all(|c| c == '.');
        if empty {
            empty_cols.push(col_index);
        }
    });

    Galaxies {
        galaxies,
        empty_rows,
        empty_cols,
        distance_factor,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_file() {
        let galaxies = parse_file("resources/2023/day11/test_input", 1);
        let expected_galaxies = vec![
            (0, 3),
            (1, 7),
            (2, 0),
            (4, 6),
            (5, 1),
            (6, 9),
            (8, 7),
            (9, 0),
            (9, 4),
        ];
        let expected_empty_rows = vec![3, 7];
        let expected_empty_cols = vec![2, 5, 8];

        assert_eq!(galaxies.galaxies, expected_galaxies);
        assert_eq!(galaxies.empty_rows, expected_empty_rows);
        assert_eq!(galaxies.empty_cols, expected_empty_cols);
    }

    #[test]
    fn test_distance() {
        let galaxies = parse_file("resources/2023/day11/test_input", 2);
        let a = galaxies.galaxies[4];
        let b = galaxies.galaxies[8];
        let expected_distance = 9;

        assert_eq!(galaxies.distance(a, b), expected_distance);
    }

    #[test]
    fn test_distance2() {
        let galaxies = parse_file("resources/2023/day11/test_input", 2);
        let a = galaxies.galaxies[0];
        let b = galaxies.galaxies[6];
        let expected_distance = 15;

        assert_eq!(galaxies.distance(a, b), expected_distance);
    }
}
