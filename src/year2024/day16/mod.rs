use std::{collections::HashMap, fs};

pub mod task1 {
    use super::lowest_scoring_path;

    pub fn ans() -> u128 {
        lowest_scoring_path("resources/2024/day16/input.txt")
    }
}

pub mod task2 {
    pub fn ans() -> u128 {
        0
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn lowest_scoring_path(file: &str) -> u128 {
    let content = fs::read_to_string(file).unwrap();

    let map: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();

    let mut start = (0, 0);
    let mut end = (0, 0);

    map.iter()
        .enumerate()
        .flat_map(|(y, row)| row.into_iter().enumerate().map(move |(x, c)| (x, y, c)))
        .for_each(|(x, y, c)| {
            if c == &'S' {
                start = (x as isize, y as isize);
            } else if c == &'E' {
                end = (x as isize, y as isize);
            }
        });

    let mut frontier = vec![(start, Direction::Right)];

    let mut scores = HashMap::new();
    scores.insert((start, Direction::Right), 0);

    while let Some((pos, direction)) = frontier.pop() {
        let mut score = *scores.get(&(pos, direction)).unwrap();

        let rotations = adjacent_rotations(&direction);
        for rotation in rotations.into_iter() {
            if !scores.contains_key(&(pos, rotation)) {
                scores.insert((pos, rotation), score + 1000);
                frontier.push((pos, rotation));
            }
        }

        let (mut x, mut y) = pos;

        let step = match direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        let neighbours = |(x, y): (isize, isize)| match direction {
            Direction::Up | Direction::Down => [
                ((x - 1, y), Direction::Left),
                ((x + 1, y), Direction::Right),
            ],
            Direction::Left | Direction::Right => {
                [((x, y - 1), Direction::Up), ((x, y + 1), Direction::Down)]
            }
        };

        loop {
            x += step.0;
            y += step.1;
            let pos = (x, y);
            score += 1;

            if &map[y as usize][x as usize] == &'#' {
                break;
            }

            let neighbours = neighbours(pos);
            let valid_neighbours = neighbours
                .into_iter()
                .filter(|((x, y), _)| map[*y as usize][*x as usize] != '#')
                .collect::<Vec<_>>();

            if valid_neighbours.is_empty() && (x, y) != end {
                continue;
            }

            if let Some(&prev_score) = scores.get(&(pos, direction)) {
                if score < prev_score {
                    scores.insert((pos, direction), score);
                } else {
                    continue;
                }
            } else {
                scores.insert((pos, direction), score);
            }

            for (_, new_dir) in valid_neighbours.into_iter() {
                if let Some(&prev_score) = scores.get(&(pos, new_dir)) {
                    if score < prev_score {
                        scores.insert((pos, new_dir), score + 1000);
                        frontier.push((pos, new_dir));
                    }
                } else {
                    scores.insert((pos, new_dir), score + 1000);
                    frontier.push((pos, new_dir));
                }
            }
        }
    }

    scores
        .iter()
        .filter(|((pos, _), _)| pos == &end)
        .map(|(_, score)| *score as u128)
        .min()
        .unwrap()
}

fn adjacent_rotations(direction: &Direction) -> [Direction; 2] {
    match direction {
        Direction::Up | Direction::Down => [Direction::Left, Direction::Right],
        Direction::Left | Direction::Right => [Direction::Down, Direction::Up],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lowest_scoring_path() {
        assert_eq!(
            lowest_scoring_path("resources/2024/day16/test_input_2.txt"),
            7036
        );

        assert_eq!(
            lowest_scoring_path("resources/2024/day16/test_input.txt"),
            11048
        )
    }
}
