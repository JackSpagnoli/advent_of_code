use std::fs;

pub mod task1 {
    use super::concat_output;

    pub fn ans() -> String {
        concat_output("resources/2024/day17/input.txt")
    }
}

pub mod task2 {
    pub fn ans() -> u128 {
        0
    }
}

struct Program {
    ip: isize,
    program: Vec<u128>,
    reg_a: u128,
    reg_b: u128,
    reg_c: u128,
    output_buffer: Vec<u128>,
}

impl Program {
    fn from_file(file: &str) -> Self {
        let contents = fs::read_to_string(file).unwrap();
        let mut lines = contents.lines();

        let reg_a: u128 = lines.next().unwrap().split(": ").collect::<Vec<&str>>()[1]
            .parse()
            .unwrap();
        let reg_b: u128 = lines.next().unwrap().split(": ").collect::<Vec<&str>>()[1]
            .parse()
            .unwrap();
        let reg_c: u128 = lines.next().unwrap().split(": ").collect::<Vec<&str>>()[1]
            .parse()
            .unwrap();

        lines.next();

        let program: Vec<u128> = lines.next().unwrap().split(": ").collect::<Vec<&str>>()[1]
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect();

        let output_buffer = Vec::new();
        Self {
            ip: 0,
            program,
            reg_a,
            reg_b,
            reg_c,
            output_buffer,
        }
    }

    fn next_op(&mut self) -> Option<()> {
        if self.ip < 0 || self.ip >= self.program.len() as isize {
            return None;
        }

        let opcode = self.program[self.ip as usize];
        let literal_operand = self.program[(self.ip + 1) as usize];
        let combo_operand = match literal_operand {
            4 => Some(self.reg_a),
            5 => Some(self.reg_b),
            6 => Some(self.reg_c),
            7 => None,
            literal => Some(literal),
        };

        match opcode {
            0 => {
                // a division
                let numerator = self.reg_a;

                // 2^operand
                let denominator = (1 << combo_operand.unwrap()) as u128;

                self.reg_a = numerator / denominator;
            }
            1 => {
                // xor
                self.reg_b = self.reg_b ^ literal_operand;
            }
            2 => {
                // modulo
                let numerator = combo_operand.unwrap();
                let denominator = 8;

                self.reg_b = numerator % denominator;
            }
            3 => {
                // jnz
                if self.reg_a != 0 {
                    self.ip = literal_operand as isize;
                    self.ip -= 2;
                }
            }
            4 => {
                // XOR
                self.reg_b = self.reg_b ^ self.reg_c;
            }
            5 => {
                // OUT
                self.output_buffer.push(combo_operand.unwrap() % 8);
            }
            6 => {
                // b division
                let numerator = self.reg_a;

                // 2^operand
                let denominator = (1 << combo_operand.unwrap()) as u128;

                self.reg_b = numerator / denominator;
            }
            7 => {
                // b division
                let numerator = self.reg_a;

                // 2^operand
                let denominator = (1 << combo_operand.unwrap()) as u128;

                self.reg_c = numerator / denominator;
            }
            _ => panic!("Invalid opcode"),
        }

        self.ip += 2;

        Some(())
    }
}

fn concat_output(file: &str) -> String {
    let mut program = Program::from_file(file);

    while program.next_op().is_some() {}

    program
        .output_buffer
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concat_output() {
        assert_eq!(
            concat_output("resources/2024/day17/test_input.txt"),
            "4,6,3,5,6,3,5,2,1,0"
        );
    }

    #[test]
    fn test_program() {
        let mut program = Program {
            ip: 0,
            program: vec![2, 6],
            reg_a: 0,
            reg_b: 0,
            reg_c: 9,
            output_buffer: Vec::new(),
        };

        while program.next_op().is_some() {}

        assert_eq!(program.reg_b, 1);
    }
    #[test]
    fn test_program_2() {
        let mut program = Program {
            ip: 0,
            program: vec![5, 0, 5, 1, 5, 4],
            reg_a: 10,
            reg_b: 0,
            reg_c: 0,
            output_buffer: Vec::new(),
        };

        while program.next_op().is_some() {}

        assert_eq!(program.output_buffer, vec![0, 1, 2]);
    }
}
