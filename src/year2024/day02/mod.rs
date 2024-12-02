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
    let mut deltas = report.windows(2).map(|pair| {
        let a = pair[0];
        let b = pair[1];
        (a.cmp(&b), a.abs_diff(b))
    });

    let acceptable_range = 1..=3;

    let first = deltas.next().unwrap();

    if !acceptable_range.contains(&first.1) {
        return false;
    }
    let order = first.0;

    deltas.all(|(cmp, diff)| {
        if cmp != order {
            return false;
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
