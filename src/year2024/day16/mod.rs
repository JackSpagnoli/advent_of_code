use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub mod task1 {
    use super::lowest_scoring_path;

    pub fn ans() -> u128 {
        lowest_scoring_path("resources/2024/day16/input.txt")
    }
}

pub mod task2 {
    use super::tiles_on_shortest_paths;

    pub fn ans() -> u128 {
        tiles_on_shortest_paths("resources/2024/day16/input.txt")
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

    solve_scores(map, start, end)
        .into_iter()
        .filter(|((pos, _), _)| pos == &end)
        .map(|(_, (score, _))| score as u128)
        .min()
        .unwrap()
}

fn tiles_on_shortest_paths(file: &str) -> u128 {
    // Solve the maze, then backtrack through the graph of shortest paths,
    // collecting the tiles that are part of the shortest paths and count

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

    let scores = solve_scores(map, start, end);

    let shortest_path = scores
        .iter()
        .filter(|((pos, _), _)| pos == &end)
        .map(|(_, (score, _))| score)
        .min()
        .unwrap();

    let end_backtracks = scores
        .iter()
        .filter(|((pos, _), _)| pos == &end)
        .filter(|(_, (score, _))| score == shortest_path)
        .map(|(_, (_, backtrack))| backtrack)
        .flatten()
        .collect::<HashSet<_>>();

    // Pairs of tiles and backtracks which lead to them
    let mut frontier = end_backtracks
        .into_iter()
        .map(|pos| (end, pos))
        .collect::<HashSet<_>>();

    let mut tiles = HashSet::new();
    tiles.insert(end);

    loop {
        let current = match frontier.iter().next() {
            Some(pos) => *pos,
            None => break,
        };
        frontier.remove(&current);

        let (end_pos, (prev_pos, direction)) = current;

        let step = match direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        let (mut x, mut y) = prev_pos;
        while (x, y) != end_pos {
            tiles.insert((x, y));
            x += step.0;
            y += step.1;
        }

        tiles.insert(end_pos);

        let (_, previous) = scores.get(&(*prev_pos, *direction)).unwrap();
        previous
            .into_iter()
            .map(|pos| (*prev_pos, pos))
            .for_each(|pos| {
                frontier.insert(pos);
            });
    }

    tiles.len() as u128
}

fn solve_scores(
    map: Vec<Vec<char>>,
    start: (isize, isize),
    end: (isize, isize),
) -> HashMap<((isize, isize), Direction), (i32, HashSet<((isize, isize), Direction)>)> {
    let mut frontier = vec![(start, Direction::Right)];

    // Stores position, direction, and the previous turns that reach the position with the lowest score
    let mut scores: HashMap<
        ((isize, isize), Direction),
        (i32, HashSet<((isize, isize), Direction)>),
    > = HashMap::new();
    scores.insert((start, Direction::Right), (0, HashSet::new()));

    while let Some(current) = frontier.pop() {
        let (pos, direction) = current;
        let (mut score, _) = *scores.get(&(pos, direction)).unwrap();

        let rotations = adjacent_rotations(&direction);
        for new_dir in rotations.into_iter() {
            let rotation = (pos, new_dir);
            if let Some((prev_score, prev_backtrack)) = scores.get_mut(&rotation) {
                if &(score + 1000) < prev_score {
                    *prev_score = score + 1000;
                    *prev_backtrack = HashSet::from([current]);
                    frontier.push(rotation);
                } else if &(score + 1000) == prev_score {
                    prev_backtrack.insert(current);
                }
            } else {
                scores.insert(rotation, (score + 1000, HashSet::from([current])));
                frontier.push(rotation);
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
            let new_pos = (x, y);
            score += 1;

            if &map[y as usize][x as usize] == &'#' {
                break;
            }

            let neighbours = neighbours(new_pos);
            let valid_neighbours = neighbours
                .into_iter()
                .filter(|((x, y), _)| map[*y as usize][*x as usize] != '#')
                .collect::<Vec<_>>();

            if !(!valid_neighbours.is_empty() || new_pos == end) {
                continue;
            }

            // Add straight path
            if let Some((prev_score, prev_backtrack)) = scores.get_mut(&(new_pos, direction)) {
                // If score is lower, replace it
                if &score < prev_score {
                    *prev_score = score;
                    *prev_backtrack = HashSet::from([current]);
                    frontier.push((new_pos, direction));
                } else if &score == prev_score {
                    // If score matches, add last turn to the set
                    prev_backtrack.insert(current);
                }
                // If score is higher, ignore it
            } else {
                scores.insert((new_pos, direction), (score, HashSet::from([current])));
                frontier.push((new_pos, direction));
            }

            break;
        }
    }

    scores
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

    #[test]
    fn test_tiles_on_shortest_paths() {
        assert_eq!(
            tiles_on_shortest_paths("resources/2024/day16/test_input_2.txt"),
            45
        );

        assert_eq!(
            tiles_on_shortest_paths("resources/2024/day16/test_input.txt"),
            64
        )
    }
}
