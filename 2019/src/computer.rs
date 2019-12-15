pub struct Computer {
    pub input: Vec<i64>,
    pub output: Vec<i64>,
    memory: Vec<i64>,
    ic: usize,
    rel_base: usize,
}

#[derive(PartialEq)]
pub enum Status {
    AwaitingInput,
    ReturningOutput,
    Halt,
}

impl Computer {
    pub fn new() -> Computer {
        Computer {
            input: Vec::new(),
            output: Vec::new(),
            memory: Vec::new(),
            ic: 0,
            rel_base: 0,
        }
    }

    pub fn dump_memory(&self) -> Vec<i64> {
        self.memory.clone()
    }

    fn get_first_operand(&self, inst: i64) -> i64 {
        let param_mode = inst / 100 % 10;
        match param_mode {
            0 => self.memory[self.memory[self.ic + 1] as usize],
            1 => self.memory[self.ic + 1],
            2 => self.memory[self.rel_base + self.ic + 1],
            o => panic!("Unsupported mode: {}", o),
        }
    }

    fn get_second_operand(&self, inst: i64) -> i64 {
        let param_mode = inst / 1000 % 10;
        match param_mode {
            0 => self.memory[self.memory[self.ic + 2] as usize],
            1 => self.memory[self.ic + 2],
            o => panic!("Unsupported mode: {}", o),
        }
    }

    pub fn load(&mut self, program: &Vec<i64>) {
        self.memory.clear();
        self.memory.append(&mut program.clone());
        self.ic = 0;
    }

    pub fn run(self: &mut Self) -> Status {
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
                    if self.input.is_empty() {
                        return Status::AwaitingInput;
                    } else {
                        let dst_addr = self.memory[self.ic + 1] as usize;
                        self.memory[dst_addr] = self.input.remove(0);
                        self.ic += 2;
                    }
                },
                4 => {
                    let output = self.get_first_operand(instruction);
                    self.output.push(output);
                    self.ic += 2;
                    return Status::ReturningOutput;
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
                9 => {
                    self.rel_base += self.get_first_operand(instruction) as usize;
                    self.ic += 2;
                }
                99 => return Status::Halt,
                o => panic!("Wrong opcode{}", o),
            }
        }
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
        computer.load(&input);
        computer.run();
        assert_eq!(computer.memory, output);
    }

    #[test]
    fn test_run_2() {
        let mut computer= Computer::new();
        let input = vec![2,3,0,3,99];
        let output = vec![2,3,0,6,99];
        computer.load(&input);
        computer.run();
        assert_eq!(computer.memory, output);
    }

    #[test]
    fn test_run_3() {
        let mut computer= Computer::new();
        let input = vec![2,4,4,5,99,0];
        let output = vec![2,4,4,5,99,9801];
        computer.load(&input);
        computer.run();
        assert_eq!(computer.memory, output);
    }

    #[test]
    fn test_run_4() {
        let mut computer= Computer::new();
        let input = vec![1,1,1,4,99,5,6,0,99];
        let output = vec![30,1,1,4,2,5,6,0,99];
        computer.load(&input);
        computer.run();
        assert_eq!(computer.memory, output);
    }

    #[test]
    fn test_run_5() {
        let mut computer= Computer::new();
        let input = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
        let output = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
        computer.load(&input);
        loop {
            let status = computer.run();
            if status == Status::Halt { break; }
        }
        assert_eq!(computer.output, output);
    }

    #[test]
    fn test_run_6() {
        let mut computer= Computer::new();
        let input = vec![1102,34915192,34915192,7,4,7,99,0];
        computer.load(&input);
        loop {
            let status = computer.run();
            if status == Status::Halt { break; }
        }
        assert_eq!(computer.output[0].to_string().len(), 16);
    }
}
