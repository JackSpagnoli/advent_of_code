use std::fs;

use pathfinding::directed::dijkstra::dijkstra;

pub mod task1 {
    use super::find_shortest_path;

    pub fn ans() -> u128 {
        find_shortest_path("resources/2022/day12/input") as u128
    }
}

pub mod task2 {
    use super::find_shortest_path_to_low_ground;

    pub fn ans() -> u128 {
        find_shortest_path_to_low_ground("resources/2022/day12/input") as u128
    }
}

fn successors(
    (y, x): (usize, usize),
    height_map: &Vec<Vec<isize>>,
    up_limit: bool,
) -> Vec<((usize, usize), usize)> {
    if y >= height_map.len() || x >= height_map[0].len() {
        return vec![];
    }
    let mut orthogonal_paths: Vec<(usize, usize)> = vec![];
    if y <= height_map.len() - 2
        && ((height_map[y + 1][x] - height_map[y][x] <= 1 && up_limit)
            || (height_map[y + 1][x] - height_map[y][x] >= -1 && !up_limit))
    {
        orthogonal_paths.push((y + 1, x));
    }
    if y >= 1
        && ((height_map[y - 1][x] - height_map[y][x] <= 1 && up_limit)
            || (height_map[y - 1][x] - height_map[y][x] >= -1 && !up_limit))
    {
        orthogonal_paths.push((y - 1, x));
    }
    if x <= height_map[0].len() - 2
        && ((height_map[y][x + 1] - height_map[y][x] <= 1 && up_limit)
            || (height_map[y][x + 1] - height_map[y][x] >= -1 && !up_limit))
    {
        orthogonal_paths.push((y, x + 1));
    }
    if x >= 1
        && ((height_map[y][x - 1] - height_map[y][x] <= 1 && up_limit)
            || (height_map[y][x - 1] - height_map[y][x] >= -1 && !up_limit))
    {
        orthogonal_paths.push((y, x - 1));
    }
    // println!("({y},{x}) : {orthogonal_paths:?}");
    orthogonal_paths.into_iter().map(|p| (p, 1)).collect()
}

fn find_shortest_path(file: &str) -> usize {
    let (height_map, destination_pos, start_point) = generate_height_map(file);
    let result = dijkstra(
        &start_point,
        |&(y, x)| successors((y, x), &height_map, true),
        |p| *p == destination_pos,
    )
    .expect("No path");
    result.1
}

fn find_shortest_path_to_low_ground(file: &str) -> usize {
    let (height_map, start_point, _) = generate_height_map(file);
    let result = dijkstra(
        &start_point,
        |&(y, x)| successors((y, x), &height_map, false),
        |&(y, x)| height_map[y][x] == 0,
    )
    .expect("No path");
    result.1
}


type HeightMap = Vec<Vec<isize>>;
type Point = (usize, usize);
fn generate_height_map(file: &str) -> (HeightMap, Point, Point) {
    let contents = fs::read_to_string(file).expect("Error reading file");
    let lines = contents.lines();

    let mut height_map: Vec<Vec<isize>> = vec![];
    let mut end_point: (usize, usize) = (0, 0);
    let mut start_point: (usize, usize) = (0, 0);

    for (j, line) in lines.enumerate() {
        let mut i = 0;
        height_map.push(
            line.chars()
                .map(|x| {
                    i += 1;
                    if x == 'E' {
                        end_point = (j, i - 1);
                        return 25;
                    } else if x == 'S' {
                        start_point = (j, i - 1);
                        return 0;
                    }
                    (x as isize) - 97
                })
                .collect(),
        );
    }
    (height_map, end_point, start_point)
}
