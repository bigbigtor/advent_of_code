use std::io::{self, Read};
use std::collections::HashSet;
use std::f32;
use advent_of_code_2019::point::Point;

fn main() -> io::Result<()> {
    let input = read_input()?;
    let asteroids = get_asteroids(&input);
    let res1 = get_best_count(&asteroids);
    println!("{}", res1);
    Ok(())
}

fn get_best_count(asteroids: &Vec<Point>) -> usize {
    asteroids.iter()
             .map(|p| get_num_angles(&p, &asteroids))
             .max()
             .unwrap()
}

fn get_num_angles(ast: &Point, asteroids: &Vec<Point>) -> usize {
    asteroids.iter()
             .filter(|&p| p != ast) //skip myself
             .map(|p| get_angle(&ast, p))
             .collect::<HashSet<i32>>()
             .len()
}

fn get_angle(p1: &Point, p2: &Point) -> i32 {
    let dx = (p2.y - p1.y) as f32;
    let dy = (p2.x - p1.x) as f32;
    let angle = f32::atan2(dy, dx) * 180.0 / f32::consts::PI;
    (angle * 10.0).round() as i32
}

fn read_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn get_asteroids(input: &str) -> Vec<Point> {
    input.trim()
          .lines()
          .enumerate()
          .map(|(y, line)| {
              line.chars()
                  .enumerate()
                  .filter(|(_x, c)| *c == '#')
                  .map(|(x, _c)| Point::new(x as i16, y as i16))
                  .collect()
          })
          .flat_map(|vec: Vec<Point>| vec.into_iter())
          .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = ".#..#
.....
#####
....#
...##";
        let asteroids = get_asteroids(input);
        let output = 8;
        let res1 = get_best_count(&asteroids);
        assert_eq!(res1, output);
    }

    #[test]
    fn test_2() {
        let input = "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";
        let asteroids = get_asteroids(input);
        let output = 33;
        let res1 = get_best_count(&asteroids);
        assert_eq!(res1, output);
    }

    #[test]
    fn test_3() {
        let input = "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.";
        let asteroids = get_asteroids(input);
        let output = 35;
        let res1 = get_best_count(&asteroids);
        assert_eq!(res1, output);
    }

    #[test]
    fn test_4() {
        let input = ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..";
        let asteroids = get_asteroids(input);
        let output = 41;
        let res1 = get_best_count(&asteroids);
        assert_eq!(res1, output);
    }
    #[test]
    fn test_5() {
        let input = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";
        let asteroids = get_asteroids(input);
        let output = 210;
        let res1 = get_best_count(&asteroids);
        assert_eq!(res1, output);
    }
}
