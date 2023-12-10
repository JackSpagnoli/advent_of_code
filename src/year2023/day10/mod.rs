use std::fs;

pub mod task1 {
    use super::longest_path;

    pub fn ans() -> u128 {
        longest_path("resources/2023/day10/input")
    }
}

pub mod task2 {
    use super::contained_area;

    pub fn ans() -> u128 {
        contained_area("resources/2023/day10/input")
    }
}

fn longest_path(file: &str) -> u128 {
    trace_path(file).steps.div_ceil(2)
}

fn contained_area(file: &str) -> u128 {
    let path_trace = trace_path(file);
    let mut vertices = path_trace.vertices;

    // Add the first vertex to the end to close the polygon.
    vertices.push(vertices[0]);

    // Using the shoelace formula:
    // 2 * Area = sum_i( (y_i + y_i+1) * (x_i - x_i+1) )
    let area = (vertices
        .windows(2)
        .map(cast_coordinates)
        .map(|pair| (pair[0].0 + pair[1].0) * (pair[0].1 - pair[1].1))
        .sum::<isize>()
        .abs()
        / 2) as u128;

    // Each step adds 1/2 a unit of area, with the last step counted twice
    let edge_area = ((path_trace.steps) / 2) - 1;

    area - edge_area
}

fn cast_coordinates(coordinates: &[Coordinate]) -> Vec<(isize, isize)> {
    coordinates
        .iter()
        .map(|coordinate| (coordinate.0 as isize, coordinate.1 as isize))
        .collect()
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Coordinate = (usize, usize);
struct PathTrace {
    steps: u128,
    vertices: Vec<Coordinate>,
}
fn trace_path(file: &str) -> PathTrace {
    let contents = fs::read_to_string(file).expect("Error reading the file");

    let rows: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let start_row = rows.iter().position(|row| row.contains(&'S')).unwrap();
    let start_column = rows[start_row].iter().position(|&c| c == 'S').unwrap();

    let mut next_pos = (start_row, start_column);
    let mut steps = 0;

    // Select initial direction.
    // If can go right, go right, if not check down, if can do neither default to up.
    let mut next_direction = Direction::Up;
    let right_cell = rows[next_pos.0][next_pos.1 + 1];
    let down_cell = rows[next_pos.0 + 1][next_pos.1];
    if right_cell == '-' || right_cell == 'J' || right_cell == '7' {
        next_direction = Direction::Right;
    } else if down_cell == '|' || down_cell == 'L' || down_cell == 'J' {
        next_direction = Direction::Down;
    }

    let mut vertices = vec![next_pos];
    loop {
        steps += 1;
        match next_direction {
            Direction::Up => {
                next_pos.0 -= 1;
            }
            Direction::Down => {
                next_pos.0 += 1;
            }
            Direction::Left => {
                next_pos.1 -= 1;
            }
            Direction::Right => {
                next_pos.1 += 1;
            }
        }

        let next_char = rows[next_pos.0][next_pos.1];

        if next_char == 'S' {
            return PathTrace { steps, vertices };
        }

        let last_direction = next_direction;
        next_direction = change_direction(next_direction, next_char);

        if next_direction != last_direction {
            vertices.push(next_pos);
        }
    }
}

fn change_direction(direction: Direction, cell: char) -> Direction {
    if cell == '-' || cell == '|' {
        return direction;
    }

    match (cell, direction) {
        ('F', Direction::Up) | ('L', Direction::Down) => Direction::Right,
        ('F', Direction::Left) | ('7', Direction::Right) => Direction::Down,
        ('J', Direction::Down) | ('7', Direction::Up) => Direction::Left,
        ('J', Direction::Right) | ('L', Direction::Left) => Direction::Up,
        _ => panic!("Bruh"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_path() {
        assert_eq!(longest_path("resources/2023/day10/test_input"), 4);
        assert_eq!(longest_path("resources/2023/day10/test_input2"), 8);
    }

    #[test]
    fn test_contained_area() {
        assert_eq!(contained_area("resources/2023/day10/test_input3"), 4);
        assert_eq!(contained_area("resources/2023/day10/test_input4"), 8);
        assert_eq!(contained_area("resources/2023/day10/test_input5"), 10);
    }
}
