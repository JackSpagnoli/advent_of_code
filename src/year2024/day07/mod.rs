use std::fs;

pub mod task1 {
    use super::sum_possible_equations;

    pub fn ans() -> u128 {
        sum_possible_equations("resources/2024/day07/input.txt", false)
    }
}

pub mod task2 {
    use super::sum_possible_equations;

    pub fn ans() -> u128 {
        sum_possible_equations("resources/2024/day07/input.txt", true)
    }
}

fn sum_possible_equations(file: &str, use_concat: bool) -> u128 {
    let contents = fs::read_to_string(file).unwrap();

    contents
        .lines()
        .map(|line| line_value(line, use_concat))
        .sum::<u128>()
}

fn line_value(line: &str, use_concat: bool) -> u128 {
    let (total, operands) = line.split_once(": ").unwrap();

    let total = total.parse::<u128>().unwrap();
    let operands = operands
        .split(" ")
        .map(|x| x.parse::<u128>().unwrap())
        .collect::<Vec<u128>>();

    if is_valid_equation(total, operands, use_concat) {
        return total;
    } else {
        return 0;
    }
}

fn is_valid_equation(total: u128, operands: Vec<u128>, use_concat: bool) -> bool {
    if operands.len() == 2 {
        let is_addition = operands[0] + operands[1] == total;
        let is_multiplication = operands[0] * operands[1] == total;
        let is_concat = format!("{}{}", operands[0], operands[1])
            .parse::<u128>()
            .unwrap()
            == total;

        return is_addition || is_multiplication || (use_concat && is_concat);
    }

    if operands[0] >= total {
        return false;
    }

    let a = operands[0];
    let b = operands[1];
    let c = operands[2..].to_vec();

    let sum_operand = a + b;
    let mul_operand = a * b;
    let concat_operand = format!("{}{}", a, b).parse::<u128>().unwrap();

    let sum_operands = vec![vec![sum_operand], c.clone()].concat();
    let mul_operands = vec![vec![mul_operand], c.clone()].concat();
    let concat_operands = vec![vec![concat_operand], c].concat();

    is_valid_equation(total, sum_operands, use_concat)
        || is_valid_equation(total, mul_operands, use_concat)
        || (use_concat && is_valid_equation(total, concat_operands, use_concat))
}

#[cfg(test)]
mod tests {
    use super::sum_possible_equations;

    #[test]
    fn test_sum_possible_equations() {
        assert_eq!(
            sum_possible_equations("resources/2024/day07/test_input.txt", false),
            3749
        );
    }

    #[test]
    fn test_sum_possible_equations_with_concat() {
        assert_eq!(
            sum_possible_equations("resources/2024/day07/test_input.txt", true),
            11387
        );
    }
}
