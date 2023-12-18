use std::{
    cmp::{max, min},
    collections::{BinaryHeap, HashMap},
};

pub mod task1 {
    use super::shortest_path;

    pub fn ans() -> u128 {
        shortest_path("resources/2023/day17/input")
    }
}

pub mod task2 {
    use super::shortest_path_ultra;

    pub fn ans() -> u128 {
        shortest_path_ultra("resources/2023/day17/input")
    }
}

fn shortest_path(file: &str) -> u128 {
    let map = parse_file(file);
    let start_pos = (0, 0);
    let end_pos = (map.len() as isize - 1, map[0].len() as isize - 1);
    dijkstra(&map, start_pos, end_pos, 1, 3)
}

fn shortest_path_ultra(file: &str) -> u128 {
    let map = parse_file(file);
    let start_pos = (0, 0);
    let end_pos = (map.len() as isize - 1, map[0].len() as isize - 1);
    dijkstra(&map, start_pos, end_pos, 4, 10)
}

#[derive(PartialEq, Eq)]
struct HeapEntry {
    directed_position: DirectedPositon,
    cost: isize,
}
impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reversed to make it a min heap
        self.cost.cmp(&other.cost).reverse()
    }
}

fn dijkstra(
    map: &Map,
    start_pos: Coordinate,
    end_pos: Coordinate,
    min_move: isize,
    max_move: isize,
) -> u128 {
    // Implementation of dijkstra's that only allows between min_move and max_move steps in one direction before needing to turn

    // Define sets and maps
    let mut open_set: BinaryHeap<HeapEntry> = std::collections::BinaryHeap::new();
    let mut came_from: HashMap<DirectedPositon, DirectedPositon> = HashMap::new();
    let mut cheapest_paths: HashMap<DirectedPositon, isize> = HashMap::new();

    // Add inital values to sets and maps
    open_set.push(HeapEntry {
        directed_position: (start_pos, 0),
        cost: 0,
    });
    open_set.push(HeapEntry {
        directed_position: (start_pos, 1),
        cost: 0,
    });

    cheapest_paths.insert((start_pos, 0), 0);
    cheapest_paths.insert((start_pos, 1), 0);

    let mut current_best_path_cost = isize::MAX;

    // Loop until open_set is empty
    while let Some(HeapEntry {
        directed_position: (pos, axis),
        cost,
    }) = open_set.pop()
    {
        if cost > current_best_path_cost {
            continue;
        }

        let neighbours = get_neighbours(map, pos, axis, min_move, max_move);
        let neighbour_axis = 1 - axis;
        for neighbour_pos in neighbours {
            let neighbour_path_cost = get_path_cost(map, pos, neighbour_pos);
            let tenative_neighbour_cost = cost + neighbour_path_cost;

            let neighbour_hash = (neighbour_pos, neighbour_axis);
            if tenative_neighbour_cost < *cheapest_paths.get(&neighbour_hash).unwrap_or(&isize::MAX)
            {
                if neighbour_pos == end_pos {
                    current_best_path_cost = tenative_neighbour_cost;
                }

                came_from.insert(neighbour_hash, (pos, axis));
                cheapest_paths.insert(neighbour_hash, tenative_neighbour_cost);

                open_set.retain(|x| x.directed_position != neighbour_hash);
                open_set.push(HeapEntry {
                    directed_position: neighbour_hash,
                    cost: tenative_neighbour_cost,
                });
            }
        }
    }

    let end_pos_cost_0 = cheapest_paths.get(&(end_pos, 0)).unwrap();
    let end_pos_cost_1 = cheapest_paths.get(&(end_pos, 1)).unwrap();

    if end_pos_cost_0 < end_pos_cost_1 {
        #[cfg(test)]
        {
            let path = backtrace(&came_from, (end_pos, 0));
            println!("Path: {:?}", path);
        }
        *end_pos_cost_0 as u128
    } else {
        #[cfg(test)]
        {
            let path = backtrace(&came_from, (end_pos, 1));
            println!("Path: {:?}", path);
        }
        *end_pos_cost_1 as u128
    }
}

#[cfg(test)]
fn backtrace(
    came_from: &HashMap<DirectedPositon, DirectedPositon>,
    end_pos: DirectedPositon,
) -> Vec<Coordinate> {
    let mut path = vec![end_pos.0];
    let mut current_pos = end_pos;
    while let Some(&next_pos) = came_from.get(&current_pos) {
        path.push(next_pos.0);
        current_pos = next_pos;
    }
    path
}

fn get_neighbours(
    map: &Map,
    pos: Coordinate,
    axis: Axis,
    min_move: isize,
    max_move: isize,
) -> Vec<Coordinate> {
    let width = map[0].len() as isize;
    let height = map.len() as isize;

    let mut neighbours: Vec<Coordinate> = vec![];

    if axis == 0 {
        let below = (pos.0 + min_move..=pos.0 + max_move).map(|y| (y, pos.1));
        let above = (pos.0 - max_move..=pos.0 - min_move).map(|y| (y, pos.1));

        neighbours.extend(below);
        neighbours.extend(above);
    } else {
        let right = (pos.1 + min_move..=pos.1 + max_move).map(|x| (pos.0, x));
        let left = (pos.1 - max_move..=pos.1 - min_move).map(|x| (pos.0, x));

        neighbours.extend(right);
        neighbours.extend(left);
    }

    neighbours
        .into_iter()
        .filter(|&x| x.0 >= 0 && x.0 < height && x.1 >= 0 && x.1 < width)
        .collect()
}

fn get_path_cost(map: &Map, from: Coordinate, to: Coordinate) -> isize {
    let y_range = min(from.0, to.0)..=max(from.0, to.0);
    let x_range = min(from.1, to.1)..=max(from.1, to.1);
    y_range
        .flat_map(|y| x_range.clone().map(move |x| (y, x)))
        .filter(|pos| pos.0 != from.0 || pos.1 != from.1)
        .map(|(y, x)| map[y as usize][x as usize])
        .sum()
}

// 0 = x, 1 = y
type Axis = isize;
type Coordinate = (isize, isize);
type DirectedPositon = (Coordinate, Axis);

type Map = Vec<Vec<isize>>;
fn parse_file(file: &str) -> Map {
    let contents = std::fs::read_to_string(file).unwrap();

    contents.lines().map(parse_line).collect()
}
fn parse_line(line: &str) -> Vec<isize> {
    line.chars()
        .map(|c| c.to_digit(10).unwrap() as isize)
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::year2023::day17::shortest_path_ultra;

    use super::shortest_path;

    #[test]
    fn test_astar() {
        let shortest = shortest_path("resources/2023/day17/test_input");

        assert_eq!(shortest, 102);
    }

    #[test]
    fn test_astar_ultra() {
        let shortest = shortest_path_ultra("resources/2023/day17/test_input");

        assert_eq!(shortest, 94);
    }
}
