use std::collections::{HashMap, VecDeque};

pub mod task1 {
    use super::number_of_tiles;

    pub fn ans() -> u128 {
        number_of_tiles("resources/2023/day21/input", 64)
    }
}

pub mod task2 {
    use super::number_of_tiles_repeating;

    pub fn ans() -> u128 {
        number_of_tiles_repeating("resources/2023/day21/input", 26501365)
    }
}

fn number_of_tiles_repeating(file: &str, total_steps: u128) -> u128 {
    // Gonna be honest I had to read this to get this working.
    // https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21
    // Even then I had a nightmare getting my implementation to behave on the pretty lacking test data.

    // The TLDR is that since our number of steps is 26501365 = 65 + (202300 * 131),
    // and 131 is the width of the grid we're given, the question becomes more about
    // how grids cyclically repeat. (The middle row/column of the grid is empty which forces periodicity)

    // In the end we can look at a single grid, run it for 131 steps for our 'odd' number,
    // and 132 steps for our 'even' number.
    // The large diamond formed after the total number of steps will be n full grids wide.
    // By some weird counting it will contain (n+1)^2 odd grids and n^2 even grids.
    // The edge grids have a number of steps equal to n half-filled even grids,
    // minus (n-1) half-filled odd grids.

    // This uses breath-first search to find the distance from the centre point to each tile
    // and filters based on parity.
    let (_, tiles) = parse_file(file);

    let grid_width = tiles.len() as u128;
    let grid_radius = (grid_width - 1) / 2;

    let centre_point = (grid_radius as isize, grid_radius as isize);
    let coordinate_distance = bfs(&tiles, centre_point);

    let even_tiles = coordinate_distance
        .values()
        .filter(|dist| *dist % 2 == 0)
        .count() as u128;
    let even_corners = coordinate_distance
        .values()
        .filter(|dist| *dist % 2 == 0 && *dist > &grid_radius)
        .count() as u128;

    let odd_tiles = coordinate_distance
        .values()
        .filter(|dist| *dist % 2 == 1)
        .count() as u128;
    let odd_corners = coordinate_distance
        .values()
        .filter(|dist| *dist % 2 == 1 && *dist > &grid_radius)
        .count() as u128;

    let n = (total_steps - grid_radius) / grid_width;

    let n_plus_one = n + 1;
    let odd_factor = n_plus_one * n_plus_one;
    let even_factor = n * n;
    (odd_tiles * odd_factor) as u128 + (even_tiles * even_factor) as u128
        - (odd_corners * n_plus_one) as u128
        + (even_corners * n) as u128
}

fn number_of_tiles(file: &str, steps: u128) -> u128 {
    let (start_pos, tiles) = parse_file(file);

    bfs(&tiles, start_pos).values().filter(|dist| **dist <= steps).filter(|dist| **dist % 2 == steps % 2).count() as u128
}

fn bfs(tiles: &Map, start_pos: Coordinate) -> HashMap<Coordinate, u128> {
    let mut frontier = VecDeque::new();
    let mut visited = HashMap::new();

    frontier.push_back((start_pos, 0));

    while let Some((coord, dist)) = frontier.pop_front() {
        if visited.contains_key(&coord) {
            continue;
        }

        visited.insert(coord, dist);

        let neighbour_coords = [
            (coord.0 - 1, coord.1),
            (coord.0 + 1, coord.1),
            (coord.0, coord.1 - 1),
            (coord.0, coord.1 + 1),
        ];

        for neighbour_coord in neighbour_coords {
            if neighbour_coord.0 < 0
                || neighbour_coord.1 < 0
                || neighbour_coord.1 >= tiles[0].len() as isize
                || neighbour_coord.0 >= tiles.len() as isize
            {
                continue;
            }
            if !visited.contains_key(&neighbour_coord)
                && tiles[neighbour_coord.0 as usize][neighbour_coord.1 as usize] == Tile::Plot
            {
                frontier.push_back((neighbour_coord, dist + 1));
            }
        }
    }

    visited
}

#[derive(Debug, PartialEq)]
enum Tile {
    Plot,
    Rock,
}
type Map = Vec<Vec<Tile>>;
type Coordinate = (isize, isize);
fn parse_file(file: &str) -> (Coordinate, Map) {
    let contents = std::fs::read_to_string(file).unwrap();

    let mut start_pos = (0, 0);

    let tiles = contents
        .lines()
        .enumerate()
        .map(|(j, line)| {
            line.chars()
                .enumerate()
                .map(|(i, c)| match c {
                    '#' => Tile::Rock,
                    '.' => Tile::Plot,
                    'S' => {
                        start_pos = (j as isize, i as isize);
                        Tile::Plot
                    }
                    _ => panic!("Invalid character in input file"),
                })
                .collect()
        })
        .collect();

    (start_pos, tiles)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_tiles() {
        assert_eq!(number_of_tiles("resources/2023/day21/test_input", 0), 1);
        assert_eq!(number_of_tiles("resources/2023/day21/test_input", 1), 2);
        assert_eq!(number_of_tiles("resources/2023/day21/test_input", 6), 16);
    }
}

