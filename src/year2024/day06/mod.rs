use std::collections::HashSet;

pub mod task1 {
    use super::count_distinct_points;

    pub fn ans() -> u128 {
        count_distinct_points("resources/2024/day06/input.txt")
    }
}

pub mod task2 {
    use super::find_loops;

    pub fn ans() -> u128 {
        find_loops("resources/2024/day06/input.txt")
    }
}

fn count_distinct_points(file: &str) -> u128 {
    let path: Path = file.to_string().into();

    let agent_inital = (path.agent.x, path.agent.y);

    let mut visited = path.collect::<HashSet<_>>();
    visited.insert(agent_inital);

    visited.len() as u128
}

fn find_loops(file: &str) -> u128 {
    let initial_path = Path::from(file.to_string());

    let new_obstacles = (0..initial_path.map_size.0)
        .flat_map(|x| (0..initial_path.map_size.1).map(move |y| (x, y)))
        .filter(|(x, y)| !initial_path.obstacles.contains(&(*x, *y)))
        .collect::<Vec<_>>();

    new_obstacles
        .into_iter()
        .filter(|(x, y)| {
            let mut path: Path = initial_path.clone();
            path.obstacles.insert((*x, *y));

            while let Some(_) = path.next() {}

            path.in_loop
        })
        .count() as u128
}

#[derive(Clone, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Agent {
    x: usize,
    y: usize,
    direction: Direction,
}

#[derive(Clone)]
struct Path {
    agent: Agent,
    obstacles: HashSet<(usize, usize)>,
    map_size: (usize, usize),
    visited: HashSet<Agent>,
    in_loop: bool,
}

type FilePath = String;
impl From<FilePath> for Path {
    fn from(file_path: FilePath) -> Self {
        let content = std::fs::read_to_string(file_path).unwrap();

        let map_width = content.clone().lines().next().unwrap().len();
        let map_height = content.clone().lines().count();
        let map_size = (map_width, map_height);

        let mut obstacles = HashSet::new();
        let mut agent = None;

        for (y, line) in content.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        obstacles.insert((x, y));
                    }
                    '^' => {
                        agent = Some(Agent {
                            x,
                            y,
                            direction: Direction::Up,
                        });
                    }
                    _ => {}
                }
            }
        }

        let agent = agent.unwrap();

        let mut visited = HashSet::new();
        visited.insert(agent.clone());

        Self {
            agent,
            obstacles,
            map_size,
            visited,
            in_loop: false,
        }
    }
}

impl Iterator for Path {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let agent = &mut self.agent;

        let (next_x, next_y): (isize, isize) = match agent.direction {
            Direction::Up => (agent.x as isize, agent.y as isize - 1),
            Direction::Down => (agent.x as isize, agent.y as isize + 1),
            Direction::Left => (agent.x as isize - 1, agent.y as isize),
            Direction::Right => (agent.x as isize + 1, agent.y as isize),
        };

        if next_x < 0
            || next_x >= self.map_size.0 as isize
            || next_y < 0
            || next_y >= self.map_size.1 as isize
        {
            return None;
        }

        if self.obstacles.contains(&(next_x as usize, next_y as usize)) {
            self.agent.direction = match agent.direction {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            };
            return self.next();
        }

        agent.x = next_x as usize;
        agent.y = next_y as usize;

        if self.visited.contains(agent) {
            self.in_loop = true;
            return None;
        }

        self.visited.insert(agent.clone());

        Some((agent.x, agent.y))
    }
}

#[cfg(test)]
mod tests {
    use super::{count_distinct_points, find_loops};

    #[test]
    fn test_distinct_points() {
        let file = "resources/2024/day06/test_input.txt";
        assert_eq!(count_distinct_points(file), 41);
    }

    #[test]
    fn test_find_loops() {
        let file = "resources/2024/day06/test_input.txt";
        assert_eq!(find_loops(file), 6);
    }
}
