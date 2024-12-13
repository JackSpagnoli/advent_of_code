use regex::Regex;

pub mod task1 {
    use super::fewest_combinations;

    pub fn ans() -> u128 {
        fewest_combinations("resources/2024/day13/input.txt", false)
    }
}

pub mod task2 {
    use super::fewest_combinations;

    pub fn ans() -> u128 {
        fewest_combinations("resources/2024/day13/input.txt", true)
    }
}

fn fewest_combinations(file: &str, conversion: bool) -> u128 {
    let content = std::fs::read_to_string(file).unwrap();
    let games = content.split("\n\n").map(Game::from);

    games.filter_map(|game| game.solve(conversion)).sum()
}

struct Game {
    a_x: i128,
    a_y: i128,
    b_x: i128,
    b_y: i128,
    p_x: i128,
    p_y: i128,
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
    pub fn solve(mut self, conversion: bool) -> Option<u128> {
        if conversion {
            self.p_x += 10000000000000;
            self.p_y += 10000000000000;
        }

        let det = self.a_x * self.b_y - self.a_y * self.b_x;
        if det == 0 {
            return None;
        }

        let a_numerator = self.p_x * self.b_y - self.p_y * self.b_x;
        let b_numerator = self.p_y * self.a_x - self.p_x * self.a_y;

        if a_numerator % det != 0 || b_numerator % det != 0 {
            return None;
        }

        let a = a_numerator / det;
        let b = b_numerator / det;

        if a < 0 || b < 0 {
            return None;
        }

        let cost = (3 * a + b) as u128;
        Some(cost)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_fewest_combinations() {
        assert_eq!(
            super::fewest_combinations("resources/2024/day13/test_input.txt", false),
            480
        );
    }
}
