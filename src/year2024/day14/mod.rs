use std::{collections::HashMap, fs};

use regex::Regex;

pub mod task1 {
    use super::hundred_second_quadrant_score;

    pub fn ans() -> u128 {
        hundred_second_quadrant_score("resources/2024/day14/input.txt", 103, 101)
    }
}

pub mod task2 {
    use super::low_entropy;

    pub fn ans() -> u128 {
        low_entropy("resources/2024/day14/input.txt")
    }
}

fn hundred_second_quadrant_score(file: &str, height: isize, width: isize) -> u128 {
    let robots = parse_file(file);

    quadrant_product(robots, height, width, 100)
}

#[derive(Clone)]
struct Robot {
    p_x: isize,
    p_y: isize,
    v_x: isize,
    v_y: isize,
}

impl From<&str> for Robot {
    fn from(s: &str) -> Self {
        let regex = Regex::new(r"(-?\d+)").unwrap();

        let mut captures = regex.captures_iter(s);

        let p_x = captures.next().unwrap()[0].parse().unwrap();
        let p_y = captures.next().unwrap()[0].parse().unwrap();
        let v_x = captures.next().unwrap()[0].parse().unwrap();
        let v_y = captures.next().unwrap()[0].parse().unwrap();

        Robot { p_x, p_y, v_x, v_y }
    }
}

impl Robot {
    fn move_robot(mut self, times: isize) -> Self {
        self.p_x += times * self.v_x;
        self.p_y += times * self.v_y;

        self
    }

    fn wrap_around(mut self, height: isize, width: isize) -> Self {
        self.p_x = ((self.p_x % width) + width) % width;
        self.p_y = ((self.p_y % height) + height) % height;

        self
    }
}

fn parse_file(file: &str) -> Vec<Robot> {
    let contents = fs::read_to_string(file).unwrap();

    contents.lines().map(Robot::from).collect()
}

fn quadrant_product(robots: Vec<Robot>, height: isize, width: isize, times: isize) -> u128 {
    let half_width = width / 2;
    let half_height = height / 2;

    let quad_counts = robots
        .into_iter()
        .map(|r| r.move_robot(times))
        .map(|r| r.wrap_around(height, width))
        .fold((0, 0, 0, 0), |acc, r| {
            match (
                r.p_x < half_width,
                r.p_x > half_width,
                r.p_y < half_height,
                r.p_y > half_height,
            ) {
                (true, false, true, false) => (acc.0 + 1, acc.1, acc.2, acc.3),
                (false, true, true, false) => (acc.0, acc.1 + 1, acc.2, acc.3),
                (true, false, false, true) => (acc.0, acc.1, acc.2 + 1, acc.3),
                (false, true, false, true) => (acc.0, acc.1, acc.2, acc.3 + 1),
                _ => acc,
            }
        });

    quad_counts.0 * quad_counts.1 * quad_counts.2 * quad_counts.3
}

fn low_entropy(file: &str) -> u128 {
    // Look for an image being formed by iterating a number of cycles (10_000)
    // and then checking if the entropy is low enough to be considered an image
    // Use the quadrant_product function as entropy measure

    let robots = parse_file(file);

    let mut entropies: HashMap<u128, usize> = HashMap::new();

    for i in 0..10_000 {
        let entropy = quadrant_product(robots.clone(), 103, 101, i as isize);

        if let None = entropies.get(&entropy) {
            entropies.insert(entropy, i);
        }
    }

    let index_of_low_entropy = entropies
        .get_key_value(&entropies.keys().min().unwrap())
        .unwrap()
        .1;

    *index_of_low_entropy as u128
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quadrant_product() {
        assert_eq!(
            hundred_second_quadrant_score("resources/2024/day14/test_input.txt", 7, 11),
            12
        );
    }

    // #[test]
    // fn investigate_low_entropy() {
    //     let robots = parse_file("resources/2024/day14/input.txt");

    //     let low_entropy_time = low_entropy("resources/2024/day14/input.txt");

    //     let moved_robots = robots
    //         .into_iter()
    //         .map(|r| r.move_robot(low_entropy_time as isize))
    //         .map(|r| r.wrap_around(103, 101))
    //         .map(|r| (r.p_x, r.p_y))
    //         .collect::<std::collections::HashSet<(isize, isize)>>();

    //     for y in 0..103 {
    //         for x in 0..101 {
    //             if moved_robots.contains(&(x, y)) {
    //                 print!("#");
    //             } else {
    //                 print!(".");
    //             }
    //         }
    //         println!();
    //     }

    //     assert!(false);
    // }
}
