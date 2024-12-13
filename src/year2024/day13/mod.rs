use regex::Regex;

pub mod task1 {
    use super::fewest_combinations;

    pub fn ans() -> u128 {
        fewest_combinations("resources/2024/day13/input.txt")
    }
}

pub mod task2 {
    pub fn ans() -> u128 {
        0
    }
}

fn fewest_combinations(file: &str) -> u128 {
    let content = std::fs::read_to_string(file).unwrap();
    let games = content.split("\n\n").map(Game::from).collect::<Vec<Game>>();

    games.into_iter().filter_map(Game::solve).sum()
}

struct Game {
    a_x: isize,
    a_y: isize,
    b_x: isize,
    b_y: isize,
    p_x: isize,
    p_y: isize,
}

impl From<&str> for Game {
    fn from(s: &str) -> Self {
        let regex = Regex::new(r"Button A: X\+(?<a_x>\d+), Y\+(?<a_y>\d+)\nButton B: X\+(?<b_x>\d+), Y\+(?<b_y>\d+)\nPrize: X=(?<p_x>\d+), Y=(?<p_y>\d+)").unwrap();

        let captures = regex.captures(s).unwrap();

        Game {
            a_x: captures.name("a_x").unwrap().as_str().parse().unwrap(),
            a_y: captures.name("a_y").unwrap().as_str().parse().unwrap(),
            b_x: captures.name("b_x").unwrap().as_str().parse().unwrap(),
            b_y: captures.name("b_y").unwrap().as_str().parse().unwrap(),
            p_x: captures.name("p_x").unwrap().as_str().parse().unwrap(),
            p_y: captures.name("p_y").unwrap().as_str().parse().unwrap(),
        }
    }
}

impl Game {
    pub fn solve(mut self) -> Option<u128> {
        // From P, subtract A until it is a multiple of B
        let mut a: u128 = 0;
        loop {
            if self.p_x < 0 || self.p_y < 0 {
                return None;
            }

            if is_multiple_of((self.p_x, self.p_y), (self.b_x, self.b_y)) {
                break;
            }

            self.p_x -= self.a_x;
            self.p_y -= self.a_y;
            a += 1;
        }

        let b: u128 = (self.p_x / self.b_x) as u128;

        Some(3 * a + b)
    }
}

fn is_multiple_of(a: (isize, isize), b: (isize, isize)) -> bool {
    let (a_x, a_y) = a;
    let (b_x, b_y) = b;

    a_x % b_x == 0 && a_y % b_y == 0 && a_x / b_x == a_y / b_y
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_is_multiple_of() {
        assert_eq!(super::is_multiple_of((3, 3), (1, 1)), true);
        assert_eq!(super::is_multiple_of((3, 3), (1, 2)), false);
        assert_eq!(super::is_multiple_of((3, 3), (2, 1)), false);
        assert_eq!(super::is_multiple_of((3, 3), (2, 2)), false);
        assert_eq!(super::is_multiple_of((3, 3), (3, 1)), false);
        assert_eq!(super::is_multiple_of((3, 3), (1, 3)), false);
    }

    #[test]
    fn test_fewest_combinations() {
        assert_eq!(
            super::fewest_combinations("resources/2024/day13/test_input.txt"),
            480
        );
    }
}
