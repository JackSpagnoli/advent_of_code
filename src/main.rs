use std::time::Instant;

use aoc2023::*;

fn main() {
    let problems: Vec<(&dyn Fn() -> Whole128, Whole128, &str)> = vec![
        define_problem!(day01::task1, 56506),
        define_problem!(day01::task2, 56017),
    ];

    problems.iter().for_each(check_problem)
}

fn check_problem(problem: &(&dyn Fn() -> Whole128, Whole128, &str)) {
    let problem_function = problem.0;
    let expected = &problem.1;
    let problem_name = problem.2;

    let now = Instant::now();

    let actual_result = problem_function();

    let duration = now.elapsed().as_secs_f32();
    println!("Problem {problem_name} took {duration} seconds");

    match (actual_result, expected) {
        (Whole128::Signed(actual), Whole128::Signed(expected)) => assert_eq!(actual, *expected),
        (Whole128::Unsigned(actual), Whole128::Unsigned(expected)) => assert_eq!(actual, *expected),
        _ => panic!("Expected return type does not match actual return type"),
    }
}
