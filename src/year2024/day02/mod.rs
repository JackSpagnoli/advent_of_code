use std::fs;

pub mod task1 {
    use super::safe_report_count;

    pub fn ans() -> u128 {
        safe_report_count("resources/2024/day02/input.txt")
    }
}

fn safe_report_count(file: &str) -> u128 {
    fs::read_to_string(file)
        .expect("Error reading file")
        .lines()
        .map(parse_line)
        .filter(is_safe_report)
        .count() as u128
}


fn parse_line(line: &str) -> Vec<i128> {
    line.split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect()
}

fn is_safe_report(report: &Vec<i128>) -> bool {
    let valid_range = 1..=3;

    let first_diff = report[0].abs_diff(report[1]);
    if !valid_range.contains(&first_diff) {
        return false;
    }

    report.windows(3).all(|values| {
        let a = values[0];
        let b = values[1];
        let c = values[2];

        let left_cmp = a.cmp(&b);
        let right_cmp = b.cmp(&c);

        let monotonic = left_cmp == right_cmp;

        let right_diff = b.abs_diff(c);

        let valid_diff = valid_range.contains(&right_diff);

        monotonic && valid_diff
    })
}

        }

        acceptable_range.contains(&diff)
    })
}


#[cfg(test)]
mod tests {
    use crate::year2024::day02::{safe_report_count, safe_report_count_with_removal};

    #[test]
    fn test_task_1() {
        assert_eq!(safe_report_count("resources/2024/day02/test.txt"), 2);
    }

}
