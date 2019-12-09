use std::io::{self, Read};

use advent_of_code_2019::computer;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let mut program = parse(buffer);
    let mut computer = computer::Computer::new();
    program[1] = 12;
    program[2] = 2;
    let output = computer.run(&program);
    println!("{}", output[0]);
    let mut found = false;
    for noun in 0..99 {
        for verb in 0..99 {
            program[1] = noun;
            program[2] = verb;
            if computer.run(&program)[0] == 19690720 {
                println!("{}", 100 * noun + verb);
                found = true;
                break;
            }
        }
        if found { break; }
    }
    Ok(())
}

fn parse(input: String) -> Vec<i32> {
    input.split(',')
         .filter_map(|s| s.parse().ok())
         .collect()
}
