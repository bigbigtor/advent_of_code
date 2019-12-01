use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let fuels = input.lines()
                     .filter_map(|s| s.parse().ok())
                     .map(calculate_fuel)
                     .collect::<Vec<u32>>();
    let res1: u32 = fuels.into_iter().sum();
    println!("{}", res1);
    Ok(())
}

fn calculate_fuel(mass: u32) -> u32 {
    let partial_res = mass/3;
    if partial_res > 2 { partial_res - 2 } else { 0 }
}
