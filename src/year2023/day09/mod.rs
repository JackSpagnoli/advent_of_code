use std::fs;

pub mod task1 {
    use super::sum_next_terms;

    pub fn ans() -> i128 {
        sum_next_terms("resources/2023/day09/input")
    }
}

pub mod task2 {
    use super::sum_previous_terms;

    pub fn ans() -> i128 {
        sum_previous_terms("resources/2023/day09/input")
    }
}

fn sum_next_terms(file: &str) -> i128 {
    parse_file(file).into_iter().map(predict_next).sum()
}

fn sum_previous_terms(file: &str) -> i128 {
    parse_file(file)
        .into_iter()
        .map(reverse_vec)
        .map(predict_next)
        .sum()
}

fn reverse_vec(mut nums: Vec<i128>) -> Vec<i128> {
    nums.reverse();
    nums
}

fn parse_file(file: &str) -> Vec<Vec<i128>> {
    let contents = fs::read_to_string(file).expect("Error reading the file");

    contents
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|n| n.parse::<i128>().unwrap())
                .collect()
        })
        .collect()
}

fn predict_next(nums: Vec<i128>) -> i128 {
    let diffs: Vec<i128> = nums.windows(2).map(|w| w[1] - w[0]).collect();

    if diffs.iter().all(|n| *n == 0) {
        return nums[0];
    }

    let last = *nums.last().unwrap();
    let next_diff = predict_next(diffs);

    last + next_diff
}

#[cfg(test)]
mod tests {
    use super::parse_file;

    #[test]
    fn test_predict_next() {
        let sequences: Vec<Vec<i128>> = parse_file("resources/2023/day09/test_input");

        let expected_next = vec![18, 28, 68];

        for (seq, expected) in sequences.into_iter().zip(expected_next) {
            let next = super::predict_next(seq);
            assert_eq!(next, expected);
        }
    }

    #[test]
    fn test_sum_next_terms() {
        let sum = super::sum_next_terms("resources/2023/day09/test_input");
        assert_eq!(sum, 114);
    }
}
