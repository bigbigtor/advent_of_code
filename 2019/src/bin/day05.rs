use std::io::{self, Read};

use advent_of_code_2019::computer;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let program = parse(buffer);
    let mut computer = computer::Computer::new();
    computer.run(&program);
    Ok(())
}

fn parse(input: String) -> Vec<i32> {
    input.trim()
         .split(',')
         .map(|s| s.parse().unwrap())
         .collect()
}
