use std::{collections::HashMap, fs};

pub mod task1 {
    use super::{calculate_directory_sizes, sum_large_directories};

    pub fn ans() -> u128 {
        let mut directories: Vec<u128> = vec![];
        calculate_directory_sizes("resources/2022/day07/input", &mut directories);

        sum_large_directories(&directories, 100_000)
    }
}

pub mod task2 {
    use super::{calculate_directory_sizes, find_smallest_sufficient_directory};

    pub fn ans() -> u128 {
        let mut directories: Vec<u128> = vec![];
        calculate_directory_sizes("resources/2022/day07/input", &mut directories);

        find_smallest_sufficient_directory(&mut directories, 30000000)
    }
}

fn calculate_directory_sizes(file: &str, directories: &mut Vec<u128>) {
    let input_contents = fs::read_to_string(file).expect("Error reading file");
    let mut lines = input_contents.lines();

    if lines.next().unwrap() != "$ cd /" {
        panic!();
    }

    let mut directory: Vec<&str> = vec!["root"];
    let mut directory_size: u128 = 0;
    let mut hash_map: HashMap<String, u128> = HashMap::new();
    loop {
        let cmd_option = lines.next();
        if cmd_option.is_none() {
            break;
        }
        let cmd = cmd_option.unwrap();

        if cmd == "$ cd .." {
            directory.pop();
            hash_map
                .entry(directory.clone().join("/"))
                .and_modify(|size| {
                    *size += directory_size;
                })
                .or_insert(directory_size);
            directory_size = *hash_map.get(&directory.join("/")).unwrap();
        } else if cmd == "$ ls" {
            while lines
                .clone()
                .peekable()
                .peek()
                .unwrap_or(&" ")
                .split_at(1)
                .0
                != "$"
            {
                let item = lines.next().unwrap_or(" ");
                if item == " " {
                    break;
                }
                if item.split_at(3).0 != "dir" {
                    directory_size +=
                        item.split(' ').next().unwrap().parse::<u128>().unwrap();
                }
            }
            hash_map
                .entry(directory.join("/"))
                .and_modify(|size| {
                    *size += directory_size;
                })
                .or_insert(directory_size);
        } else {
            hash_map.entry(directory.join("/"));
            directory.push(cmd.split_at(5).1);
            directory_size = 0;
        }
    }

    while directory.len() > 1 {
        directory.pop();
        hash_map
            .entry(directory.clone().join("/"))
            .and_modify(|size| {
                *size += directory_size;
            })
            .or_insert(directory_size);
        directory_size = *hash_map.get(&directory.clone().join("/")).unwrap();
    }

    for (_, value) in hash_map.iter() {
        directories.push(*value);
    }
}

fn sum_large_directories(directories: &Vec<u128>, max_size: u128) -> u128 {
    let mut sum: u128 = 0;
    for size in directories {
        if *size <= max_size {
            sum += size;
        }
    }
    sum
}

fn find_smallest_sufficient_directory(directories: &mut [u128], required_space: u128) -> u128 {
    directories.sort();
    let iter_dir = directories.iter().rev();

    let necessary_deletion = required_space - (70000000u128 - iter_dir.clone().next().unwrap());

    *iter_dir
        .filter(|x| x >= &&necessary_deletion)
        .last()
        .unwrap()
}
