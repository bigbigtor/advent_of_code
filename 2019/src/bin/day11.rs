use std::io::{self, Read};
use advent_of_code_2019::robot::Robot;

fn main() -> io::Result<()> {
    let input = read_input();
    let mut robot = Robot::new();
    let res1 = robot.run(&input);
    println!("{:?}", res1);
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
