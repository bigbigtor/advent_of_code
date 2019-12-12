use std::io::{self, Read};

use advent_of_code_2019::computer::{Computer, Status};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let program = parse(buffer);
    let res1 = run_diagnostic(&program, 1);
    let res2 = run_diagnostic(&program, 5);
    res1.iter().for_each(|r| println!("{}", r));
    res2.iter().for_each(|r| println!("{}", r));
    Ok(())
}

fn run_diagnostic(program: &Vec<i32>, input: i32) -> Vec<i32> {
    let mut output = Vec::new();
    let mut computer = Computer::new();
    computer.load(&program);
    computer.input.push(input);
    loop {
        let status = computer.run();
        if status == Status::ReturningOutput {
            output.push(computer.output.remove(0));
        } else if status == Status::Halt {
            break;
        }
    }
    output
}

fn parse(input: String) -> Vec<i32> {
    input.trim()
         .split(',')
         .map(|s| s.parse().unwrap())
         .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_1() {
        let program = vec![3,0,4,0,99];
        let input = 69;
        let output = vec![69];
        let res1 = run_diagnostic(&program, input);
        assert_eq!(res1, output);
    }

    #[test]
    fn test_equal_pos_mode() {
        let program = vec![3,9,8,9,10,9,4,9,99,-1,8];
        let (input1, input2) = (8, 6);
        let (output1, output2) = (vec![1], vec![0]);
        let res1 = run_diagnostic(&program, input1);
        let res2 = run_diagnostic(&program, input2);
        assert_eq!(res1, output1);
        assert_eq!(res2, output2);
    }

    #[test]
    fn test_less_than_pos_mode() {
        let program = vec![3,9,7,9,10,9,4,9,99,-1,8];
        let (input1, input2) = (8, 6);
        let (output1, output2) = (vec![0], vec![1]);
        let res1 = run_diagnostic(&program, input1);
        let res2 = run_diagnostic(&program, input2);
        assert_eq!(res1, output1);
        assert_eq!(res2, output2);
    }

    #[test]
    fn test_equal_imm_mode() {
        let program = vec![3,3,1108,-1,8,3,4,3,99];
        let (input1, input2) = (8, 6);
        let (output1, output2) = (vec![1], vec![0]);
        let res1 = run_diagnostic(&program, input1);
        let res2 = run_diagnostic(&program, input2);
        assert_eq!(res1, output1);
        assert_eq!(res2, output2);
    }

    #[test]
    fn test_less_than_imm_mode() {
        let program = vec![3,3,1107,-1,8,3,4,3,99];
        let (input1, input2) = (8, 6);
        let (output1, output2) = (vec![0], vec![1]);
        let res1 = run_diagnostic(&program, input1);
        let res2 = run_diagnostic(&program, input2);
        assert_eq!(res1, output1);
        assert_eq!(res2, output2);
    }

    #[test]
    fn test_jump_pos_mode() {
        let program = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
        let (input1, input2) = (8, 0);
        let (output1, output2) = (vec![1], vec![0]);
        let res1 = run_diagnostic(&program, input1);
        let res2 = run_diagnostic(&program, input2);
        assert_eq!(res1, output1);
        assert_eq!(res2, output2);
    }

    #[test]
    fn test_jump_imm_mode() {
        let program = vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1];
        let (input1, input2) = (8, 0);
        let (output1, output2) = (vec![1], vec![0]);
        let res1 = run_diagnostic(&program, input1);
        let res2 = run_diagnostic(&program, input2);
        assert_eq!(res1, output1);
        assert_eq!(res2, output2);
    }

    #[test]
    fn test_combined() {
        let program = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
        let (input1, input2, input3) = (7, 8, 9);
        let (output1, output2, output3) = (vec![999], vec![1000], vec![1001]);
        let res1 = run_diagnostic(&program, input1);
        let res2 = run_diagnostic(&program, input2);
        let res3 = run_diagnostic(&program, input3);
        assert_eq!(res1, output1);
        assert_eq!(res2, output2);
        assert_eq!(res3, output3);
    }
}
