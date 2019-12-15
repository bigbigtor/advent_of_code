pub struct Computer {
    pub input: Vec<i64>,
    pub output: Vec<i64>,
    memory: Vec<i64>,
    ic: usize,
    rel_base: i64,
}

#[derive(PartialEq)]
pub enum Status {
    AwaitingInput,
    ReturningOutput,
    Halt,
}

enum Mode {
    Immediate,
    Position,
    Relative,
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

    fn get_modes(&self, inst: i64, param_num: u32) -> Vec<Mode> {
        let mut modes = Vec::new();
        for p in 1..=param_num {
            let param_mode = inst / (10 as i64).pow(p + 1) % 10;
            let mode = match param_mode {
                0 => Mode::Position,
                1 => Mode::Immediate,
                2 => Mode::Relative,
                o => panic!("Unsupported mode: {}", o),
            };
            modes.push(mode);
        }
        modes
    }

    fn read_memory(&mut self, pos: usize, mode: &Mode) -> i64 {
        self.allocate_if_needed(pos);
        match mode {
            Mode::Immediate => self.memory[pos],
            Mode::Position => self.read_memory(self.memory[pos] as usize, &Mode::Immediate),
            Mode::Relative => self.read_memory((self.rel_base + self.memory[pos]) as usize, &Mode::Immediate),
        }
    }

    fn write_memory(&mut self, pos: usize, mode: &Mode, value: i64) {
        self.allocate_if_needed(pos);
        let dst_addr = match mode {
            Mode::Position => self.memory[pos] as usize,
            Mode::Relative => (self.memory[pos] + self.rel_base) as usize,
            Mode::Immediate => panic!("Immediate mode not supported for writes"),
        };
        self.allocate_if_needed(dst_addr);
        self.memory[dst_addr] = value;
    }

    fn allocate_if_needed(&mut self, pos: usize) {
        if self.memory.len() <= pos {
            self.memory.resize_with(pos + 1, || 0);
        }
    }

    pub fn load(&mut self, program: &Vec<i64>) {
        self.memory.clear();
        self.memory.append(&mut program.clone());
        self.ic = 0;
        self.rel_base = 0;
    }

    pub fn run(self: &mut Self) -> Status {
        loop {
            let instruction = self.memory[self.ic];
            let opcode = instruction % 100;
            match opcode {
                1 => {
                    let modes = self.get_modes(instruction, 3);
                    let result = self.read_memory(self.ic + 1, &modes[0])
                               + self.read_memory(self.ic + 2, &modes[1]);
                    self.write_memory(self.ic + 3, &modes[2], result);
                    self.ic += 4;
                },
                2 => {
                    let modes = self.get_modes(instruction, 3);
                    let result = self.read_memory(self.ic + 1, &modes[0])
                               * self.read_memory(self.ic + 2, &modes[1]);
                    self.write_memory(self.ic + 3, &modes[2], result);
                    self.ic += 4;
                },
                3 => {
                    if self.input.is_empty() {
                        return Status::AwaitingInput;
                    } else {
                        let input = self.input.remove(0);
                        let modes = self.get_modes(instruction, 1);
                        self.write_memory(self.ic + 1, &modes[0], input);
                        self.ic += 2;
                    }
                },
                4 => {
                    let modes = self.get_modes(instruction, 1);
                    let output = self.read_memory(self.ic +1, &modes[0]);
                    self.output.push(output);
                    self.ic += 2;
                    return Status::ReturningOutput;
                },
                5 => {
                    let modes = self.get_modes(instruction, 2);
                    if self.read_memory(self.ic + 1, &modes[0]) != 0 {
                        self.ic = self.read_memory(self.ic + 2, &modes[1]) as usize;
                    } else {
                        self.ic += 3;
                    }
                },
                6 => {
                    let modes = self.get_modes(instruction, 2);
                    if self.read_memory(self.ic + 1, &modes[0]) == 0 {
                        self.ic = self.read_memory(self.ic + 2, &modes[1]) as usize;
                    } else {
                        self.ic += 3;
                    }
                },
                7 => {
                    let modes = self.get_modes(instruction, 3);
                    let result = if self.read_memory(self.ic + 1, &modes[0])
                                  < self.read_memory(self.ic + 2, &modes[1]) { 1 } else { 0 };
                    self.write_memory(self.ic + 3, &modes[2], result);
                    self.ic += 4;
                },
                8 => {
                    let modes = self.get_modes(instruction, 3);
                    let result = if self.read_memory(self.ic + 1, &modes[0])
                                 == self.read_memory(self.ic + 2, &modes[1]) { 1 } else { 0 };
                    self.write_memory(self.ic + 3, &modes[2], result);
                    self.ic += 4;
                },
                9 => {
                    let modes = self.get_modes(instruction, 1);
                    self.rel_base += self.read_memory(self.ic + 1, &modes[0]);
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
