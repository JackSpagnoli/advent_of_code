use std::fs;

use num::Integer;

pub mod task1 {
    use super::longest_path;

    pub fn ans() -> u128 {
        longest_path("resources/2023/day10/input")
    }
}

pub mod task2 {

    pub fn ans() -> u128 {
        1
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn longest_path(file: &str) -> u128 {
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

    loop{
        steps += 1;
        match next_direction {
            Direction::Up => {
                next_pos.0 -= 1;
            },
            Direction::Down => {
                next_pos.0 += 1;
            },
            Direction::Left => {
                next_pos.1 -= 1;
            },
            Direction::Right => {
                next_pos.1 += 1;
            },
        }
        
        let next_char = rows[next_pos.0][next_pos.1];

        if next_char == 'S' {
            return steps.div_ceil(&2)
        }

        next_direction = change_direction(next_direction, next_char)
    }
}

fn change_direction(direction: Direction, cell: char)-> Direction{
    if cell == '-' || cell == '|' {
        return direction;
    }

    match (cell, direction) {
        ('F', Direction::Up)| ('L', Direction::Down) => Direction::Right,
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
}