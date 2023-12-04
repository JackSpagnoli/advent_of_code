pub mod task1 {
    use super::total_score;

    pub fn ans() -> u128 {
        total_score("resources/2022/day02/input")
    }
}

pub mod task2 {
    use super::total_updated_score;

    pub fn ans() -> u128 {
        total_updated_score("resources/2022/day02/input")
    }
}

fn total_score(file: &str) -> u128 {
    let outcome_encoding: [[usize; 3]; 3] = [[3, 6, 0], [0, 3, 6], [6, 0, 3]];

    parse_score_with_matrix(file, outcome_encoding, 1)
}

fn total_updated_score(file: &str) -> u128 {
    let player_move_encoding: [[usize; 3]; 3] = [[2, 0, 1], [0, 1, 2], [1, 2, 0]];

    parse_score_with_matrix(file, player_move_encoding, 3)
}

fn parse_score_with_matrix(
    file: &str,
    encoding: [[usize; 3]; 3],
    right_element_factor: usize,
) -> u128 {
    let contents = std::fs::read_to_string(file).expect("Error reading file");
    contents
        .lines()
        .map(|line| line.chars())
        .map(|mut line| {
            (
                line.next().unwrap() as usize - 65,
                line.nth(1).unwrap() as usize - 88,
            )
        })
        .map(|(opponent_move, right_element)| {
            (right_element_factor * right_element) + encoding[opponent_move][right_element] + 1
        })
        .sum::<usize>() as u128
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_score() {
        assert_eq!(total_score("resources/2022/day02/test_input"), 15);
    }

    #[test]
    fn test_total_updated_score() {
        assert_eq!(total_updated_score("resources/2022/day02/test_input"), 12);
    }
}
