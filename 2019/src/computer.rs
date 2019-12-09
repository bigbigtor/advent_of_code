use std::io;

pub struct Computer {
    memory: Vec<i32>,
    ic: usize,
}

impl Computer {
    pub fn new() -> Computer {
        Computer {
            memory: Vec::new(),
            ic: 0,
        }
    }

    fn get_first_operand(&self, inst: i32) -> i32 {
        let param_mode = inst / 100 % 10;
        match param_mode {
            0 => self.memory[self.memory[self.ic + 1] as usize],
            1 => self.memory[self.ic + 1],
            o => panic!("Unsupported mode: {}", o),
        }
    }

    fn get_second_operand(&self, inst: i32) -> i32 {
        let param_mode = inst / 1000 % 10;
        match param_mode {
            0 => self.memory[self.memory[self.ic + 2] as usize],
            1 => self.memory[self.ic + 2],
            o => panic!("Unsupported mode: {}", o),
        }
    }

    pub fn run(self: &mut Self, program: &Vec<i32>) -> Vec<i32> {
        self.memory.clear();
        self.memory.append(&mut program.clone());
        self.ic = 0;
        loop {
            let instruction = self.memory[self.ic];
            let opcode = instruction % 100;
            match opcode {
                1 => {
                    let dst_addr = self.memory[self.ic + 3] as usize;
                    self.memory[dst_addr] = self.get_first_operand(instruction) 
                                          + self.get_second_operand(instruction);
                    self.ic += 4;
                },
                2 => {
                    let dst_addr = self.memory[self.ic + 3] as usize;
                    self.memory[dst_addr] = self.get_first_operand(instruction) 
                                          * self.get_second_operand(instruction);
                    self.ic += 4;
                },
                3 => {
                    let mut input = String::new();
                    println!("Please, enter the input value (integer):");
                    io::stdin().read_line(&mut input).expect("wrong input");
                    let input = input.trim()
                                     .parse::<i32>()
                                     .unwrap();
                    let dst_addr = self.memory[self.ic + 1] as usize;
                    self.memory[dst_addr] = input;
                    self.ic += 2;
                },
                4 => {
                    println!("{}", self.get_first_operand(instruction));
                    self.ic += 2;
                },
                5 => {
                    if self.get_first_operand(instruction) != 0 {
                        self.ic = self.get_second_operand(instruction) as usize;
                    } else {
                        self.ic += 3;
                    }
                },
                6 => {
                    if self.get_first_operand(instruction) == 0 {
                        self.ic = self.get_second_operand(instruction) as usize;
                    } else {
                        self.ic += 3;
                    }
                },
                7 => {
                    let dst_addr = self.memory[self.ic + 3] as usize;
                    self.memory[dst_addr] = if self.get_first_operand(instruction)
                                             < self.get_second_operand(instruction) { 1 } else { 0 };
                    self.ic += 4;
                },
                8 => {
                    let dst_addr = self.memory[self.ic + 3] as usize;
                    self.memory[dst_addr] = if self.get_first_operand(instruction)
                                            == self.get_second_operand(instruction) { 1 } else { 0 };
                    self.ic += 4;
                },
                99 => break,
                o => panic!("Wrong opcode{}", o),
            }
        }
        self.memory.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_1() {
        let mut computer= Computer::new();
        let input = vec![1,0,0,0,99];
        let output = vec![2,0,0,0,99];
        assert_eq!(computer.run(&input), output);
    }

    #[test]
    fn test_run_2() {
        let mut computer= Computer::new();
        let input = vec![2,3,0,3,99];
        let output = vec![2,3,0,6,99];
        assert_eq!(computer.run(&input), output);
    }

    #[test]
    fn test_run_3() {
        let mut computer= Computer::new();
        let input = vec![2,4,4,5,99,0];
        let output = vec![2,4,4,5,99,9801];
        assert_eq!(computer.run(&input), output);
    }

    #[test]
    fn test_run_4() {
        let mut computer= Computer::new();
        let input = vec![1,1,1,4,99,5,6,0,99];
        let output = vec![30,1,1,4,2,5,6,0,99];
        assert_eq!(computer.run(&input), output);
    }
}
