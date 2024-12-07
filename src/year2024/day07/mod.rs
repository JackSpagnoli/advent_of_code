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

    if is_valid_equation(total, operands[0], &operands[1..], use_concat) {
        return total;
    } else {
        return 0;
    }
}

fn is_valid_equation(total: u128, a: u128, operands: &[u128], use_concat: bool) -> bool {
    let b = operands[0];

    if operands.len() == 1 {
        let is_addition = a + b == total;
        let is_multiplication = a * b == total;
        let is_concat = format!("{}{}", a, b).parse::<u128>().unwrap() == total;

        return is_addition || is_multiplication || (use_concat && is_concat);
    }

    let sum_operand = a + b;
    let sum_possible = sum_operand <= total;

    let mul_operand = a * b;
    let mul_possible = mul_operand <= total;

    let concat_operand = format!("{}{}", a, b).parse::<u128>().unwrap();
    let concat_possible = concat_operand <= total;

    (sum_possible && is_valid_equation(total, sum_operand, &operands[1..], use_concat))
        || (mul_possible && is_valid_equation(total, mul_operand, &operands[1..], use_concat))
        || (use_concat
            && (concat_possible
                && is_valid_equation(total, concat_operand, &operands[1..], use_concat)))
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

    #[bench]
    fn bench_sum_possible_equations(b: &mut crate::Bencher) {
        b.iter(|| test_sum_possible_equations());
    }

    #[test]
    fn test_sum_possible_equations_with_concat() {
        assert_eq!(
            sum_possible_equations("resources/2024/day07/test_input.txt", true),
            11387
        );
    }

    #[bench]
    fn bench_sum_possible_equations_with_concat(b: &mut crate::Bencher) {
        b.iter(|| test_sum_possible_equations_with_concat());
    }
}
