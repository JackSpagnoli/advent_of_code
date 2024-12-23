use core::panic;
use std::{collections::HashSet, fs};

pub mod task1 {
    use super::sum_pushed_coordinates;

    pub fn ans() -> u128 {
        sum_pushed_coordinates("resources/2024/day15/input.txt", false)
    }
}

pub mod task2 {
    pub fn ans() -> u128 {
        super::sum_pushed_coordinates("resources/2024/day15/input.txt", true)
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_map(
    map: &str,
    expand: bool,
) -> (
    (isize, isize),
    HashSet<(isize, isize)>,
    HashSet<(isize, isize)>,
) {
    let mut walls: HashSet<(isize, isize)> = HashSet::new();
    let mut boxes: HashSet<(isize, isize)> = HashSet::new();

    let mut robot: (isize, isize) = (0, 0);

    map.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| (x as isize, y as isize, c))
        })
        .map(|(x, y, c)| if expand { (2 * x, y, c) } else { (x, y, c) })
        .for_each(|(x, y, c)| match c {
            '#' => {
                walls.insert((x, y));
                if expand {
                    walls.insert((x + 1, y));
                }
            }
            'O' => {
                boxes.insert((x, y));
            }
            '@' => {
                robot = (x, y);
            }
            '.' => (),
            _ => panic!("Invalid character"),
        });

    (robot, boxes, walls)
}

fn sum_pushed_coordinates(file: &str, expand: bool) -> u128 {
    let contents = fs::read_to_string(file).unwrap();
    let mut split = contents.split("\n\n");
    let map = split.next().unwrap();

    let (mut robot, mut boxes, walls) = parse_map(map, expand);

    let mut commands = split
        .next()
        .unwrap()
        .lines()
        .flat_map(|line| line.chars())
        .map(|char| match char {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Invalid direction"),
        });

    while let Some(command) = commands.next() {
        make_move(&command, &mut robot, &mut boxes, &walls, expand);
    }

    boxes
        .into_iter()
        .map(|(x, y)| x + (100 * y))
        .map(|x| x as u128)
        .sum()
}

fn make_move(
    command: &Direction,
    robot: &mut (isize, isize),
    boxes: &mut HashSet<(isize, isize)>,
    walls: &HashSet<(isize, isize)>,
    expand: bool,
) {
    let mut boxes_to_push = HashSet::new();

    let mut frontier: HashSet<(isize, isize)> = HashSet::new();
    match (command, expand) {
        (Direction::Up, false) => {
            frontier.insert((robot.0, robot.1 - 1));
        }
        (Direction::Up, true) => {
            frontier.insert((robot.0, robot.1 - 1));
            if !walls.contains(&(robot.0 - 1, robot.1 - 1)) {
                frontier.insert((robot.0 - 1, robot.1 - 1));
            }
        }
        (Direction::Down, false) => {
            frontier.insert((robot.0, robot.1 + 1));
        }
        (Direction::Down, true) => {
            frontier.insert((robot.0, robot.1 + 1));
            if !walls.contains(&(robot.0 - 1, robot.1 + 1)) {
                frontier.insert((robot.0 - 1, robot.1 + 1));
            }
        }
        (Direction::Right, _) => {
            frontier.insert((robot.0 + 1, robot.1));
        }
        (Direction::Left, false) => {
            frontier.insert((robot.0 - 1, robot.1));
        }
        (Direction::Left, true) => {
            if boxes.contains(&(robot.0 - 2, robot.1)) {
                frontier.insert((robot.0 - 2, robot.1));
            } else {
                frontier.insert((robot.0 - 1, robot.1));
            }
        }
    };

    loop {
        let current = match frontier.iter().next() {
            Some(current) => *current,
            None => break,
        };
        frontier.remove(&current);

        let is_wall = walls.contains(&current);
        let is_box = boxes.contains(&current);

        if !(is_wall || is_box) {
            continue;
        }

        if is_wall {
            boxes_to_push.clear();
            return;
        }

        if !is_box {
            continue;
        }
        boxes_to_push.insert(current);

        match (command, expand) {
            (Direction::Right, false) => {
                frontier.insert((current.0 + 1, current.1));
            }
            (Direction::Right, true) => {
                // Must be on a box, so next non-box tile is 2 away
                frontier.insert((current.0 + 2, current.1));
            }
            (Direction::Left, false) => {
                frontier.insert((current.0 - 1, current.1));
            }
            (Direction::Left, true) => {
                if boxes.contains(&(current.0 - 2, current.1)) {
                    frontier.insert((current.0 - 2, current.1));
                } else {
                    frontier.insert((current.0 - 1, current.1));
                }
            }
            (Direction::Up, false) => {
                frontier.insert((current.0, current.1 - 1));
            }
            (Direction::Up, true) => {
                frontier.insert((current.0, current.1 - 1));
                frontier.insert((current.0 + 1, current.1 - 1));
                if boxes.contains(&(current.0 - 1, current.1 - 1)) {
                    frontier.insert((current.0 - 1, current.1 - 1));
                }
            }
            (Direction::Down, false) => {
                frontier.insert((current.0, current.1 + 1));
            }
            (Direction::Down, true) => {
                frontier.insert((current.0, current.1 + 1));
                frontier.insert((current.0 + 1, current.1 + 1));
                if boxes.contains(&(current.0 - 1, current.1 + 1)) {
                    frontier.insert((current.0 - 1, current.1 + 1));
                }
            }
        }
    }

    let push_delta = match command {
        Direction::Up => (0, -1),
        Direction::Down => (0, 1),
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0),
    };

    if !boxes_to_push.is_empty()
        || !walls.contains(&(robot.0 + push_delta.0, robot.1 + push_delta.1))
    {
        *robot = (robot.0 + push_delta.0, robot.1 + push_delta.1);
    }

    if boxes_to_push.is_empty() {
        return;
    }

    let new_boxes: HashSet<(isize, isize)> = boxes_to_push
        .iter()
        .map(|r#box| (r#box.0 + push_delta.0, r#box.1 + push_delta.1))
        .collect();

    let boxes_to_remove = boxes_to_push.difference(&new_boxes);
    let boxes_to_add = new_boxes.difference(&boxes_to_push);

    boxes_to_remove.for_each(|r#box| {
        boxes.remove(r#box);
    });

    boxes_to_add.for_each(|r#box| {
        boxes.insert(*r#box);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_pushed_coordinates() {
        assert_eq!(
            sum_pushed_coordinates("resources/2024/day15/test_input_2.txt", false),
            2028
        );

        assert_eq!(
            sum_pushed_coordinates("resources/2024/day15/test_input.txt", false),
            10092
        );

        assert_eq!(
            sum_pushed_coordinates("resources/2024/day15/test_input.txt", true),
            9021
        );

        assert_eq!(
            sum_pushed_coordinates("resources/2024/day15/test_input_5.txt", true),
            406
        );

        assert_eq!(
            sum_pushed_coordinates("resources/2024/day15/test_input_6.txt", true),
            509
        );
    }
}
