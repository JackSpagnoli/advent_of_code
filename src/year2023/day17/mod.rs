use std::{
    cmp::{max, min},
    collections::HashMap,
};

pub mod task1 {
    use super::shortest_path;

    pub fn ans() -> u128 {
        shortest_path("resources/2023/day17/input")
    }
}

pub mod task2 {
    // use super::contained_area;

    pub fn ans() -> u128 {
        // contained_area("resources/2023/day10/input")
        1
    }
}

fn shortest_path(file: &str) -> u128 {
    let map = parse_file(file);
    let start_pos = (0, 0);
    let end_pos = (map.len() - 1, map[0].len() - 1);
    diestrkas(map, start_pos, end_pos).1
}

#[derive(PartialEq, Debug, Eq, Hash, Copy, Clone)]
enum Axis {
    Vertical,
    Horizontal,
}
type PositionDirection = (Axis, Coordinate);
fn diestrkas(map: Map, start_pos: Coordinate, end_pos: Coordinate) -> (Vec<Coordinate>, u128) {
    // This was painful to write. A* algorithm modified to work with direction and position.

    let mut open_set = vec![(Axis::Vertical, start_pos), (Axis::Horizontal, start_pos)];
    let mut came_from: HashMap<PositionDirection, PositionDirection> = HashMap::new();

    let mut current_best_scores: HashMap<PositionDirection, u128> = HashMap::new();
    current_best_scores.insert((Axis::Vertical, start_pos), 0);
    current_best_scores.insert((Axis::Horizontal, start_pos), 0);

    let mut weighted_scores: HashMap<PositionDirection, u128> = HashMap::new();
    weighted_scores.insert((Axis::Vertical, start_pos), l1_metric(start_pos, end_pos));
    weighted_scores.insert((Axis::Horizontal, start_pos), l1_metric(start_pos, end_pos));

    while let Some((current_axis, current_pos)) = open_set.pop() {
        if current_pos == end_pos {
            break;
        }

        println!("{}", open_set.len());
        let neighbours = get_neighbours(&map, current_pos, &current_axis, end_pos);
        let current_best_score = *current_best_scores
            .get(&(current_axis, current_pos))
            .unwrap();

        for neighbour in neighbours {
            let neighbour_axis = get_travel_axis(current_pos, neighbour);
            let neighbour_hash = (neighbour_axis, neighbour);
            let edge_score = get_path_score(&map, current_pos, neighbour);
            let tentative_score = current_best_score + edge_score;

            let neighbour_score_improvement =
                tentative_score < *current_best_scores.get(&neighbour_hash).unwrap_or(&u128::MAX);
            if neighbour_score_improvement {
                came_from.insert(neighbour_hash, (current_axis, current_pos));
                current_best_scores.insert(neighbour_hash, tentative_score);
                let neighbour_weighted_score = tentative_score + l1_metric(neighbour, end_pos);
                weighted_scores.insert(neighbour_hash, neighbour_weighted_score);
                // if !open_set.contains(&neighbour_hash) {
                    open_set.push(neighbour_hash);
                // }
            }
        }

        open_set.sort_by(|a, b| {
            let a_score = weighted_scores.get(a).unwrap_or(&u128::MAX);
            let b_score = weighted_scores.get(b).unwrap_or(&u128::MAX);
            a_score.cmp(b_score)
        });
    }
    let end_paths = vec![(Axis::Vertical, end_pos), (Axis::Horizontal, end_pos)];
    end_paths
        .iter()
        .min_by_key(|end_pos| current_best_scores.get(&end_pos).unwrap_or(&u128::MAX))
        .map(|end_pos| {
            (
                reconstruct_path(came_from, *end_pos),
                *current_best_scores.get(&end_pos).unwrap(),
            )
        })
        .unwrap()
}

