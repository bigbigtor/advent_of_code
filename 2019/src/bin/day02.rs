use std::io::{self, Read};

use advent_of_code_2019::computer;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let program = parse(buffer);
    let mut computer = computer::Computer::new(); 
    let output = computer.run(&program, 12, 2);
    println!("{}", output[0]);
    Ok(())
}

fn parse(input: String) -> Vec<usize> {
    input.split(',')
         .filter_map(|s| s.parse().ok())
         .collect()
}
