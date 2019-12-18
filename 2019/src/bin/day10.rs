use std::io::{self, Read};
use std::collections::{HashSet, BTreeMap};
use std::f32;
use advent_of_code_2019::point::Point;

fn main() -> io::Result<()> {
    let input = read_input()?;
    let asteroids = get_asteroids(&input);
    let (best_asteroid, res1) = get_best_count(&asteroids);
    let destroyed = get_nth_destroyed_asteroid(&best_asteroid, &asteroids, 200);
    let res2 = (destroyed.x * 100) + destroyed.y;
    println!("{}\n{}", res1, res2);
    Ok(())
}

fn get_nth_destroyed_asteroid(ast: &Point, asteroids: &Vec<Point>, n: usize) -> Point {
    let mut map = get_angle2distance_map(&ast, &asteroids);
    map.iter_mut().for_each(|(_k, v)| v.sort_by_key(|(_p, d)| *d));
    let mut destroyed = 0;
    loop {
        for (_k, v) in map.iter_mut().rev() {
            if v.len() > 0 {
                let (latest, _) = v.remove(0);
                destroyed += 1;
                if destroyed == n { return latest; }
            }
        }
    }
}

fn get_angle2distance_map(ast: &Point, asteroids: &Vec<Point>) -> BTreeMap<i32, Vec<(Point, i16)>> {
    asteroids.iter()
             .filter(|&p| p != ast) //skip myself
             .map(|p| (p, get_angle(&ast, &p), Point::get_manhattan_distance(&ast, &p)))
             .fold(BTreeMap::new(), |mut acc, (p, a, d)| {
                 let entry = acc.entry(a).or_insert(Vec::new());
                 entry.push((*p, d));
                 acc
             })
}

fn get_best_count(asteroids: &Vec<Point>) -> (Point, usize) {
    asteroids.iter()
             .map(|p| (*p, get_num_angles(&p, &asteroids)))
             .max_by_key(|(_p, num)| *num)
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
        let (_, res1) = get_best_count(&asteroids);
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
        let (_, res1) = get_best_count(&asteroids);
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
        let (_, res1) = get_best_count(&asteroids);
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
        let (_, res1) = get_best_count(&asteroids);
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
        let output2 = 802;
        let (best, res1) = get_best_count(&asteroids);
        let destroyed = get_nth_destroyed_asteroid(&best, &asteroids, 200);
        let res2 = (destroyed.x * 100) + destroyed.y;
        assert_eq!(res1, output);
        assert_eq!(res2, output2);
    }
}
