pub mod task1 {
    use super::energized_tiles;

    pub fn ans() -> u128 {
        energized_tiles("resources/2023/day16/input")
    }
}

pub mod task2 {
    use super::highest_energy;

    pub fn ans() -> u128 {
        highest_energy("resources/2023/day16/input")
    }
}

type Coordinate = (isize, isize);
#[derive(Debug, Clone)]
struct Beam {
    position: Coordinate,
    direction: Coordinate,
}
impl Beam {
    fn new(position: Coordinate, direction: Coordinate) -> Self {
        Self {
            position,
            direction,
        }
    }

    fn move_forward(mut self) -> Self {
        self.position.0 += self.direction.0;
        self.position.1 += self.direction.1;
        self
    }

    fn turn(mut self, mirror: &Mirror) -> Self {
        // ( 1, 0) -> ( 0,-1)
        // (-1, 0) -> ( 0, 1)
        // ( 0, 1) -> (-1, 0)
        // ( 0,-1) -> ( 1, 0)
        self.direction = match self.direction {
            (1, 0) => (0, -1),
            (-1, 0) => (0, 1),
            (0, 1) => (-1, 0),
            (0, -1) => (1, 0),
            _ => panic!("Invalid direction"),
        };
        if mirror == &Mirror::Negative {
            self.direction.0 *= -1;
            self.direction.1 *= -1;
        }

        self
    }
}
#[derive(PartialEq, Clone, Copy)]
enum Mirror {
    // "/"
    Positive,
    // "\"
    Negative,
}
#[derive(PartialEq, Clone, Copy)]
enum Spliiter {
    // "|"
    Vertical,
    // "-"
    Horizontal,
}
#[derive(Clone)]
struct Grid {
    mirrors: Vec<(Coordinate, Mirror)>,
    splitters: Vec<(Coordinate, Spliiter)>,
    beams: Vec<Beam>,
    energized_grid: Vec<Vec<bool>>,
    rows: usize,
    cols: usize,
    bounces: Vec<(Coordinate, Coordinate)>,
}

impl From<String> for Grid {
    fn from(grid: String) -> Self {
        let rows = grid.lines().count();
        let cols = grid.lines().next().unwrap().chars().count();

        let mut res = Self {
            mirrors: Vec::new(),
            splitters: Vec::new(),
            beams: vec![],
            energized_grid: vec![vec![false; cols]; rows],
            rows,
            cols,
            bounces: Vec::new(),
        };

        grid.lines()
            .enumerate()
            .flat_map(|(j, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(i, c)| (j as isize, i as isize, c))
            })
            .for_each(|(j, i, c)| match c {
                '/' => res.mirrors.push(((j, i), Mirror::Positive)),
                '\\' => res.mirrors.push(((j, i), Mirror::Negative)),
                '|' => res.splitters.push(((j, i), Spliiter::Vertical)),
                '-' => res.splitters.push(((j, i), Spliiter::Horizontal)),
                _ => {}
            });

        res
    }
}

impl Grid {
    fn get_mirror(&self, coordinate: Coordinate) -> Option<Mirror> {
        self.mirrors
            .iter()
            .find(|(c, _)| c == &coordinate)
            .map(|(_, m)| *m)
    }

    fn get_splitter(&self, coordinate: Coordinate) -> Option<Spliiter> {
        self.splitters
            .iter()
            .find(|(c, _)| c == &coordinate)
            .map(|(_, s)| *s)
    }

    fn energize(&mut self, coordinate: Coordinate) {
        self.energized_grid[coordinate.0 as usize][coordinate.1 as usize] = true;
    }

    fn is_out_of_bounds(&self, coordinate: Coordinate) -> bool {
        coordinate.0 < 0
            || coordinate.1 < 0
            || coordinate.0 >= self.rows as isize
            || coordinate.1 >= self.cols as isize
    }

    fn path_beams(mut self, start_pos: Coordinate, start_direction: Coordinate) -> Self {
        self.beams.push(Beam::new(start_pos, start_direction));
        while let Some(mut beam) = self.beams.pop() {
            loop {
                beam = beam.move_forward();
                if self.is_out_of_bounds(beam.position) {
                    break;
                }
                self.energize(beam.position);
                if let Some(mirror) = self.get_mirror(beam.position) {
                    if self.bounces.contains(&(beam.position, beam.direction)) {
                        break;
                    }
                    self.bounces.push((beam.position, beam.direction));
                    beam = beam.turn(&mirror);
                } else if let Some(splitter) = self.get_splitter(beam.position) {
                    if self.bounces.contains(&(beam.position, beam.direction)) {
                        break;
                    }
                    self.bounces.push((beam.position, beam.direction));
                    if splitter == Spliiter::Vertical && beam.direction.1 != 0 {
                        let upward_beam = Beam {
                            position: beam.position,
                            direction: (-1, 0),
                        };
                        let downward_beam = Beam {
                            position: beam.position,
                            direction: (1, 0),
                        };
                        self.beams.push(upward_beam);
                        self.beams.push(downward_beam);
                        break;
                    } else if splitter == Spliiter::Horizontal && beam.direction.0 != 0 {
                        let leftward_beam = Beam {
                            position: beam.position,
                            direction: (0, -1),
                        };
                        let rightward_beam = Beam {
                            position: beam.position,
                            direction: (0, 1),
                        };
                        self.beams.push(leftward_beam);
                        self.beams.push(rightward_beam);
                        break;
                    }
                }
            }
        }

        self
    }

    fn count_energized(self) -> usize {
        self.energized_grid
            .into_iter()
            .map(|row| row.into_iter().filter(|b| *b).count())
            .sum()
    }
}

fn energized_tiles(file: &str) -> u128 {
    let content = std::fs::read_to_string(file).unwrap();
    Grid::from(content)
        .path_beams((0, -1), (0, 1))
        .count_energized() as u128
}

fn highest_energy(file: &str) -> u128 {
    let content = std::fs::read_to_string(file).unwrap();
    let grid = Grid::from(content);
    let left_starts = (0..grid.rows)
        .map(|j| j as isize)
        .map(|j| (j, -1))
        .map(|position| (position, (0, 1)));
    let right_starts = (0..grid.rows)
        .map(|j| j as isize)
        .map(|j| (j, grid.cols as isize))
        .map(|position| (position, (0, -1)));
    let top_starts = (0..grid.cols)
        .map(|i| i as isize)
        .map(|i| (-1, i))
        .map(|position| (position, (1, 0)));
    let bottom_starts = (0..grid.cols)
        .map(|i| i as isize)
        .map(|i| (grid.rows as isize, i))
        .map(|position| (position, (-1, 0)));

    left_starts
        .chain(right_starts)
        .chain(top_starts)
        .chain(bottom_starts)
        .map(|(position, direction)| {
            grid.clone()
                .path_beams(position, direction)
                .count_energized()
        })
        .max()
        .unwrap() as u128
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_energized_tiles() {
        assert_eq!(energized_tiles("resources/2023/day16/test_input"), 46);
    }

    #[test]
    fn test_highest_energy() {
        assert_eq!(highest_energy("resources/2023/day16/test_input"), 51);
    }
    
}
