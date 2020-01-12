use std::io::{self, Read};
use advent_of_code_2019::nanofactory::Nanofactory;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let nanofactory = Nanofactory::from(input.as_str());
    let part1 = nanofactory.find_min_ore_qty();
    println!("{}", part1);
    Ok(())
}
