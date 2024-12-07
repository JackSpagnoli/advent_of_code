/* trunk-ignore(clippy/E0554) */
#![feature(iter_next_chunk, int_roundings, test)]
extern crate test;
pub use test::Bencher;

pub type ProblemDefinition = (
    &'static dyn Fn() -> ProblemAnswer,
    ProblemAnswer,
    &'static str,
);

pub enum ProblemAnswer {
    Signed(i128),
    Unsigned(u128),
    String(String),
}

#[macro_export()]
macro_rules! define_problem {
    ($year:ident::$day:ident::$task:ident, $expected:expr) => {{
        let problem_number = &stringify!($year::$day::$task)[..];
        use $year::$day::$task::ans as ans_func;
        let expected_value = $expected;
        if expected_value < 0 {
            let return_function: &dyn Fn() -> ProblemAnswer =
                &|| ProblemAnswer::Signed(ans_func() as i128);
            (
                return_function,
                ProblemAnswer::Signed(expected_value as i128),
                problem_number,
            )
        } else {
            let return_function: &dyn Fn() -> ProblemAnswer =
                &|| ProblemAnswer::Unsigned(ans_func() as u128);
            (
                return_function,
                ProblemAnswer::Unsigned(expected_value as u128),
                problem_number,
            )
        }
    }};
}

#[macro_export()]
macro_rules! define_string_problem {
    ($year:ident::$day:ident::$task:ident, $expected:expr) => {{
        let problem_number = &stringify!($year::$day::$task)[..];
        use $year::$day::$task::ans as ans_func;
        let expected_value = $expected;
        let return_function: &dyn Fn() -> ProblemAnswer =
            &|| ProblemAnswer::String(ans_func() as String);
        (
            return_function,
            ProblemAnswer::String(expected_value.to_string()),
            problem_number,
        )
    }};
}

pub mod year2022;
pub mod year2023;
pub mod year2024;
