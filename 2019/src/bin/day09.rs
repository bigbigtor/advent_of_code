use std::io::{self, Read};
use advent_of_code_2019::computer::{Computer, Status};

fn main() -> io::Result<()> {
    let input = read_input();
    run_boost(&input, 1).iter().for_each(|out| println!("{}", out));
    run_boost(&input, 2).iter().for_each(|out| println!("{}", out));
    Ok(())
}

fn run_boost(input: &Vec<i64>, mode: i64) -> Vec<i64> {
    let mut computer = Computer::new();
    computer.load(&input);
    computer.input.push(mode);
    loop {
        let status = computer.run();
        if status == Status::Halt { break; }
    }
    computer.output
}

fn read_input() -> Vec<i64> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    buffer.trim()
          .split(",")
          .map(|s| s.parse().unwrap())
          .collect()
}
