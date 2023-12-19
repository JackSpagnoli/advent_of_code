use std::{
    cmp::{max, min},
    collections::HashMap,
};

use itertools::Itertools;
use std::ops::RangeInclusive;

pub mod task1 {
    use super::accepted_parts;

    pub fn ans() -> u128 {
        accepted_parts("resources/2023/day19/input")
    }
}

pub mod task2 {
    use super::accepted_part_ranges;

    pub fn ans() -> u128 {
        accepted_part_ranges("resources/2023/day19/input")
    }
}

fn accepted_part_ranges(file: &str) -> u128 {
    let (workflows, _) = parse_file(file);

    let start_ranges: [RatingValueRange; 4] = [(1..=4000), (1..=4000), (1..=4000), (1..=4000)];
    let mut part_ranges = vec![PartRange {
        ranges: start_ranges,
        next_workflow: "in".to_string(),
    }];

    let mut accepted_ranges = vec![];

    while let Some(part_range) = part_ranges.pop() {
        let workflow = workflows.get(&part_range.next_workflow).unwrap();

        let mut ranges = part_range.ranges;

        for step in workflow {
            if let Step::Else(next_workflow) = step {
                if next_workflow == "R" {
                    break;
                } else if next_workflow == "A" {
                    accepted_ranges.push(ranges);
                    break;
                } else {
                    part_ranges.push(PartRange {
                        ranges: ranges.clone(),
                        next_workflow: next_workflow.to_string(),
                    });
                    continue;
                }
            } else if let Step::Conditonal(operation, next_workflow) = step {
                let mut branch_ranges = ranges.clone();

                match operation {
                    Operation::GreaterThan(rating, value) => {
                        let branch_lower_bound = max(*value + 1, *ranges[*rating].start());
                        let branch_upper_bound = *ranges[*rating].end();

                        let main_lower_bound = *ranges[*rating].start();
                        let main_upper_bound = min(*value, *ranges[*rating].end());

                        branch_ranges[*rating] = branch_lower_bound..=branch_upper_bound;
                        ranges[*rating] = main_lower_bound..=main_upper_bound;

                        if branch_ranges[*rating].start() <= branch_ranges[*rating].end() {
                            if next_workflow == "A" {
                                accepted_ranges.push(branch_ranges);
                            } else if next_workflow != "R" {
                                part_ranges.push(PartRange {
                                    ranges: branch_ranges,
                                    next_workflow: next_workflow.to_string(),
                                });
                            }
                        }

                        if ranges[*rating].start() > ranges[*rating].end() {
                            break;
                        }
                    }
                    Operation::LessThan(rating, value) => {
                        let branch_lower_bound = *ranges[*rating].start();
                        let branch_upper_bound = min(*value - 1, *ranges[*rating].end());

                        let main_lower_bound = max(*value, *ranges[*rating].start());
                        let main_upper_bound = *ranges[*rating].end();

                        branch_ranges[*rating] = branch_lower_bound..=branch_upper_bound;
                        ranges[*rating] = main_lower_bound..=main_upper_bound;

                        if branch_ranges[*rating].start() <= branch_ranges[*rating].end() {
                            if next_workflow == "A" {
                                accepted_ranges.push(branch_ranges);
                            } else if next_workflow != "R" {
                                part_ranges.push(PartRange {
                                    ranges: branch_ranges,
                                    next_workflow: next_workflow.to_string(),
                                });
                            }
                        }

                        if ranges[*rating].start() > ranges[*rating].end() {
                            break;
                        }
                    }
                }
            } else {
                panic!("Bruh");
            }
        }
    }
    
    reduce_ranges(accepted_ranges).into_iter().map(|range| {
        range.into_iter().map(|range| {
            (range.end() - range.start() + 1) as u128
        }).product::<u128>()
    }).sum::<u128>()
}

