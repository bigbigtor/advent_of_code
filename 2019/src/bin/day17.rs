use std::io::{self, Read};
use std::collections::HashMap;
use advent_of_code_2019::computer::{Computer, Status};
use advent_of_code_2019::point::Point;

fn main() -> io::Result<()> {
    let ascii_program = read_input()?;
    let mut computer = Computer::new();
    computer.load(&ascii_program);
    while computer.run() == Status::ReturningOutput {
        continue;
    }
    display_map(&computer.output);
    let map = populate_map(&computer.output);
    let crossings = find_crossings(&map);
    let res1 = crossings.iter()
                        .map(|p| p.x * p.y)
                        .sum::<i16>();
    println!("{:?}", res1);
    Ok(())
}

fn populate_map(output: &Vec<i64>) -> HashMap<Point, i64> {
    let width = output.iter().position(|&d| d == 10).unwrap() + 1;
    output.iter()
          .enumerate()
          .map(|(i, c)| {
              let point = Point::new(
                  (i % width) as i16,
                  (i / width) as i16
              );
              (point, *c)
          })
          .collect()
}

fn display_map(output: &Vec<i64>) {
    let display = output.iter().map(|&c| (c as u8) as char).collect::<String>();
    println!("{}", display);
}

fn find_crossings(map: &HashMap<Point, i64>) -> Vec<Point> {
    map.iter()
       .filter(|(_k, &v)| v == 35)
       .map(|(k, _v)| (k, k.get_neighbours()))
       .filter(|(_k, n)| {
           n.iter()
            .filter(|p| map.contains_key(p))
            .filter(|p| map[p] == 35)
            .collect::<Vec<_>>()
            .len() == 4
       })
       .map(|(k, _n)| *k)
       .collect::<Vec<_>>()
}

fn read_input() -> io::Result<Vec<i64>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let input = buffer.trim()
                      .split(",")
                      .filter_map(|s| s.parse().ok())
                      .collect();
    Ok(input)
}
