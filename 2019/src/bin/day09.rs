use std::io::{self, Read};
use advent_of_code_2019::computer::{Computer, Status};

fn main() -> io::Result<()> {
    let input = read_input();
    let mut computer = Computer::new();
    computer.load(&input);
    computer.input.push(1);
    loop {
        let status = computer.run();
        if status == Status::Halt { break; }
    }
    computer.output.iter().for_each(|out| println!("{}", out));
    Ok(())
}

fn read_input() -> Vec<i64> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    buffer.trim()
          .split(",")
          .map(|s| s.parse().unwrap())
          .collect()
}
