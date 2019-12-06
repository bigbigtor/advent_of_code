use std::io::{self};
use std::collections::{HashSet, HashMap};

use advent_of_code_2019::point::Point;

fn main() -> io::Result<()> {
    let path1 = read_line();
    let path2 = read_line();
    if let (Some(res1), Some(res2)) = get_solutions(&path1, &path2) {
        println!("{}\n{}", res1, res2);
    }
    Ok(())
}

fn get_solutions(path1: &Vec<String>, path2: &Vec<String>) -> (Option<i16>, Option<i32>) {
    let initial = Point::new(0,0);
    let visited1 = get_visited_points(&initial, path1);
    let visited2 = get_visited_points(&initial, path2);
    let visited_set1 = visited1.keys().cloned().collect::<HashSet<Point>>();
    let visited_set2 = visited2.keys().cloned().collect::<HashSet<Point>>();
    let intersection = visited_set1.intersection(&visited_set2);
    let res1 = intersection.clone().map(|x| Point::get_manhattan_distance(&initial, x)).min();
    let res2 = intersection.map(|x| visited1.get(x).unwrap() + visited2.get(x).unwrap()).min();
    (res1, res2)
}

fn get_visited_points(initial: &Point, path: &Vec<String>) -> HashMap<Point, i32> {
    let mut current_point = initial.clone();
    let mut current_steps = 0;
    let mut result = HashMap::new();
    path.iter()
        .map(|seg| parse_movements(seg))
        .flat_map(|vec| vec.into_iter())
        .for_each(|mov| {
            current_point = current_point.translate(mov);
            current_steps += 1;
            result.insert(current_point, current_steps);
        });
    result
}

fn parse_movements(s: &str) -> Vec<(i16, i16)> {
    let mut result = Vec::new();
    let (dir, dist) = s.split_at(1);
    let dist = if let Ok(d) = dist.parse::<i16>() { d } else { 0 };
    for _ in 0..dist {
        let tup = match dir {
            "U" => (0, 1),
            "D" => (0, -1),
            "L" => (-1, 0),
            "R" => (1, 0),
            o => panic!("Invalid direction: {}", o),
        };
        result.push(tup);
    }
    result
}

fn parse_input(input: &str) -> Vec<String> {
    input.trim()
         .split(",")
         .map(String::from)
         .collect()
}

fn read_line() -> Vec<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    parse_input(&buffer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution1() {
        let path1 = parse_input("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let path2 = parse_input("U62,R66,U55,R34,D71,R55,D58,R83");
        println!("{:?}", path1);
        println!("{:?}", path2);
        let output = (Some(159), Some(610));
        assert_eq!(get_solutions(&path1, &path2), output);
    }

    #[test]
    fn  test_solution2() {
        let path1 = parse_input("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let path2 = parse_input("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        println!("{:?}", path1);
        println!("{:?}", path2);
        let output = (Some(135), Some(410));
        assert_eq!(get_solutions(&path1, &path2), output);
    }
}
