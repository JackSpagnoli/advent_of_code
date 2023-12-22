use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

pub mod task1 {
    use super::disintegratable_blocks;

    pub fn ans() -> u128 {
        disintegratable_blocks("resources/2023/day22/input")
    }
}

pub mod task2 {
    use super::sum_chain_reaction;

    pub fn ans() -> u128 {
        sum_chain_reaction("resources/2023/day22/input")
    }
}

fn sum_chain_reaction(file: &str) -> u128 {
    let blocks = parse_file(file);

    let blocks = fall_blocks(blocks);

    blocks
        .values()
        .map(|block| {
            let mut blocks = blocks.clone();
            blocks.remove(&block.id);
            blocks = blocks
                .into_iter()
                .map(|(id, mut b)| {
                    b.supported_by.remove(&block.id);
                    (id, b)
                })
                .collect();
            chain_reaction(blocks)
        })
        .sum()
}

fn chain_reaction(mut blocks: HashMap<usize, Block>) -> u128 {
    let block_to_fall = blocks
        .iter()
        .filter(|(_, block)| block.z_range.0 > 1)
        .find(|(_, block)| block.supported_by.is_empty());

    if block_to_fall.is_none() {
        return 0;
    }

    let block_to_fall = block_to_fall.unwrap().1.id;
    blocks.remove(&block_to_fall);

    blocks = blocks
        .into_iter()
        .map(|(id, mut b)| {
            b.supported_by.remove(&block_to_fall);
            (id, b)
        })
        .collect();

    1 + chain_reaction(blocks)
}

fn disintegratable_blocks(file: &str) -> u128 {
    let blocks = parse_file(file);

    let blocks = fall_blocks(blocks);

    let structural_blocks: HashMap<usize, Vec<usize>> = structural_blocks(&blocks);
    let non_structural_blocks = (blocks.len() - structural_blocks.len()) as u128;

    let disintegrable_structural_blocks = structural_blocks
        .iter()
        .filter(|(_, supporting)| {
            supporting
                .iter()
                .all(|id| blocks.get(id).unwrap().supported_by.len() > 1)
        })
        .count() as u128;

    non_structural_blocks + disintegrable_structural_blocks
}

fn structural_blocks(blocks: &HashMap<usize, Block>) -> HashMap<usize, Vec<usize>> {
    let mut structural_blocks: HashMap<usize, Vec<usize>> = HashMap::new();
    blocks.values().for_each(|block| {
        block.supported_by.iter().for_each(|supporting_block| {
            structural_blocks
                .entry(*supporting_block)
                .or_default()
                .push(block.id);
        })
    });

    structural_blocks
}

fn fall_blocks(mut blocks: Vec<Block>) -> HashMap<usize, Block> {
    blocks.sort();

    let mut fallen_blocks: HashMap<usize, Block> = HashMap::new();

    for mut block in blocks {
        loop {
            let can_fall = can_fall(&fallen_blocks, &block);

            if can_fall.is_ok() {
                block.z_range.0 -= 1;
                block.z_range.1 -= 1;
            } else {
                block.supported_by = can_fall.unwrap_err();
                fallen_blocks.insert(block.id, block);
                break;
            }
        }
    }

    fallen_blocks
}

fn can_fall(fallen_blocks: &HashMap<usize, Block>, block: &Block) -> Result<(), HashSet<usize>> {
    if block.z_range.0 == 1 {
        return Err(HashSet::new());
    }

    let x_range = block.x_range.0..=block.x_range.1;
    let y_range = block.y_range.0..=block.y_range.1;

    let supporting_cells =
        x_range.flat_map(|x| y_range.clone().map(move |y| (block.z_range.0 - 1, y, x)));

    let supporting_blocks: HashSet<usize> = supporting_cells
        .flat_map(|(z, y, x)| {
            fallen_blocks
                .values()
                .filter(|block| {
                    block.z_range.1 == z
                        && block.y_range.0 <= y
                        && block.y_range.1 >= y
                        && block.x_range.0 <= x
                        && block.x_range.1 >= x
                })
                .map(|block| block.id)
                .collect::<Vec<usize>>()
        })
        .collect();

    if supporting_blocks.is_empty() {
        Ok(())
    } else {
        Err(supporting_blocks)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Block {
    id: usize,
    supported_by: HashSet<usize>,
    y_range: (usize, usize),
    x_range: (usize, usize),
    z_range: (usize, usize),
}

impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Block {
    fn cmp(&self, other: &Self) -> Ordering {
        let z_ordering = self.z_range.0.cmp(&other.z_range.0);
        if z_ordering != Ordering::Equal {
            return z_ordering;
        }

        let y_ordering = self.y_range.0.cmp(&other.y_range.0);
        if y_ordering != Ordering::Equal {
            return y_ordering;
        }

        self.x_range.0.cmp(&other.x_range.0)
    }
}

fn parse_file(file: &str) -> Vec<Block> {
    let contents = std::fs::read_to_string(file).unwrap();

    contents
        .lines()
        .enumerate()
        .map(|(id, line)| {
            let mut parts = line.split('~');

            let mut range_starts = parts.next().unwrap().split(',');
            let x_start = range_starts.next().unwrap().parse::<usize>().unwrap();
            let y_start = range_starts.next().unwrap().parse::<usize>().unwrap();
            let z_start = range_starts.next().unwrap().parse::<usize>().unwrap();

            let mut range_ends = parts.next().unwrap().split(',');
            let x_end = range_ends.next().unwrap().parse::<usize>().unwrap();
            let y_end = range_ends.next().unwrap().parse::<usize>().unwrap();
            let z_end = range_ends.next().unwrap().parse::<usize>().unwrap();

            assert!(y_start <= y_end);
            assert!(x_start <= x_end);
            assert!(z_start <= z_end);

            Block {
                id,
                supported_by: HashSet::new(),
                y_range: (y_start, y_end),
                x_range: (x_start, x_end),
                z_range: (z_start, z_end),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fall_blocks() {
        let blocks = parse_file("resources/2023/day22/test_input");

        let fallen_blocks = fall_blocks(blocks);

        for block in fallen_blocks {
            println!("{:?}", block);
        }
    }

    #[test]
    fn test_disintegratable_blocks() {
        assert_eq!(disintegratable_blocks("resources/2023/day22/test_input"), 5);
    }

    #[test]
    fn test_chain_reaction() {
        let blocks = parse_file("resources/2023/day22/test_input");

        let mut blocks = fall_blocks(blocks);

        let block_to_fall = 0;
        blocks.remove(&block_to_fall);

        blocks = blocks
            .into_iter()
            .map(|(id, mut b)| {
                b.supported_by.remove(&block_to_fall);
                (id, b)
            })
            .collect();

        assert_eq!(chain_reaction(blocks), 6);
    }

    #[test]
    fn test_chain_reaction2() {
        let blocks = parse_file("resources/2023/day22/test_input");

        let mut blocks = fall_blocks(blocks);

        let block_to_fall = 1;
        blocks.remove(&block_to_fall);

        blocks = blocks
            .into_iter()
            .map(|(id, mut b)| {
                b.supported_by.remove(&block_to_fall);
                (id, b)
            })
            .collect();

        assert_eq!(chain_reaction(blocks), 0);
    }

    #[test]
    fn test_sum_chain_reaction() {
        assert_eq!(sum_chain_reaction("resources/2023/day22/test_input"), 7);
    }
}

