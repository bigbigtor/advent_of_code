use std::io::{self, Read};
use std::collections::HashMap;
use advent_of_code_2019::robot::Robot;
use advent_of_code_2019::point::Point;
use advent_of_code_2019::color::Color;

fn main() -> io::Result<()> {
    let input = read_input();
    let mut robot = Robot::new();
    let mut grid: HashMap<Point, Color> = HashMap::new();
    grid.insert(Point::new(0, 0), Color::White);
    robot.run(&input, &mut grid);
    println!("{:?}", grid.len());
    let identifier = get_identifier(&grid);
    println!("{}", identifier);
    Ok(())
}

fn get_identifier(grid: &HashMap<Point, Color>) -> String {
    let xmin = grid.keys().map(|p| p.x).min().unwrap();
    let ymin = grid.keys().map(|p| p.y).min().unwrap();
    let xmax = grid.keys().map(|p| p.x).max().unwrap();
    let ymax = grid.keys().map(|p| p.y).max().unwrap();
    (ymin..=ymax).into_iter()
                 .rev()
                 .map(|y| {
                     (xmin..=xmax).into_iter()
                                  .map(|x| grid.get(&Point::new(x, y)))
                                  .map(|opt| {
                                      match opt {
                                          Some(c) => *c,
                                          None => Color::Black,
                                      }
                                  })
                                  .map(|c| c.to_string())
                                  .collect::<String>()
                 })
                 .collect::<Vec<String>>()
                 .join("\n")
}

fn read_input() -> Vec<i64> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    buffer.trim()
          .split(",")
          .map(|s| s.parse().unwrap())
          .collect()
}
