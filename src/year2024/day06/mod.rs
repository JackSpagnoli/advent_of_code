use std::collections::HashSet;

pub mod task1 {
    use super::count_distinct_points;

    pub fn ans() -> u128 {
        count_distinct_points("resources/2024/day06/input.txt")
    }
}

pub mod task2 {
    pub fn ans() -> u128 {
        0
    }
}

fn count_distinct_points(file: &str) -> u128 {
    let path: Path = file.to_string().into();

    let agent_inital = (path.agent.x, path.agent.y);

    let mut visited = path.collect::<HashSet<_>>();
    visited.insert(agent_inital);

    visited.len() as u128
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Agent {
    x: usize,
    y: usize,
    direction: Direction,
}

struct Path {
    agent: Agent,
    obstacles: HashSet<(usize, usize)>,
    map_size: (usize, usize),
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

        Self {
            agent,
            obstacles,
            map_size,
        }
    }
}

impl Iterator for Path {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let agent = &mut self.agent;

        println!("Agent: {:?}", agent);

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

        Some((agent.x, agent.y))
    }
}

#[cfg(test)]
mod tests {
    use super::count_distinct_points;

    #[test]
    fn test_distinct_points() {
        let file = "resources/2024/day06/test_input.txt";
        assert_eq!(count_distinct_points(file), 41);
    }
}
