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

    if is_valid_equation(total, &operands, use_concat) {
        return total;
    } else {
        return 0;
    }
}

fn is_valid_equation(total: u128, operands: &[u128], use_concat: bool) -> bool {
    if operands.len() == 2 {
        let is_addition = operands[0] + operands[1] == total;
        let is_multiplication = operands[0] * operands[1] == total;
        let is_concat = format!("{}{}", operands[0], operands[1])
            .parse::<u128>()
            .unwrap()
            == total;

        return is_addition || is_multiplication || (use_concat && is_concat);
    }

    let a = operands[0];
    let b = operands[1];
    let c = &operands[2..];

    let mut operands = Vec::with_capacity(3);

    if a + b <= total {
        operands.push(a + b);
    }
    if a * b <= total {
        operands.push(a * b);
    }

    if use_concat {
        let concat = format!("{}{}", a, b).parse::<u128>().unwrap();
        if concat <= total {
            operands.push(concat);
        }
    }

    for operand in operands {
        let mut new_operands = vec![operand];
        new_operands.extend_from_slice(c);
        if is_valid_equation(total, &new_operands, use_concat) {
            return true;
        }
    }
    return false;
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