fn reduce_ranges(mut ranges: Vec<PartRanges>) -> Vec<PartRanges> {
    // Removes nested ranges, then performs a plane sweep over each axis to break up overlapping ranges
    // then recurse on the new set of RangeInclusive
    ranges = remove_duplicate_ranges(ranges);
    ranges = remove_nested_ranges(ranges);

    ranges.sort_by(sort_ranges);

    // Find a pair of ranges that overlap
    for j in 0..ranges.len() - 1 {
        for i in j + 1..ranges.len() {
            if let Some(index) = do_cuboids_overlap(&ranges[j], &ranges[i]) {
                let mut range_a = ranges.remove(j);
                let mut range_b = ranges.remove(i - 1);

                if range_a[index].start() > range_b[index].start() {
                    std::mem::swap(&mut range_a, &mut range_b);
                }

                if range_b[index].end() > range_a[index].end() {
                    // a and b partially overlap in this axis
                    let new_range_start = *range_b[index].start();
                    let new_range_end = *range_a[index].end();

                    range_a[index] = *range_a[index].start()..=new_range_start - 1;
                    range_b[index] = new_range_end + 1..=*range_b[index].end();

                    let new_range = new_range_start..=new_range_end;

                    let mut a_right = range_a.clone();
                    let mut b_left = range_b.clone();

                    a_right[index] = new_range.clone();
                    b_left[index] = new_range.clone();

                    ranges.push(range_a);
                    ranges.push(a_right);
                    ranges.push(b_left);
                    ranges.push(range_b);
                } else {
                    // b is contained within a in this axis

                    let new_range_start = *range_b[index].start();
                    let new_range_end = *range_b[index].end();

                    let mut a_right = range_a.clone();
                    a_right[index] = new_range_end + 1..=*range_a[index].end();
                    let mut a_middle = range_a.clone();
                    a_middle[index] = new_range_start..=new_range_end;
                    
                    range_a[index] = *range_a[index].start()..=new_range_start - 1;

                    ranges.push(range_a);
                    ranges.push(a_middle);
                    ranges.push(a_right);
                    ranges.push(range_b);
                }
                return reduce_ranges(ranges);
            }
        }
    }

    ranges
}

fn do_cuboids_overlap(a: &PartRanges, b: &PartRanges) -> Option<RatingIndex> {
    if !(0..4).all(|i| do_ranges_overlap(&a[i], &b[i])) {
        return None;
    }
    (0..4)
        .filter(|i| a[*i] != b[*i])
        .find(|i| do_ranges_overlap(&a[*i], &b[*i]))
}

fn do_ranges_overlap(a: &RangeInclusive<RatingValue>, b: &RangeInclusive<RatingValue>) -> bool {
    a.start() <= b.end() && b.start() <= a.end()
}

fn remove_duplicate_ranges(ranges: Vec<PartRanges>) -> Vec<PartRanges> {
    let mut new_ranges = vec![];

    for range in ranges {
        if !new_ranges.contains(&range) {
            new_ranges.push(range);
        }
    }

    new_ranges
}

fn remove_nested_ranges(ranges: Vec<PartRanges>) -> Vec<PartRanges> {
    if ranges.len() == 1 {
        return ranges;
    }

    let mut new_ranges = vec![];

    for i in 0..ranges.len() {
        let mut are_nested = false;
        for j in 0..ranges.len() {
            if i == j {
                continue;
            }
            if is_nested(&ranges[i], &ranges[j]) {
                are_nested = true;
                break;
            }
        }
        if !are_nested {
            new_ranges.push(ranges[i].clone());
        }
    }

    new_ranges
}

fn is_nested(a: &PartRanges, b: &PartRanges) -> bool {
    // Is a nested within b
    for range_index in 0..4 {
        if !is_nested_range(&a[range_index], &b[range_index]) {
            return false;
        }
    }
    true
}

fn is_nested_range(a: &RangeInclusive<RatingValue>, b: &RangeInclusive<RatingValue>) -> bool {
    // is a nested within b
    a.start() >= b.start() && a.end() <= b.end()
}

fn sort_ranges(a: &PartRanges, b: &PartRanges) -> std::cmp::Ordering {
    for range_index in 0..4 {
        let range_ordering = compare_range(&a[range_index], &b[range_index]);
        if range_ordering != std::cmp::Ordering::Equal {
            return range_ordering;
        }
    }
    std::cmp::Ordering::Equal
}

fn compare_range(
    a: &RangeInclusive<RatingValue>,
    b: &RangeInclusive<RatingValue>,
) -> std::cmp::Ordering {
    match a.start().cmp(b.start()) {
        std::cmp::Ordering::Equal => a.end().cmp(b.end()),
        ordering => ordering,
    }
}

