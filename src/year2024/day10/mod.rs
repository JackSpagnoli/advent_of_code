use std::collections::HashSet;

pub mod task1 {
    pub fn ans() -> u128 {
        super::trailhead_sum("resources/2024/day10/input.txt").1
    }
}

pub mod task2 {
    pub fn ans() -> u128 {
        super::trailhead_sum("resources/2024/day10/input.txt").0
    }
}

type Map = Vec<Vec<u8>>;

fn trailhead_sum(file: &str) -> (u128, u128) {
    let content = std::fs::read_to_string(file).unwrap();

    let map: Map = content
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    map.iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, &cell)| (x, y, cell)))
        .filter(|(_, _, cell)| *cell == 0)
        .map(|(x, y, _)| (x, y))
        .map(|pos| trails(&map, pos))
        .fold((0, 0), |acc, counts| {
            (acc.0 + counts.0, acc.1 + counts.1.len() as u128)
        })
}

fn trails(map: &Map, pos: (usize, usize)) -> (u128, HashSet<(usize, usize)>) {
    let trail_height = map[pos.1][pos.0];

    if trail_height == 9 {
        return (1, HashSet::from_iter(vec![pos]));
    }

    let map_height = map.len();
    let map_width = map[0].len();

    [
        (pos.0 as isize + 1, pos.1 as isize),
        (pos.0 as isize - 1, pos.1 as isize),
        (pos.0 as isize, pos.1 as isize + 1),
        (pos.0 as isize, pos.1 as isize - 1),
    ]
    .into_iter()
    .filter(|(x, y)| *x >= 0 && *x < map_width as isize && *y >= 0 && *y < map_height as isize)
    .map(|(x, y)| (x as usize, y as usize))
    .filter(|(x, y)| map[*y][*x] == trail_height + 1)
    .map(|pos| trails(map, pos))
    .fold(
        (0, HashSet::new()),
        |(acc_count, mut acc_set), (count, mut set)| {
            acc_set.extend(set.drain());
            (acc_count + count, acc_set)
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trailhead_sum() {
        assert_eq!(trailhead_sum("resources/2024/day10/test_input.txt").1, 36);
    }
    #[test]
    fn test_trailheads() {
        assert_eq!(trailhead_sum("resources/2024/day10/test_input.txt").0, 81);
    }
}
