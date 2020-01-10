extern crate regex;

use std::io::{self, Read};
use advent_of_code_2019::moon::Moon;
use regex::Regex;

fn main() -> io::Result<()> {
    let mut moons = get_input();
    for _time in 0..1000 {
        Moon::apply_gravity(&mut moons);
        Moon::apply_velocity(&mut moons);
    }
    let energy = Moon::get_system_total_energy(&moons);
    let steps = Moon::get_cycle_steps(&moons);
    println!("{}", energy);
    println!("{}", steps);
    Ok(())
}

fn get_input() -> Vec<Moon> {
    let mut buffer = String::new();
    let re = r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>";
    let regex = Regex::new(re).unwrap();
    io::stdin().read_to_string(&mut buffer).unwrap();
    buffer.trim()
          .lines()
          .map(|line| {
              let caps = regex.captures_iter(line).next().unwrap();
              Moon::new(
                  caps[1].parse().unwrap(),
                  caps[2].parse().unwrap(),
                  caps[3].parse().unwrap(),
              )
          })
          .collect()
}
