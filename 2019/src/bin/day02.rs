use std::io::{self, Read};

struct Computer<'a> {
    memory: &'a mut Vec<usize>,
    ic: usize,
}

impl<'a> Computer<'a> {
    fn run(self: &mut Self, input: &Vec<usize>, noun: usize, verb: usize) -> Vec<usize> {
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

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let program = parse(buffer);
    let mut computer = Computer {
        memory: &mut Vec::new(),
        ic: 0,
    };
    let output = computer.run(&program, 12, 2);
    println!("{}", output[0]);
    Ok(())
}

fn parse(input: String) -> Vec<usize> {
    input.split(',')
         .filter_map(|s| s.parse().ok())
         .collect()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_1() {
        let mut computer: Computer = Computer {
            memory: &mut Vec::new(),
            ic: 0,
        };
        let input = vec![1,0,0,0,99];
        let output = vec![2,0,0,0,99];
        assert_eq!(computer.run(&input, 0, 0), output);
    }

    #[test]
    fn test_run_2() {
        let mut computer: Computer = Computer {
            memory: &mut Vec::new(),
            ic: 0,
        };
        let input = vec![2,3,0,3,99];
        let output = vec![2,3,0,6,99];
        assert_eq!(computer.run(&input, 3, 0), output);
    }

    #[test]
    fn test_run_3() {
        let mut computer: Computer = Computer {
            memory: &mut Vec::new(),
            ic: 0,
        };
        let input = vec![2,4,4,5,99,0];
        let output = vec![2,4,4,5,99,9801];
        assert_eq!(computer.run(&input, 4, 4), output);
    }

    #[test]
    fn test_run_4() {
        let mut computer: Computer = Computer {
            memory: &mut Vec::new(),
            ic: 0,
        };
        let input = vec![1,1,1,4,99,5,6,0,99];
        let output = vec![30,1,1,4,2,5,6,0,99];
        assert_eq!(computer.run(&input, 1, 1), output);
    }
}
