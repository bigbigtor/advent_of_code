use std::io::{self, Read};
use std::collections::HashSet;
use std::f32;

fn main() -> io::Result<()> {
    let input = read_input()?;
    let asteroids = get_asteroids(&input);
    let res1 = get_best_count(&asteroids);
    println!("{}", res1);
    Ok(())
}

fn get_best_count(asteroids: &Vec<(usize, usize)>) -> usize {
    asteroids.iter()
             .map(|(x, y)| get_num_angles(*x, *y, &asteroids))
             .max()
             .unwrap()
}

fn get_num_angles(x: usize, y: usize, asteroids: &Vec<(usize, usize)>) -> usize {
    asteroids.iter()
             .filter(|(tx, ty)| (*tx, *ty) != (x, y)) //skip myself
             .map(|(tx, ty)| get_angle(x, y, *tx, *ty))
             .collect::<HashSet<i32>>()
             .len()
}

fn get_angle(x1: usize, y1: usize, x2: usize, y2: usize) -> i32 {
    let dx = (y2 as i32 - y1 as i32) as f32;
    let dy = (x2 as i32 - x1 as i32) as f32;
    let angle = f32::atan2(dy, dx) * 180.0 / f32::consts::PI;
    (angle * 10.0).round() as i32
}

fn read_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn get_asteroids(input: &str) -> Vec<(usize, usize)> {
    input.trim()
          .lines()
          .enumerate()
          .map(|(y, line)| {
              line.chars()
                  .enumerate()
                  .filter(|(_x, c)| *c == '#')
                  .map(|(x, _c)| (x, y))
                  .collect()
          })
          .flat_map(|vec: Vec<(usize, usize)>| vec.into_iter())
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
        let map = get_asteroids(input);
        let output = 8;
        let res1 = get_best_count(&map);
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
        let map = get_asteroids(input);
        let output = 33;
        let res1 = get_best_count(&map);
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
        let map = get_asteroids(input);
        let output = 35;
        let res1 = get_best_count(&map);
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
        let map = get_asteroids(input);
        let output = 41;
        let res1 = get_best_count(&map);
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
        let map = get_asteroids(input);
        let output = 210;
        let res1 = get_best_count(&map);
        assert_eq!(res1, output);
    }
}
