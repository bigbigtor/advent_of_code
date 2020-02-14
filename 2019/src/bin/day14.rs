use std::io::{self, Read};
use advent_of_code_2019::nanofactory::Nanofactory;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let nanofactory = Nanofactory::from(input.as_str());
    let part1 = nanofactory.find_min_ore_qty(1);
    let part2 = find_max_fuel(&nanofactory);
    println!("{}", part1);
    println!("{}", part2);
    Ok(())
}

fn find_max_fuel(factory: &Nanofactory) -> u64 {
    let ore = 1_000_000_000_000;
    let mut min = ore / factory.find_min_ore_qty(1);
    let mut max = min * 2;
    while (max - min) > 1 {
        let middle = (min + max) / 2;
        if factory.find_min_ore_qty(middle) < ore {
            min = middle;
        } else {
            max = middle;
        }
    }
    min
}
