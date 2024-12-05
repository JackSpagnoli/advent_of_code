use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

pub mod task1 {
    use super::sum_middle_valid_page_numbers;

    pub fn ans() -> u128 {
        sum_middle_valid_page_numbers("resources/2024/day05/input.txt")
    }
}

pub mod task2 {
    use super::sum_middle_corrected_page_numbers;

    pub fn ans() -> u128 {
        sum_middle_corrected_page_numbers("resources/2024/day05/input.txt")
    }
}

type Rules = HashMap<u128, HashSet<u128>>;
type Update = Vec<u128>;
type Updates = Vec<Update>;

fn read_file(file: &str) -> (Updates, Rules) {
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

    let pages: Updates = pages
        .into_iter()
        .map(|line| {
            line.split(",")
                .map(|num| num.parse().unwrap())
                .collect::<Update>()
        })
        .collect();

    (pages, rule_map)
}

fn sum_middle_valid_page_numbers(file: &str) -> u128 {
    let (pages, rule_map) = read_file(file);

    pages
        .into_iter()
        .filter(|page| is_valid_update(&rule_map, page))
        .map(|page| {
            let len = page.len();
            let middle_index = len / 2;

            page[middle_index]
        })
        .sum()
}

fn sum_middle_corrected_page_numbers(file: &str) -> u128 {
    let (pages, rules) = read_file(file);

    pages
        .into_iter()
        .filter(|update| !is_valid_update(&rules, update))
        .map(|update| correct_invalid_update(&rules, &update))
        .map(|page| {
            let len = page.len();
            let middle_index = len / 2;

            page[middle_index]
        })
        .sum()
}

fn find_relevant_rules(rules: Rules, update: &Update) -> Rules {
    let pages_to_print = update.into_iter().map(|x| *x).collect::<HashSet<u128>>();

    let relevant_rules: Rules = rules
        .into_iter()
        .filter(|(y, _)| pages_to_print.contains(y))
        .map(|(y, x_set)| {
            let relevant_x = x_set
                .into_iter()
                .filter(|x| pages_to_print.contains(x))
                .map(|x| x)
                .collect::<HashSet<u128>>();
            (y, relevant_x)
        })
        .collect();

    relevant_rules
}

fn is_valid_update(rules: &Rules, update: &Update) -> bool {
    let relevant_rules = find_relevant_rules(rules.clone(), update);

    let mut printed_pages: HashSet<u128> = HashSet::new();

    for page in update {
        if let Some(prior) = relevant_rules.get(page) {
            if !prior.is_subset(&printed_pages) {
                return false;
            }
        }

        printed_pages.insert(*page);
    }

    true
}

fn correct_invalid_update(rules: &Rules, update: &Update) -> Update {
    let relevant_rules = find_relevant_rules(rules.clone(), update);

    let mut corrected_update: Update = Vec::new();

    let mut update_queue: VecDeque<u128> = update.clone().into();

    while let Some(page) = update_queue.pop_front() {
        // If page has no rules, add to printed pages and continue
        if let None = relevant_rules.get(&page) {
            corrected_update.push(page);
            continue;
        }

        let prior = relevant_rules.get(&page).unwrap();

        // If all prior pages have been printed, add to printed pages and continue
        let printed_pages: HashSet<u128> = corrected_update.iter().map(|x| *x).collect();
        if prior.is_subset(&printed_pages) {
            corrected_update.push(page);
            continue;
        }

        // If page is not in a valid location, requeue
        update_queue.push_back(page);
    }

    corrected_update
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

    #[test]
    fn test_sum_middle_corrected_page_numbers() {
        assert_eq!(
            sum_middle_corrected_page_numbers("resources/2024/day05/test.txt"),
            123
        );
    }
}
