use std::io::{self, Read};
use permute::permute;

use advent_of_code_2019::computer::{Computer, Status};

fn main() -> io::Result<()> {
    let program = read_input();
    let res1 = find_highest_signal(&program);
    println!("{:?}", res1);
    Ok(())
}

fn find_highest_signal(program: &Vec<i32>) -> i32 {
    permute(vec![0, 1, 2, 3, 4])
        .iter()
        .map(|phases| {
            let mut amps = init_amps(&program, &phases);
            let mut signal = 0;
            amps.iter_mut().for_each(|amp| {
                amp.input.push(signal);
                loop {
                    let status = amp.run();
                    if status == Status::ReturningOutput {
                        signal = amp.output.remove(0);
                    } else if status == Status::Halt {
                        break;
                    }
                }
            });
            signal
        })
        .max()
        .unwrap()
}

fn init_amps(program: &Vec<i32>, phases: &Vec<i32>) -> Vec<Computer> {
    (0..phases.len())
        .map(|p| {
            let mut c = Computer::new();
            c.load(&program);
            c.input.push(phases[p]);
            c
        })
        .collect()
}

fn read_input() -> Vec<i32> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    buffer.trim()
          .split(',')
          .map(|s| s.parse().unwrap())
          .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let program = vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
        let output = 43210;
        let res1 = find_highest_signal(&program);
        assert_eq!(res1, output);
    }

    #[test]
    fn test_2() {
        let program = vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0];
        let output = 54321;
        let res1 = find_highest_signal(&program);
        assert_eq!(res1, output);
    }

    #[test]
    fn test_3() {
        let program = vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];
        let output = 65210;
        let res1 = find_highest_signal(&program);
        assert_eq!(res1, output);
    }
}
