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
        define_problem!(year2022::day08::task1, 1672),
        define_problem!(year2022::day08::task2, 327180),
        define_problem!(year2022::day09::task1, 6464),
        define_problem!(year2022::day09::task2, 2604),
        define_problem!(year2022::day10::task1, 14420),
        define_string_problem!(year2022::day10::task2, "RGLRBZAU"),
        define_problem!(year2022::day11::task1, 76728),
        define_problem!(year2022::day11::task2, 21553910156u128),
        define_problem!(year2022::day12::task1, 352),
        define_problem!(year2022::day12::task2, 345),
        define_problem!(year2022::day13::task1, 5003),
        define_problem!(year2022::day13::task2, 20280),
        define_problem!(year2022::day14::task1, 1133),
        define_problem!(year2022::day14::task2, 27566),
        define_problem!(year2022::day15::task1, 5716881),
        define_problem!(year2022::day15::task2, 10852583132904u128),
        // define_problem!(year2022::day16::task1, 1707),
        // define_problem!(year2022::day16::task2, 0),
        define_problem!(year2022::day18::task1, 4310),
        define_problem!(year2022::day18::task2, 2466),
        // 2023
        define_problem!(year2023::day01::task1, 56506),
        define_problem!(year2023::day01::task2, 56017),
        define_problem!(year2023::day02::task1, 2237),
        define_problem!(year2023::day02::task2, 66681),
        define_problem!(year2023::day03::task1, 553079),
        define_problem!(year2023::day03::task2, 84363105),
        define_problem!(year2023::day04::task1, 21088),
        define_problem!(year2023::day04::task2, 6874754),
        define_problem!(year2023::day05::task1, 324724204),
        define_problem!(year2023::day05::task2, 104070862),
        define_problem!(year2023::day06::task1, 1108800),
        define_problem!(year2023::day06::task2, 36919753),
        define_problem!(year2023::day07::task1, 248422077),
        define_problem!(year2023::day07::task2, 249817836),
        define_problem!(year2023::day08::task1, 13019),
        define_problem!(year2023::day08::task2, 13524038372771u128),
        define_problem!(year2023::day09::task1, 2005352194),
        define_problem!(year2023::day09::task2, 1077),
        define_problem!(year2023::day10::task1, 6956),
        define_problem!(year2023::day10::task2, 455),
        define_problem!(year2023::day11::task1, 9974721),
        define_problem!(year2023::day11::task2, 702770569197u128),
        define_problem!(year2023::day12::task1, 7344),
        // define_problem!(year2023::day12::task2, 1088006519007u128),
        define_problem!(year2023::day13::task1, 34918),
        define_problem!(year2023::day13::task2, 33054),
        define_problem!(year2023::day14::task1, 113456),
        define_problem!(year2023::day14::task2, 118747),
        define_problem!(year2023::day15::task1, 513643),
        define_problem!(year2023::day15::task2, 265345),
        define_problem!(year2023::day16::task1, 6978),
        define_problem!(year2023::day16::task2, 7315),
        define_problem!(year2023::day17::task1, 698),
        define_problem!(year2023::day17::task2, 825),
        define_problem!(year2023::day18::task1, 53844),
        define_problem!(year2023::day18::task2, 42708339569950u128),
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
        (ProblemAnswer::Signed(actual), ProblemAnswer::Signed(expected)) => {
            assert_eq!(actual, *expected)
        }
        (ProblemAnswer::Unsigned(actual), ProblemAnswer::Unsigned(expected)) => {
            assert_eq!(actual, *expected)
        }
        (ProblemAnswer::String(actual), ProblemAnswer::String(expected)) => {
            assert_eq!(actual, *expected)
        }
        _ => panic!("Expected return type does not match actual return type"),
    }
}
