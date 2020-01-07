use std::io::{self, Read};
use advent_of_code_2019::arcade_cabinet::ArcadeCabinet;

fn main() -> io::Result<()> {
    let input = read_input();
    let mut arcade = ArcadeCabinet::new();
    arcade.load_game(&input);
    arcade.start_game();
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