fn get_path_score(map: &Map, current: Coordinate, neighbour: Coordinate) -> u128 {
    let y_range = min(current.0, neighbour.0)..=max(current.0, neighbour.0);
    let x_range = min(current.1, neighbour.1)..=max(current.1, neighbour.1);

    y_range
        .into_iter()
        .flat_map(|y| x_range.clone().into_iter().map(move |x| map[y][x] as u128))
        .sum::<u128>()
        - (map[current.0][current.1] as u128)
}

fn get_travel_axis(current: Coordinate, neighbour: Coordinate) -> Axis {
    if current.1 == neighbour.1 {
        Axis::Vertical
    } else {
        Axis::Horizontal
    }
}

fn get_neighbours(
    map: &Map,
    current: Coordinate,
    axis: &Axis,
    endpos: Coordinate,
) -> Vec<Coordinate> {
    let height = map.len() as isize;
    let width = map[0].len() as isize;
    let mut neighbours = vec![];

    let current = (current.0 as isize, current.1 as isize);

    if (axis != &Axis::Vertical) || current == (0, 0) {
        neighbours.push((current.0 - 1, current.1));
        neighbours.push((current.0 - 2, current.1));
        neighbours.push((current.0 - 3, current.1));
        neighbours.push((current.0 + 1, current.1));
        neighbours.push((current.0 + 2, current.1));
        neighbours.push((current.0 + 3, current.1));
    }
    if axis != &Axis::Horizontal || current == (0, 0) {
        neighbours.push((current.0, current.1 - 1));
        neighbours.push((current.0, current.1 - 2));
        neighbours.push((current.0, current.1 - 3));
        neighbours.push((current.0, current.1 + 1));
        neighbours.push((current.0, current.1 + 2));
        neighbours.push((current.0, current.1 + 3));
    }

    if neighbours.contains(&(endpos.0 as isize, endpos.1 as isize)) {
        return vec![endpos];
    }

    neighbours
        .into_iter()
        .filter(|coordinate| {
            coordinate.0 < height && coordinate.0 >= 0 && coordinate.1 < width && coordinate.1 >= 0
        })
        .map(|(a, b)| (a as usize, b as usize))
        .collect()
}

fn reconstruct_path(
    came_from: HashMap<PositionDirection, PositionDirection>,
    current: PositionDirection,
) -> Vec<Coordinate> {
    let mut path = vec![current.1];
    let mut current = current;

    while let Some(next) = came_from.get(&current) {
        path.push(next.1);
        current = *next;
    }

    path
}

fn l1_metric(a: Coordinate, b: Coordinate) -> u128 {
    ((a.0 as isize - b.0 as isize).abs() + (a.1 as isize - b.1 as isize).abs()) as u128
}

type Coordinate = (usize, usize);
type Weight = u8;
type Map = Vec<Vec<Weight>>;
fn parse_file(file: &str) -> Map {
    let contents = std::fs::read_to_string(file).unwrap();

    contents.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Vec<Weight> {
    line.chars()
        .map(|c| c.to_digit(10).unwrap() as Weight)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_neighbours() {
        let map = parse_file("resources/2023/day17/test_input");
        let current = (0, 4);
        let direction = Axis::Vertical;
        let neighbours = get_neighbours(&map, current, &direction, (10, 10));
        assert_eq!(
            neighbours,
            vec![(0, 3), (0, 2), (0, 1), (0, 5), (0, 6), (0, 7)]
        );
    }

    #[test]
    fn test_a_star() {
        let map = parse_file("resources/2023/day17/test_input");
        let start_pos = (0, 0);
        let end_pos = (12, 12);
        let (path, score) = diestrkas(map, start_pos, end_pos);

        let mut printable_grid = vec![vec!['.'; 13]; 13];
        println!("{:?}", path);
        for coordinate in path {
            printable_grid[coordinate.0][coordinate.1] = '#';
        }

        for row in printable_grid {
            for c in row {
                print!("{}", c);
            }
            println!("");
        }

        assert_eq!(score, 102);
    }
}
