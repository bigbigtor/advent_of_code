use std::io::{self, Read};

use advent_of_code_2019::computer;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let program = parse(buffer);
    let mut computer = computer::Computer::new(); 
    let output = computer.run(&program, 12, 2);
    println!("{}", output[0]);
    let mut noun = 0;
    let mut verb = 0;
    let mut found = false;
    for n in 0..99 {
        for v in 0..99 {
            if computer.run(&program, n, v)[0] == 19690720 {
                noun = n;
                verb = v;
                found = true;
                break;
            }
        }
        if found { break; }
    }
    println!("{}", 100 * noun + verb);
    Ok(())
}

fn parse(input: String) -> Vec<i32> {
    input.split(',')
         .filter_map(|s| s.parse().ok())
         .collect()
}
