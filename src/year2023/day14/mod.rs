use std::{
    cmp::Ordering,
    collections::HashMap,
    ops::{Index, IndexMut},
};

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
    parse_file(file).roll_north().grid_load()
}

fn billion_cycles_load(file: &str) -> u128 {
    let grid = parse_file(file);
    billion_cycles(grid).grid_load()

}

fn billion_cycles(grid: Grid) -> Grid {
    let number_of_cycles = 1_000_000_000;

    let mut memory: HashMap<String, usize> = HashMap::new();

    let mut current_grid = grid;
    let mut cycles = 0;
    memory.insert(current_grid.string(), cycles);

    loop {
        current_grid = current_grid.cycle();
        cycles += 1;

        if cycles == number_of_cycles {
            return current_grid;
        }

        if memory.contains_key(&current_grid.string()) {
            let previous_cycles = *memory.get(&current_grid.string()).unwrap();
            let loop_length = cycles - previous_cycles;

            let remaining_cycles = number_of_cycles - cycles;
            let mod_cycles = remaining_cycles % loop_length;

            let billionth_grid_string = memory
                .into_iter()
                .find(|(_, index)| *index == mod_cycles + previous_cycles)
                .unwrap()
                .0;
            return parse_string(billionth_grid_string);
        }

        memory.insert(current_grid.string(), cycles);
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    Static,
    Rolling,
}
impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Empty,
            '#' => Tile::Static,
            'O' => Tile::Rolling,
            _ => panic!("Invalid character in input file"),
        }
    }
}
struct Grid {
    tiles: Vec<Vec<Tile>>,
    transposed: bool,
    reversed: bool,
}
impl Grid {
    fn transpose(mut self) -> Self {
        self.transposed = !self.transposed;
        self
    }
    fn reverse(mut self) -> Self {
        self.reversed = !self.reversed;
        self
    }
    fn width(&self) -> usize {
        if self.transposed {
            self.tiles.len()
        } else {
            self.tiles[0].len()
        }
    }
    fn height(&self) -> usize {
        if self.transposed {
            self.tiles[0].len()
        } else {
            self.tiles.len()
        }
    }

    fn roll_row(self, row: usize) -> Self {
        let width = self.width();

        (0..width).fold(self, |grid, pass| grid.roll_row_pass(row, pass))
    }
    fn roll_row_pass(mut self, row: usize, pass: usize) -> Self {
        let width = self.width();

        for j in 0..width - pass - 1 {
            let tile_a = self[(row, j)];
            let tile_b = self[(row, j + 1)];
            if sort_tiles(&tile_a, &tile_b) == Ordering::Greater {
                // swap the value of tiles a and b
                self[(row, j)] = tile_b;
                self[(row, j + 1)] = tile_a;
            }
        }

        self
    }

    fn roll_west(self) -> Self {
        (0..self.height()).fold(self, |grid, row| {
            grid.roll_row(row)
        })
    }
    fn roll_east(self) -> Self {
        self.reverse().roll_west().reverse()
    }
    fn roll_north(self) -> Self {
        self.transpose().roll_west().transpose()
    }
    fn roll_south(self) -> Self {
        self.reverse().transpose().roll_west().transpose().reverse()
    }

    fn cycle(self) -> Self {
        self.roll_north().roll_west().roll_south().roll_east()
    }

    fn grid_load(&mut self) -> u128 {
        (0..self.height())
            .map(|row| self.count_rolling(row) * (self.width() - row))
            .sum::<usize>() as u128
    }
    fn count_rolling(&self, row: usize) -> usize {
        (0..self.width())
            .filter(|col| self[(row, *col)] == Tile::Rolling)
            .count()
    }
    fn string(&self) -> String{
        // Convert self to a string by iterating over height and width
        // and appending the character representation of each tile
        let mut string = String::new();

        for j in 0..self.height() {
            for i in 0..self.width() {
                string.push(match self[(j,i)] {
                    Tile::Empty => '.',
                    Tile::Static => '#',
                    Tile::Rolling => 'O',
                });
            }
            string.push('\n');
        }

        string
    }
}
type Coordinate = (usize, usize);
impl Index<Coordinate> for Grid {
    type Output = Tile;

    fn index(&self, index: Coordinate) -> &Self::Output {
        let (j,i) = index;

        let (j,i) = match (self.transposed, self.reversed){
            (false, false) => (j,i),
            (false, true) => (j, self.width() - i - 1),
            (true, false) => (i,j),
            (true, true) => (self.width() - i - 1, j),
        };

        &self.tiles[j][i]
    }
}
impl IndexMut<Coordinate> for Grid {
    fn index_mut(&mut self, index: Coordinate) -> &mut Self::Output {
        let (j,i) = index;

        let (j,i) = match (self.transposed, self.reversed){
            (false, false) => (j,i),
            (false, true) => (j, self.width() - i - 1),
            (true, false) => (i,j),
            (true, true) => (self.width() - i - 1, j),
        };

        &mut self.tiles[j][i]
    }
}
fn parse_file(file: &str) -> Grid {
    let contents = std::fs::read_to_string(file).expect("Something went wrong reading the file");

    parse_string(contents)
}
fn parse_string(string: String) -> Grid{
    let tiles = string.lines().map(parse_line).collect::<Vec<Vec<Tile>>>();
    let transposed = false;
    let reversed = false;

    Grid {
        tiles,
        transposed,
        reversed
    }
}
fn parse_line(line: &str) -> Vec<Tile> {
    line.chars().map(Tile::from).collect::<Vec<Tile>>()
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

        grid.tiles.iter().for_each(|row| {
            println!("{:?}", row);
        });
    }

    #[test]
    fn test_roll_tiles() {
        let grid = parse_file("resources/2023/day14/test_input").roll_north();

        grid.tiles.iter().for_each(|row| {
            println!("{:?}", row);
        });
        println!();

        let grid = parse_file("resources/2023/day14/test_input").roll_south();

        grid.tiles.iter().for_each(|row| {
            println!("{:?}", row);
        });
        println!();

        let grid = parse_file("resources/2023/day14/test_input").roll_west();

        grid.tiles.iter().for_each(|row| {
            println!("{:?}", row);
        });
        println!();

        let grid = parse_file("resources/2023/day14/test_input").roll_east();

        grid.tiles.iter().for_each(|row| {
            println!("{:?}", row);
        });
        println!();
    }

    #[test]
    fn test_total_load() {
        assert_eq!(total_load("resources/2023/day14/test_input"), 136);
    }

    #[test]
    fn test_cycle() {
        let grid = parse_file("resources/2023/day14/test_input").cycle();

        grid.tiles.iter().for_each(|row| {
            println!("{:?}", row);
        });
    }

        #[test]
        fn test_billion_cycles() {
            assert_eq!(billion_cycles_load("resources/2023/day14/test_input"), 64);
        }
}
