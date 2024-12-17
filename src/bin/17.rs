advent_of_code::solution!(17);

enum InstructionType {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl InstructionType {
    fn from(opcode: u8) -> Self {
        match opcode {
            0 => InstructionType::Adv,
            1 => InstructionType::Bxl,
            2 => InstructionType::Bst,
            3 => InstructionType::Jnz,
            4 => InstructionType::Bxc,
            5 => InstructionType::Out,
            6 => InstructionType::Bdv,
            7 => InstructionType::Cdv,
            _ => panic!(),
        }
    }
}

struct Instruction {
    typ: InstructionType,
    operand: u8,
}

impl Instruction {
    fn from(opcode: u8, operand: u8) -> Self {
        Instruction {
            typ: InstructionType::from(opcode),
            operand,
        }
    }
}

#[derive(Debug)]
struct DeviceState {
    reg_a: u32,
    reg_b: u32,
    reg_c: u32,
    instr_p: usize,
    program: Vec<u8>,
}

impl DeviceState {
    fn from_str(str: &str) -> Self {
        let (reg_str, program_str) = str.split_once("\n\n").unwrap();

        let reg_vals: Vec<_> = reg_str
            .lines()
            .map(|x| x.split_once(": ").unwrap().1)
            .map(|x| x.parse().unwrap())
            .collect();

        let program = program_str
            .split_once(": ")
            .unwrap()
            .1
            .trim()
            .split(',')
            .filter_map(|x| x.parse().ok())
            .collect();

        DeviceState {
            reg_a: reg_vals[0],
            reg_b: reg_vals[1],
            reg_c: reg_vals[2],
            instr_p: 0,
            program,
        }
    }

    fn execute(&mut self) -> String {
        let mut program_output = String::new();

        while let Some(instr) = self.get_instruction() {
            if let Some(output) = self.execute_instruction(instr) {
                if program_output.is_empty() {
                    program_output = output;
                } else {
                    program_output += format!(",{}", output).as_str();
                }
            }
        }

        program_output
    }

    fn get_instruction(&mut self) -> Option<Instruction> {
        if self.instr_p < self.program.len() {
            let opcode = self.program[self.instr_p];
            self.instr_p += 1;
            let operand = self.program[self.instr_p];
            self.instr_p += 1;

            Some(Instruction::from(opcode, operand))
        } else {
            None
        }
    }

    fn get_combo_operand(&self, operand: u8) -> u32 {
        match operand {
            0..4 => operand as u32,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => panic!(),
        }
    }

    fn execute_instruction(&mut self, instr: Instruction) -> Option<String> {
        match instr.typ {
            InstructionType::Adv => {
                self.reg_a = self.reg_a / 2u32.pow(self.get_combo_operand(instr.operand));

                None
            }
            InstructionType::Bxl => {
                self.reg_b = self.reg_b ^ instr.operand as u32;

                None
            }
            InstructionType::Bst => {
                self.reg_b = self.get_combo_operand(instr.operand) % 8;

                None
            }
            InstructionType::Jnz => {
                if self.reg_a != 0 {
                    self.instr_p = instr.operand as usize;
                }

                None
            }
            InstructionType::Bxc => {
                self.reg_b = self.reg_b ^ self.reg_c;

                None
            }
            InstructionType::Out => {
                let output = (self.get_combo_operand(instr.operand) % 8).to_string();

                Some(output)
            }
            InstructionType::Bdv => {
                self.reg_b = self.reg_a / 2u32.pow(self.get_combo_operand(instr.operand));

                None
            }
            InstructionType::Cdv => {
                self.reg_c = self.reg_a / 2u32.pow(self.get_combo_operand(instr.operand));

                None
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut device = DeviceState::from_str(input);

    let output = device.execute();

    Some(output)
}

pub fn part_two(input: &str) -> Option<String> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("4635635210")));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
