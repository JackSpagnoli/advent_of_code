use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub mod task1 {
    use super::sum_middle_valid_page_numbers;

    pub fn ans() -> u128 {
        sum_middle_valid_page_numbers("resources/2024/day05/input.txt")
    }
}

pub mod task2 {
    pub fn ans() -> u128 {
        0
    }
}

type Rules = HashMap<u128, HashSet<u128>>;

fn sum_middle_valid_page_numbers(file: &str) -> u128 {
    let contents = fs::read_to_string(file).unwrap();
    let mut contents = contents.split("\n\n");

    let rules: Vec<&str> = contents.next().unwrap().lines().collect();
    let pages: Vec<&str> = contents.next().unwrap().lines().collect();

    // A rule states that page A must be printed before page B
    // Ie. rules.get(B) = A
    let rules: Vec<(u128, u128)> = rules
        .into_iter()
        .map(|line| {
            let mut line = line.split("|");
            let x: u128 = line.next().unwrap().parse().unwrap();
            let y: u128 = line.next().unwrap().parse().unwrap();

            (x, y)
        })
        .collect();

    let mut rule_map: Rules = HashMap::new();
    for (x, y) in rules {
        rule_map.entry(y).or_insert(HashSet::new()).insert(x);
    }

    pages
        .into_iter()
        .map(|line| {
            line.split(",")
                .map(|num| num.parse().unwrap())
                .collect::<Vec<u128>>()
        })
        .filter(|page| is_valid_page(&rule_map, page))
        .map(|page| {
            let len = page.len();
            let middle_index = len / 2;

            page[middle_index]
        })
        .sum()
}

fn is_valid_page(rules: &Rules, pages: &Vec<u128>) -> bool {
    let pages_to_print = pages.into_iter().map(|x| *x).collect::<HashSet<u128>>();

    let relevant_rules: Rules = rules
        .into_iter()
        .filter(|(y, _)| pages_to_print.contains(y))
        .map(|(y, x_set)| {
            let relevant_x = x_set
                .into_iter()
                .filter(|x| pages_to_print.contains(x))
                .map(|x| *x)
                .collect::<HashSet<u128>>();
            (*y, relevant_x)
        })
        .collect();

    let mut printed_pages: HashSet<u128> = HashSet::new();

    for page in pages {
        if let Some(prior) = relevant_rules.get(page) {
            if !prior.is_subset(&printed_pages) {
                return false;
            }
        }

        printed_pages.insert(*page);
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_middle_valid_page_numbers() {
        assert_eq!(
            sum_middle_valid_page_numbers("resources/2024/day05/test.txt"),
            143
        );
    }
}
