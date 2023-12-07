use std::fs;

use regex::Regex;

pub mod task1 {
    use super::product_top_worries;

    pub fn ans() -> u128 {
        product_top_worries("resources/2022/day11/input", 20, true)
    }
}

pub mod task2 {
    use super::product_top_worries;

    pub fn ans() -> u128 {
        product_top_worries("resources/2022/day11/input", 10000, false)
    }
}

type Rule = (
    Vec<isize>,
    Box<dyn Fn(isize) -> isize>,
    usize,
    usize,
    usize,
    usize,
);

fn parse_rules(file: &str) -> Vec<Rule> {
    let contents = fs::read_to_string(file).expect("Error reading file");

    let regex = Regex::new(
        r"Monkey \d+:\n  Starting items: (?P<items>(?:\d+,?\s?)+)\n  Operation: new = (?P<operand1>[^ ]+) (?P<operator>.) (?P<operand2>[^ ]+)\n  Test: divisible by (?P<divisor>\d+)\n    If true: throw to monkey (?P<true>\d+)\n    If false: throw to monkey (?P<false>\d+)"
    ).unwrap();

    let mut rules: Vec<Rule> = vec![];
    for capture in regex.captures_iter(&contents) {
        let mut monkey: Rule = (vec![], Box::new(move |_| 0), 0, 0, 0, 0);
        for item in capture["items"].split(", ") {
            monkey.0.push(item.parse::<isize>().unwrap());
        }

        monkey.1 = generate_closure(
            capture["operator"].to_owned(),
            capture["operand1"].to_owned(),
            capture["operand2"].to_owned(),
        );

        monkey.2 = capture["divisor"].parse::<usize>().unwrap();
        monkey.3 = capture["true"].parse::<usize>().unwrap();
        monkey.4 = capture["false"].parse::<usize>().unwrap();

        rules.push(monkey);
    }

    rules
}

fn generate_closure<'a>(
    operator: String,
    operand1: String,
    operand2: String,
) -> Box<dyn (Fn(isize) -> isize) + 'a> {
    let inner_closure: Box<dyn Fn(isize, isize) -> isize>;
    if operator == "+" {
        inner_closure = Box::new(move |a, b| a + b);
    } else if operator == "*" {
        inner_closure = Box::new(move |a, b| a * b);
    } else {
        panic!("Invalid operation");
    }

    let mid_closure: Box<dyn Fn(isize, isize) -> isize> = if operand1 == "old" {
        Box::new(inner_closure)
    } else {
        Box::new(move |_, b| inner_closure(operand1.parse::<isize>().unwrap(), b))
    };

    if operand2 == "old" {
        Box::new(move |old| mid_closure(old, old))
    } else {
        Box::new(move |old| mid_closure(old, operand2.parse::<isize>().unwrap()))
    }
}

fn product_top_worries(file: &str, rounds: usize, divide_by_three: bool) -> u128 {
    let mut rules = parse_rules(file);

    for _ in 0..rounds {
        make_moves(&mut rules, divide_by_three);
    }

    let mut inspections: Vec<u128> = rules.iter().map(|monkey| monkey.5 as u128).collect();

    inspections.sort_by(|a, b| b.cmp(a));

    inspections[0] * inspections[1]
}

fn make_moves(monkeys: &mut Vec<Rule>, divide_by_three: bool) {
    let divisor = find_divisor(monkeys);
    for i in 0..monkeys.len() {
        monkeys[i].5 += monkeys[i].0.len();
        for item in 0..monkeys[i].0.len() {
            let mut worry = monkeys[i].0[item];

            worry = (monkeys[i].1)(worry);
            if divide_by_three {
                worry /= 3;
            } else {
                worry %= divisor;
            }

            let next_monkey = if worry % (monkeys[i].2 as isize) == 0 {
                monkeys[i].3
            } else {
                monkeys[i].4
            };
            monkeys[next_monkey].0.push(worry);
        }
        monkeys[i].0 = vec![];
    }
}

fn find_divisor(monkeys: &Vec<Rule>) -> isize {
    let mut prod: isize = 1;
    for monkey in monkeys {
        prod *= monkey.2 as isize;
    }
    prod
}