fn accepted_parts(file: &str) -> u128 {
    let (workflows, mut parts) = parse_file(file);

    while let Some((part_index, part)) = parts
        .iter()
        .find_position(|part| part.next_workflow != "A" && part.next_workflow != "R")
    {
        let workflow = workflows.get(&part.next_workflow).unwrap();

        for step in workflow {
            if let Step::Conditonal(operation, next_workflow) = step {
                if let Operation::GreaterThan(rating, value) = operation {
                    if part.ratings[*rating] > *value {
                        parts[part_index].next_workflow = next_workflow.to_string();
                        break;
                    }
                } else if let Operation::LessThan(rating, value) = operation {
                    if part.ratings[*rating] < *value {
                        parts[part_index].next_workflow = next_workflow.to_string();
                        break;
                    }
                }
            } else if let Step::Else(next_workflow) = step {
                parts[part_index].next_workflow = next_workflow.to_string();
                break;
            } else {
                panic!("Bruh");
            }
        }
    }

    parts
        .into_iter()
        .filter(|part| part.next_workflow == "A")
        .map(|part| part.ratings.iter().sum::<usize>() as u128)
        .sum::<u128>()
}

type WorkflowName = String;
type Parts = Vec<Part>;
#[derive(Debug)]
struct Part {
    ratings: [RatingValue; 4 as RatingIndex],
    next_workflow: WorkflowName,
}

type PartRanges = [RangeInclusive<RatingValue>; 4 as RatingIndex];
struct PartRange {
    ranges: PartRanges,
    next_workflow: WorkflowName,
}

type Workflows = HashMap<WorkflowName, Workflow>;
type Workflow = Vec<Step>;

type RatingIndex = usize;
type RatingValue = usize;
type RatingValueRange = RangeInclusive<RatingValue>;
#[derive(PartialEq, Debug)]
enum Operation {
    LessThan(RatingIndex, RatingValue),
    GreaterThan(RatingIndex, RatingValue),
}
#[derive(PartialEq, Debug)]
enum Step {
    Conditonal(Operation, WorkflowName),
    Else(WorkflowName),
}

fn parse_file(file: &str) -> (Workflows, Parts) {
    let contents = std::fs::read_to_string(file).unwrap();

    let [workflow_str, parts_str] = contents.split("\n\n").collect::<Vec<&str>>()[..2]
        .try_into()
        .unwrap();

    (parse_workflows(workflow_str), parse_parts(parts_str))
}

fn parse_workflows(workflow_str: &str) -> Workflows {
    workflow_str.lines().map(parse_workflow_line).collect()
}

fn parse_workflow_line(line: &str) -> (WorkflowName, Workflow) {
    let mut parts = line.split('{');
    let name = parts.next().unwrap().to_string();
    let steps = parts
        .next()
        .unwrap()
        .to_string()
        .trim_end_matches('}')
        .to_string()
        .split(',')
        .map(parse_step)
        .collect();

    (name, steps)
}

fn parse_step(step: &str) -> Step {
    if !step.contains(':') {
        return Step::Else(step.to_string());
    }
    let mut parts = step.split(':');
    let operation = parts.next().unwrap();
    let next_workflow = parts.next().unwrap().to_string();
    if operation.contains('>') {
        let mut parts = operation.split('>');
        let rating_str = parts.next().unwrap();
        let rating = parse_rating_str(rating_str);
        let value = parts.next().unwrap().parse().unwrap();

        return Step::Conditonal(Operation::GreaterThan(rating, value), next_workflow);
    } else if operation.contains('<') {
        let mut parts = operation.split('<');
        let rating_str = parts.next().unwrap();
        let rating = parse_rating_str(rating_str);
        let value = parts.next().unwrap().parse().unwrap();

        return Step::Conditonal(Operation::LessThan(rating, value), next_workflow);
    }
    panic!("Invalid operation: {}", operation);
}

fn parse_rating_str(rating_str: &str) -> RatingIndex {
    match rating_str {
        "x" => 0,
        "m" => 1,
        "a" => 2,
        "s" => 3,
        _ => panic!("Invalid rating: {}", rating_str),
    }
}

fn parse_parts(parts_str: &str) -> Parts {
    parts_str.lines().map(parse_part_line).collect()
}

fn parse_part_line(line: &str) -> Part {
    let ratings: [RatingValue; 4] = line
        .trim_start_matches('{')
        .trim_end_matches('}')
        .split(',')
        .map(parse_rating)
        .collect::<Vec<RatingValue>>()
        .try_into()
        .unwrap();

    let next_workflow = "in".to_string();

    Part {
        ratings,
        next_workflow,
    }
}

