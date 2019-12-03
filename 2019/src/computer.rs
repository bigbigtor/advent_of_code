pub struct Computer {
    memory: Vec<usize>,
    ic: usize,
}

impl Computer {
    pub fn new() -> Computer {
        Computer {
            memory: Vec::new(),
            ic: 0,
        }
    }

    pub fn run(self: &mut Self, input: &Vec<usize>, noun: usize, verb: usize) -> Vec<usize> {
        self.memory.clear();
        self.memory.append(&mut input.clone());
        self.memory[1] = noun;
        self.memory[2] = verb;
        self.ic = 0;
        loop {
            let opcode = self.memory[self.ic];
            match opcode {
                1 => {
                    let op1_addr = self.memory[self.ic + 1];
                    let op2_addr = self.memory[self.ic + 2];
                    let dst_addr = self.memory[self.ic + 3];
                    self.memory[dst_addr] = self.memory[op1_addr] + self.memory[op2_addr];
                    self.ic += 4;
                },
                2 => {
                    let op1_addr = self.memory[self.ic + 1];
                    let op2_addr = self.memory[self.ic + 2];
                    let dst_addr = self.memory[self.ic + 3];
                    self.memory[dst_addr] = self.memory[op1_addr] * self.memory[op2_addr];
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
        assert_eq!(computer.run(&input, 0, 0), output);
    }

    #[test]
    fn test_run_2() {
        let mut computer= Computer::new();
        let input = vec![2,3,0,3,99];
        let output = vec![2,3,0,6,99];
        assert_eq!(computer.run(&input, 3, 0), output);
    }

    #[test]
    fn test_run_3() {
        let mut computer= Computer::new();
        let input = vec![2,4,4,5,99,0];
        let output = vec![2,4,4,5,99,9801];
        assert_eq!(computer.run(&input, 4, 4), output);
    }

    #[test]
    fn test_run_4() {
        let mut computer= Computer::new();
        let input = vec![1,1,1,4,99,5,6,0,99];
        let output = vec![30,1,1,4,2,5,6,0,99];
        assert_eq!(computer.run(&input, 1, 1), output);
    }
}
