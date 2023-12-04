use std::time::Instant;

use advent_of_code::*;

fn main() {
    let problems: Vec<(&dyn Fn() -> ProblemAnswer, ProblemAnswer, &str)> = vec![
        // 2022
        define_problem!(year2022::day01::task1, 74711),
        define_problem!(year2022::day01::task2, 209481),
        define_problem!(year2022::day02::task1, 13682),
        define_problem!(year2022::day02::task2, 12881),
        define_problem!(year2022::day03::task1, 7872),
        define_problem!(year2022::day03::task2, 2497),
        define_problem!(year2022::day04::task1, 569),
        define_problem!(year2022::day04::task2, 936),
        define_string_problem!(year2022::day05::task1, "FWSHSPJWM"),
        define_string_problem!(year2022::day05::task2, "PWPWHGFZS"),
        define_problem!(year2022::day06::task1, 1210),
        define_problem!(year2022::day06::task2, 3476),
        define_problem!(year2022::day07::task1, 1477771),
        define_problem!(year2022::day07::task2, 3579501),
        // 2023
        define_problem!(year2023::day01::task1, 56506),
        define_problem!(year2023::day01::task2, 56017),
        define_problem!(year2023::day02::task1, 2237),
        define_problem!(year2023::day02::task2, 66681),
        define_problem!(year2023::day03::task1, 553079),
        define_problem!(year2023::day03::task2, 84363105),
        define_problem!(year2023::day04::task1, 21088),
        define_problem!(year2023::day04::task2, 6874754),
    ];

    problems.iter().for_each(check_problem)
}

fn check_problem(problem: &(&dyn Fn() -> ProblemAnswer, ProblemAnswer, &str)) {
    let problem_function = problem.0;
    let expected = &problem.1;
    let problem_name = problem.2;

    let now = Instant::now();

    let actual_result = problem_function();

    let duration = now.elapsed().as_secs_f32();
    println!("Problem {problem_name} took {duration} seconds");

    match (actual_result, expected) {
        (ProblemAnswer::Signed(actual), ProblemAnswer::Signed(expected)) => assert_eq!(actual, *expected),
        (ProblemAnswer::Unsigned(actual), ProblemAnswer::Unsigned(expected)) => assert_eq!(actual, *expected),
        (ProblemAnswer::String(actual), ProblemAnswer::String(expected)) => assert_eq!(actual, *expected),
        _ => panic!("Expected return type does not match actual return type"),
    }
}
