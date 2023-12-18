pub mod task1 {
    use super::contained_area;

    pub fn ans() -> u128 {
        contained_area("resources/2023/day18/input")
    }
}

pub mod task2 {
    use super::contained_area_hex;

    pub fn ans() -> u128 {
        contained_area_hex("resources/2023/day18/input")
    }
}

fn contained_area_hex(file: &str) -> u128 {
    let (vertices, steps) = parse_file_hex(file);

    get_area(vertices, steps)
}

fn contained_area(file: &str) -> u128 {
    let (vertices, steps) = parse_file(file);

    get_area(vertices, steps)
}

fn get_area(vertices: Vec<Vertex>, steps: isize) -> u128 {
    let vertices_count = vertices.len();
    let looped_vertices: Vec<Vertex> = vertices
        .into_iter()
        .cycle()
        .take(vertices_count + 1)
        .collect();

    //Shoelace algorithm
    let internal_area = looped_vertices
        .windows(2)
        .map(|window| {
            let (y1, x1) = window[0].position;
            let (y2, x2) = window[1].position;
            (y1 + y2) * (x1 - x2)
        })
        .sum::<isize>()
        .unsigned_abs() as u128
        / 2;

    let edge_area = (steps / 2) as u128 + 1;

    internal_area + edge_area
}

type Coordinate = (isize, isize);
#[derive(Debug, Clone)]
struct Vertex {
    position: Coordinate,
}
fn parse_file(file: &str) -> (Vec<Vertex>, isize) {
    let contents = std::fs::read_to_string(file).unwrap();

    let regex = regex::Regex::new(r"(?<direction>[URDL]) (?<steps>\d+) ").unwrap();

    let mut current_position = (0, 0);
    let mut total_steps = 0;
    let vertices = contents
        .lines()
        .map(|line| {
            let captures = regex.captures_iter(line).next().unwrap();

            let direction = captures.name("direction").unwrap().as_str();
            let steps = captures
                .name("steps")
                .unwrap()
                .as_str()
                .parse::<isize>()
                .unwrap();

            total_steps += steps;

            match direction {
                "D" => current_position.0 += steps,
                "U" => current_position.0 -= steps,
                "R" => current_position.1 += steps,
                "L" => current_position.1 -= steps,
                _ => panic!("Invalid direction"),
            }

            Vertex {
                position: current_position,
            }
        })
        .collect();

    (vertices, total_steps)
}

fn parse_file_hex(file: &str) -> (Vec<Vertex>, isize) {
    let contents = std::fs::read_to_string(file).unwrap();

    let regex = regex::Regex::new(r"\(#(.{6})\)").unwrap();

    let mut current_position = (0, 0);
    let mut total_steps = 0;
    let vertices = contents
        .lines()
        .map(|line| {
            let captures = regex.captures_iter(line).next().unwrap();
            let hex = captures.get(0).unwrap().as_str();

            // Convert the first 5 characters as a hexadecimal number to a usize
            let steps = usize::from_str_radix(&hex[2..7], 16).unwrap() as isize;

            total_steps += steps;

            match hex.chars().nth(7).unwrap() {
                '1' => current_position.0 += steps,
                '3' => current_position.0 -= steps,
                '0' => current_position.1 += steps,
                '2' => current_position.1 -= steps,
                _ => panic!("Invalid direction"),
            }

            Vertex {
                position: current_position,
            }
        })
        .collect();

    (vertices, total_steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contained_area() {
        assert_eq!(contained_area("resources/2023/day18/test_input"), 62);
    }

    #[test]
    fn test_contained_area_hex() {
        assert_eq!(
            contained_area_hex("resources/2023/day18/test_input"),
            952408144115u128
        );
    }
}
