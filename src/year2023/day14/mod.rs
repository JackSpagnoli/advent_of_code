use std::{cmp::Ordering, collections::HashMap};

pub mod task1 {
    use super::total_load;

    pub fn ans() -> u128 {
        total_load("resources/2023/day14/input")
    }
}

pub mod task2 {
    use super::billion_cycles_load;

    pub fn ans() -> u128 {
        billion_cycles_load("resources/2023/day14/input")
    }
}

fn total_load(file: &str) -> u128 {
    let grid = parse_file(file);
    let rolled_grid = roll_north(grid);

    grid_load(rolled_grid)
}

fn billion_cycles_load(file: &str) -> u128 {
    let grid = parse_file(file);
    let rolled_grid = billion_cycles(grid);

    grid_load(rolled_grid)
}

fn grid_load(grid: Grid) -> u128 {
    grid
        .into_iter()
        .rev()
        .enumerate()
        .map(|(index, row)| {
            (row.into_iter()
                .filter(|tile| *tile == Tile::Rolling)
                .count()
                * (index + 1)) as u128
        })
        .sum()
}

fn billion_cycles(grid: Grid) -> Grid {
    let number_of_cycles = 1_000_000_000;

    let mut memory: HashMap<Grid, usize> = HashMap::new();

    let mut current_grid = grid;
    let mut cycles = 0;
    memory.insert(current_grid.clone(), cycles);

    loop {
        current_grid = cycle(current_grid);
        cycles += 1;

        if cycles == number_of_cycles {
            return current_grid;
        }

        if memory.contains_key(&current_grid) {
            let previous_cycles = *memory.get(&current_grid).unwrap();
            let loop_length = cycles - previous_cycles;

            let remaining_cycles = number_of_cycles - cycles;
            let mod_cycles = remaining_cycles % loop_length;

            return memory
                .into_iter()
                .find(|(_, index)| *index == mod_cycles + previous_cycles)
                .unwrap()
                .0;
        }

        memory.insert(current_grid.clone(), cycles);
    }
}

fn cycle(grid: Grid) -> Grid {
    roll_east(roll_south(roll_west(roll_north(grid))))
}

fn roll_north(grid: Grid) -> Grid {
    let mut transposed_grid = transpose(&grid);

    transposed_grid = transposed_grid.into_iter().map(roll_column).collect();

    transpose(&transposed_grid)
}

fn roll_south(grid: Grid) -> Grid {
    let mut transposed_grid = transpose(&grid);

    transposed_grid = transposed_grid
        .into_iter()
        .map(|mut column| {
            column.reverse();
            column
        })
        .map(roll_column)
        .map(|mut column| {
            column.reverse();
            column
        })
        .collect();

    transpose(&transposed_grid)
}

fn roll_west(grid: Grid) -> Grid {
    grid.into_iter().map(roll_column).collect()
}

fn roll_east(grid: Grid) -> Grid {
    grid.into_iter()
        .map(|mut column| {
            column.reverse();
            column
        })
        .map(roll_column)
        .map(|mut column| {
            column.reverse();
            column
        })
        .collect()
}

fn roll_column(mut column: Vec<Tile>) -> Vec<Tile> {
    let n = column.len();
    (0..n - 1).for_each(|i| {
        (0..n - i - 1).for_each(|j| {
            if sort_tiles(&column[j], &column[j + 1]) == Ordering::Greater {
                column.swap(j, j + 1)
            }
        })
    });
    column
}

fn transpose(grid: &Grid) -> Grid {
    let mut transposed_pattern = vec![vec![Tile::Empty; grid.len()]; grid[0].len()];

    grid.iter().enumerate().for_each(|(j, row)| {
        row.iter().enumerate().for_each(|(i, tile)| {
            transposed_pattern[i][j] = *tile;
        })
    });

    transposed_pattern
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    Static,
    Rolling,
}
type Grid = Vec<Vec<Tile>>;
fn parse_file(file: &str) -> Grid {
    let contents = std::fs::read_to_string(file).expect("Something went wrong reading the file");

    contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    '#' => Tile::Static,
                    'O' => Tile::Rolling,
                    _ => panic!("Invalid character in input file"),
                })
                .collect::<Vec<Tile>>()
        })
        .collect::<Vec<Vec<Tile>>>()
}

fn sort_tiles(a: &Tile, b: &Tile) -> Ordering {
    match (a, b) {
        (Tile::Rolling, _) => Ordering::Less,
        (Tile::Empty, Tile::Rolling) => Ordering::Greater,
        (_, _) => Ordering::Equal,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_file() {
        let grid = parse_file("resources/2023/day14/test_input");

        grid.iter().for_each(|row| {
            println!("{:?}", row);
        });
    }

    #[test]
    fn test_roll_tiles() {
        let grid = parse_file("resources/2023/day14/test_input");
        let rolled_grid = roll_north(grid);

        rolled_grid.iter().for_each(|row| {
            println!("{:?}", row);
        });
    }

    #[test]
    fn test_total_load() {
        assert_eq!(total_load("resources/2023/day14/test_input"), 136);
    }

    #[test]
    fn test_cycle() {
        let grid = parse_file("resources/2023/day14/test_input");
        let cycled_grid = cycle(grid);

        cycled_grid.iter().for_each(|row| {
            println!("{:?}", row);
        });
    }

    #[test]
    fn test_billion_cycles() {
        assert_eq!(
            billion_cycles_load("resources/2023/day14/test_input"),
            64
        );
    }
}
