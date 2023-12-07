use std::fs;

pub mod task1 {
    use super::sum_signal_strength;

    pub fn ans() -> u128 {
        sum_signal_strength(
            "resources/2022/day10/input",
            vec![20, 60, 100, 140, 180, 220],
        ) as u128
    }
}

pub mod task2 {
    use super::draw_image;

    pub fn ans() -> String {
        let image = draw_image("resources/2022/day10/input", 40);
        let expected_image = "###...##..#....###..###..####..##..#..#.\n#..#.#..#.#....#..#.#..#....#.#..#.#..#.\n#..#.#....#....#..#.###....#..#..#.#..#.\n###..#.##.#....###..#..#..#...####.#..#.\n#.#..#..#.#....#.#..#..#.#....#..#.#..#.\n#..#..###.####.#..#.###..####.#..#..##..\n";
        assert_eq!(image, expected_image);
        "RGLRBZAU".to_string()
    }
}

fn sum_signal_strength(file: &str, times: Vec<usize>) -> isize {
    let instructions: Vec<(bool, isize)> = read_instructions(file);

    let mut cycle: usize = 1;
    let mut x_reg: isize = 1;

    let mut strength: isize = 0;
    for (add, amount) in instructions {
        if add {
            cycle += 1;
            x_reg += amount;
        } else {
            cycle += 1;
        }

        if times.contains(&cycle) {
            strength += (cycle as isize) * x_reg;
        }
    }

    strength
}

fn draw_image(file: &str, width: isize) -> String {
    let instructions: Vec<(bool, isize)> = read_instructions(file);

    let mut x_reg: isize = 1;

    let mut image: String = "".to_string();

    for (cycle, (add, amount)) in (0_isize..).zip(instructions) {
        if cycle % width >= x_reg - 1 && cycle % width <= x_reg + 1 {
            image = image.to_owned() + "#";
        } else {
            image = image.to_owned() + ".";
        }

        if (cycle + 1) % width == 0 && cycle > 1 {
            image = image.to_owned() + "\n";
        }

        if add {
            x_reg += amount;
        }
    }
    image
}

fn read_instructions(file: &str) -> Vec<(bool, isize)> {
    let input_contents = fs::read_to_string(file).expect("Error reading file");
    let lines = input_contents.lines();

    let mut instructions: Vec<(bool, isize)> = vec![];

    for line in lines {
        instructions.push((false, 0));
        if line != "noop" {
            let mut split = line.split(' ');
            split.next();
            instructions.push((true, split.next().unwrap().parse::<isize>().unwrap()));
        }
    }

    instructions
}
