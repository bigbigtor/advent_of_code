use std::io::{self, Read};
use permute::permute;

use advent_of_code_2019::computer::{Computer, Status};

fn main() -> io::Result<()> {
    let program = read_input();
    let res1 = find_highest_signal(&program);
    let res2 = find_highest_signal_feedback_loop(&program);
    println!("{:?}\n{:?}", res1, res2);
    Ok(())
}

fn find_highest_signal_feedback_loop(program: &Vec<i64>) -> i64 {
    permute(vec![5, 6, 7, 8, 9])
        .iter()
        .map(|phases| {
            let mut amps = init_amps(&program, &phases);
            let mut signal = 0;
            let mut halted: Vec<bool> = vec![false; amps.len()];
            for i in (0..amps.len()).into_iter().cycle() {
                amps[i].input.push(signal);
                match amps[i].run() {
                    Status::ReturningOutput => signal = amps[i].output.remove(0),
                    Status::Halt => halted[i] = true,
                    _ => continue,
                };
                if halted.iter().all(|&h| h == true) {
                    break;
                }
            }
            signal
        })
        .max()
        .unwrap()
}

fn find_highest_signal(program: &Vec<i64>) -> i64 {
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

fn init_amps(program: &Vec<i64>, phases: &Vec<i64>) -> Vec<Computer> {
    (0..phases.len())
        .map(|p| {
            let mut c = Computer::new();
            c.load(&program);
            c.input.push(phases[p]);
            c
        })
        .collect()
}

fn read_input() -> Vec<i64> {
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

    #[test]
    fn test_4() {
        let program = vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5];
        let output = 139629729;
        let res1 = find_highest_signal_feedback_loop(&program);
        assert_eq!(res1, output);
    }

    #[test]
    fn test_5() {
        let program = vec![3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10];
        let output = 18216;
        let res1 = find_highest_signal_feedback_loop(&program);
        assert_eq!(res1, output);
    }
}
