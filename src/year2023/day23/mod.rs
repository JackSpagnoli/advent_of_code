use std::collections::HashMap;

pub mod task1 {
    use super::longest_path;

    pub fn ans() -> u128 {
        longest_path("resources/2023/day23/input", false)
    }
}

pub mod task2 {
    use super::longest_path;

    pub fn ans() -> u128 {
        longest_path("resources/2023/day23/input", true)
    }
}

fn longest_path(file: &str, bidirectional: bool) -> u128 {
    let maze = parse_file(file, bidirectional);

    // Use a depth first search to find the longest path that doesn't visit the same junction twice
    let mut stack = vec![(0, vec![maze.start])];
    let mut max_distance = 0;
    while let Some((dist, path)) = stack.pop() {
        if path.last() == Some(&maze.end) {
            max_distance = max_distance.max(dist);
            continue;
        }

        let junction = maze.junctions.get(path.last().unwrap()).unwrap();
        for (neighbour, distance) in junction.neighbours.iter() {
            if !path.contains(neighbour) {
                let mut new_path = path.clone();
                new_path.push(*neighbour);
                stack.push((dist + *distance as u128, new_path));
            }
        }
    }

    max_distance
}

type Coordinate = (isize, isize);
type ID = usize;
type Distance = usize;
#[derive(Debug)]
struct Junction {
    coordinate: Coordinate,
    neighbours: HashMap<ID, Distance>,
}
struct Maze {
    junctions: HashMap<ID, Junction>,
    start: ID,
    end: ID,
}
fn parse_file(file: &str, bidirectional: bool) -> Maze {
    let contents = std::fs::read_to_string(file).unwrap();

    let mut lines = contents.lines();

    let top_row = lines.next().unwrap();
    let start_x = top_row.find('.').unwrap();
    let start_junction = Junction {
        coordinate: (0, start_x as isize),
        neighbours: HashMap::new(),
    };

    let bottom_row = lines.next_back().unwrap();
    let end_x = bottom_row.find('.').unwrap();
    let end_y = lines.count() + 1;
    let end_junction = Junction {
        coordinate: (end_y as isize, end_x as isize),
        neighbours: HashMap::new(),
    };

    let chars = contents
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut junctions = HashMap::new();
    junctions.insert(0, start_junction);
    junctions.insert(1, end_junction);

    // Use a BFS search to find all junctions
    let mut queue = vec![(0, (1, 0))];
    while let Some((junction_id, direction)) = queue.pop() {
        let junction = junctions.get(&junction_id).unwrap();
        let (mut y, mut x) = junction.coordinate;
        let (mut dy, mut dx) = direction;

        let mut distance = 0;
        let mut forwards = true;
        let mut backwards = true;

        loop {
            distance += 1;
            y += dy;
            x += dx;

            if y == end_y as isize && x == end_x as isize {
                junctions
                    .get_mut(&junction_id)
                    .unwrap()
                    .neighbours
                    .insert(1, distance);
                junctions
                    .get_mut(&1)
                    .unwrap()
                    .neighbours
                    .insert(junction_id, distance);
                break;
            }

            let neighbours: Vec<Coordinate> = [(1, 0), (-1, 0), (0, 1), (0, -1)]
                .into_iter()
                .filter(|d| chars[(y + d.0) as usize][(x + d.1) as usize] != '#')
                .filter(|d| d.0 != -dy || d.1 != -dx)
                .collect();

            if neighbours.is_empty() {
                break;
            }

            if neighbours.len() == 1 {
                let neighbour = neighbours[0];
                dy = neighbour.0;
                dx = neighbour.1;

                let directionality = direcionality(direction, chars[y as usize][x as usize]);
                forwards = forwards && directionality.0;
                backwards = backwards && directionality.1;

                continue;
            }

            if neighbours.len() > 1 {
                let next_junction_id = if let Some(next_junction) =
                    junctions.iter().find(|(_, j)| j.coordinate == (y, x))
                {
                    *next_junction.0
                } else {
                    let next_junction_id = junctions.len();
                    let next_junction = Junction {
                        coordinate: (y, x),
                        neighbours: HashMap::new(),
                    };
                    junctions.insert(next_junction_id, next_junction);

                    for neighbour in neighbours {
                        queue.push((next_junction_id, neighbour));
                    }

                    next_junction_id
                };

                if forwards || bidirectional {
                    junctions
                        .get_mut(&junction_id)
                        .unwrap()
                        .neighbours
                        .insert(next_junction_id, distance);
                }
                if backwards || bidirectional {
                    junctions
                        .get_mut(&next_junction_id)
                        .unwrap()
                        .neighbours
                        .insert(junction_id, distance);
                }

                break;
            }
        }
    }

    Maze {
        junctions,
        start: 0,
        end: 1,
    }
}

fn direcionality(direction: Coordinate, tile: char) -> (bool, bool) {
    if direction == (1, 0) {
        if tile == '^' {
            return (false, true);
        }
        if tile == 'v' {
            return (true, false);
        }
    }
    if direction == (-1, 0) {
        if tile == '^' {
            return (true, false);
        }
        if tile == 'v' {
            return (false, true);
        }
    }
    if direction == (0, 1) {
        if tile == '>' {
            return (true, false);
        }
        if tile == '<' {
            return (false, true);
        }
    }
    if direction == (0, -1) {
        if tile == '>' {
            return (false, true);
        }
        if tile == '<' {
            return (true, false);
        }
    }
    (true, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_file() {
        let maze = parse_file("resources/2023/day23/test_input", false);

        let mut junctions = maze.junctions.iter().collect::<Vec<(&usize, &Junction)>>();
        junctions.sort_by_key(|(id, _)| *id);
        println!("{:?}", junctions);
    }

    #[test]
    fn test_max_distance() {
        assert_eq!(longest_path("resources/2023/day23/test_input", false), 94);
    }

    #[test]
    fn test_max_distance_bidirectional() {
        assert_eq!(longest_path("resources/2023/day23/test_input", true), 154);
    }
}

