use std::{collections::HashSet, fs};

pub mod task1 {
    use super::sum_pushed_coordinates;

    pub fn ans() -> u128 {
        sum_pushed_coordinates("resources/2024/day15/input.txt")
    }
}

pub mod task2 {
    pub fn ans() -> u128 {
        0
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn as_delta(&self) -> (isize, isize) {
        match self {
            Direction::Down => (0, 1),
            Direction::Up => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

fn sum_pushed_coordinates(file: &str) -> u128 {
    let contents = fs::read_to_string(file).unwrap();
    let mut split = contents.split("\n\n");
    let map = split.next().unwrap();

    let mut walls: HashSet<(isize, isize)> = HashSet::new();
    let mut boxes: HashSet<(isize, isize)> = HashSet::new();

    let mut robot: (isize, isize) = (0, 0);

    map.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| match c {
            '#' => {
                walls.insert((x as isize, y as isize));
            }
            'O' => {
                boxes.insert((x as isize, y as isize));
            }
            '@' => {
                robot = (x as isize, y as isize);
            }
            '.' => (),
            _ => panic!("Invalid character"),
        })
    });

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
        let mut push_stack = vec![];

        let (dx, dy) = command.as_delta();

        let mut next = (robot.0, robot.1);
        loop {
            next = (next.0 + dx, next.1 + dy);

            if walls.contains(&next) {
                push_stack.clear();
                break;
            }

            if boxes.contains(&next) {
                push_stack.push(next);
                continue;
            }

            break;
        }

        if !push_stack.is_empty() || !walls.contains(&next) {
            robot = (robot.0 + dx, robot.1 + dy);
        }

        if push_stack.is_empty() {
            continue;
        }

        let first_box = push_stack.first().unwrap();
        let last_box = push_stack.last().unwrap();

        let next = (last_box.0 + dx, last_box.1 + dy);

        boxes.remove(&first_box);
        boxes.insert(next);
    }

    boxes
        .into_iter()
        .map(|(x, y)| x + (100 * y))
        .map(|x| x as u128)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_pushed_coordinates() {
        assert_eq!(
            sum_pushed_coordinates("resources/2024/day15/test_input_2.txt"),
            2028
        );

        assert_eq!(
            sum_pushed_coordinates("resources/2024/day15/test_input.txt"),
            10092
        );
    }
}