fn parse_rating(rating: &str) -> RatingValue {
    rating.split('=').nth(1).unwrap().parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_workflow_line() {
        let line = "px{a<2006:qkq,m>2090:A,rfg}";

        let (name, workflow) = parse_workflow_line(line);

        let expected_name = "px".to_string();
        let expected_steps = vec![
            Step::Conditonal(Operation::LessThan(2, 2006), "qkq".to_string()),
            Step::Conditonal(Operation::GreaterThan(1, 2090), "A".to_string()),
            Step::Else("rfg".to_string()),
        ];

        assert_eq!(name, expected_name);
        assert_eq!(workflow, expected_steps);
    }

    #[test]
    fn test_parse_part_line() {
        let line = "{x=787,m=2655,a=1222,s=2876}";

        let part = parse_part_line(line);

        let expected_ratings = [787, 2655, 1222, 2876];

        assert_eq!(part.ratings, expected_ratings);
    }

    #[test]
    fn test_accepted_parts() {
        let accepted = accepted_parts("resources/2023/day19/test_input");

        assert_eq!(accepted, 19114);
    }

    #[test]
    fn test_is_nested_range() {
        let a = 2..=9;
        let b = 1..=10;

        assert!(is_nested_range(&a, &b));

        let a = 1..=10;
        let b = 2..=11;

        assert!(!is_nested_range(&a, &b));
    }

    #[test]
    fn test_remove_nested_ranges() {
        let a = [1..=10, 1..=10, 1..=10, 1..=10];
        let b = [2..=9, 2..=9, 2..=9, 2..=9];
        let c = [3..=8, 3..=8, 3..=8, 3..=8];
        let d = [4..=7, 4..=7, 4..=7, 4..=7];
        let e = [5..=6, 5..=6, 5..=6, 5..=6];

        let ranges = vec![a.clone(), b, c, d, e];

        let ranges = remove_nested_ranges(ranges);

        let expected_ranges = vec![a];

        assert_eq!(ranges, expected_ranges);
    }

    #[test]
    fn test_reduce_ranges_1d_overlap() {
        let a = [1..=5, 1..=10, 1..=10, 1..=10];
        let b = [4..=10, 1..=10, 1..=10, 1..=10];

        let mut ranges = vec![a, b];
        ranges = reduce_ranges(ranges);

        let expected_range_a = [1..=3, 1..=10, 1..=10, 1..=10];
        let expected_range_b = [4..=5, 1..=10, 1..=10, 1..=10];
        let expected_range_c = [6..=10, 1..=10, 1..=10, 1..=10];

        let expected_ranges = vec![expected_range_a, expected_range_b, expected_range_c];

        assert_eq!(ranges, expected_ranges);
    }

    #[test]
    fn test_reduce_ranges_2d_overlap() {
        let a = [1..=5, 1..=5, 1..=10, 1..=10];
        let b = [3..=10, 3..=10, 1..=10, 1..=10];

        let mut ranges = vec![a, b];
        ranges = reduce_ranges(ranges);

        let expected_range_a = [1..=2, 1..=5, 1..=10, 1..=10];
        let expected_range_b = [3..=5, 1..=2, 1..=10, 1..=10];
        let expected_range_c = [3..=5, 3..=5, 1..=10, 1..=10];
        let expected_range_d = [3..=5, 6..=10, 1..=10, 1..=10];
        let expected_range_e = [6..=10, 3..=10, 1..=10, 1..=10];

        let expected_ranges = vec![
            expected_range_a,
            expected_range_b,
            expected_range_c,
            expected_range_d,
            expected_range_e,
        ];

        assert_eq!(ranges, expected_ranges);
    }

    #[test]
    fn test_reduce_ranges_2d_nest() {
        let a = [1..=10, 1..=5, 1..=10, 1..=10];
        let b = [4..=6, 3..=10, 1..=10, 1..=10];

        let mut ranges = vec![a, b];
        ranges = reduce_ranges(ranges);

        let expected_range_a = [1..=3, 1..=5, 1..=10, 1..=10];
        let expected_range_b = [4..=6, 1..=2, 1..=10, 1..=10];
        let expected_range_c = [4..=6, 3..=5, 1..=10, 1..=10];
        let expected_range_d = [4..=6, 6..=10, 1..=10, 1..=10];
        let expected_range_e = [7..=10, 1..=5, 1..=10, 1..=10];

        let expected_ranges = vec![
            expected_range_a,
            expected_range_b,
            expected_range_c,
            expected_range_d,
            expected_range_e,
        ];

        assert_eq!(ranges, expected_ranges);
    }

    #[test]
    fn test_accepted_ranges() {
        let accepted = accepted_part_ranges("resources/2023/day19/test_input");

        assert_eq!(accepted, 167409079868000u128);
    }
}

