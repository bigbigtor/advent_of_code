use advent_of_code_2019::repair_droid::RepairDroid;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let input = read_input();
    let mut droid = RepairDroid::new();
    droid.load(&input);
    droid.map_area();
    let res1 = droid.get_target_position();
    println!("{:?}", res1);
    Ok(())
}

fn read_input() -> Vec<i64> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    buffer.trim()
          .split(",")
          .map(|line| line.parse().unwrap())
          .collect()
}
