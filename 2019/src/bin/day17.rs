use advent_of_code_2019::computer::{Computer, Status};
use advent_of_code_2019::direction::Direction;
use advent_of_code_2019::point::Point;
use std::collections::HashMap;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut ascii_program = read_input()?;
    let mut computer = Computer::new();
    computer.load(&ascii_program);
    while computer.run() == Status::ReturningOutput {
        continue;
    }
    //display_map(&computer.output);
    let map = populate_map(&computer.output);
    let crossings = find_crossings(&map);
    let res1 = crossings.iter().map(|p| p.x * p.y).sum::<i16>();
    println!("{:?}", res1);
    //println!("{}", serialize_path(&map));
    ascii_program[0] = 2;
    computer.load(&ascii_program);
    println!("{}", solve_part_2(&mut computer));
    Ok(())
}

fn solve_part_2(computer: &mut Computer) -> i64 {
    "A,B,A,A,B,C,B,C,C,B\n"
        .chars()
        .map(|c| c as i64)
        .for_each(|c| computer.input.push(c));
    "L,12,R,8,L,6,R,8,L,6\n"
        .chars()
        .map(|c| c as i64)
        .for_each(|c| computer.input.push(c));
    "R,8,L,12,L,12,R,8\n"
        .chars()
        .map(|c| c as i64)
        .for_each(|c| computer.input.push(c));
    "L,6,R,6,L,12\n"
        .chars()
        .map(|c| c as i64)
        .for_each(|c| computer.input.push(c));
    "n\n"
        .chars()
        .map(|c| c as i64)
        .for_each(|c| computer.input.push(c));
    while computer.run() != Status::Halt {
        continue;
    }
    computer.output.pop().unwrap()
}

fn serialize_path(map: &HashMap<Point, i64>) -> String {
    let mut result = String::new();
    let (init_point, init_dir) = map
        .iter()
        .map(|(k, v)| (k, Direction::try_from(*v)))
        .find(|(_point, dir)| dir.is_some())
        .map(|(point, dir)| (*point, dir.unwrap()))
        .unwrap();
    let mut cur_point = init_point;
    let mut cur_dir = init_dir;
    let mut step_count = 0;
    loop {
        let front = cur_point.step(cur_dir);
        let left = cur_point.step(Direction::get_next_anticlockwise(cur_dir));
        let right = cur_point.step(Direction::get_next_clockwise(cur_dir));
        if map.get(&front) == Some(&35) {
            cur_point = front;
            step_count += 1;
        } else if map.get(&left) == Some(&35) {
            cur_dir = Direction::get_next_anticlockwise(cur_dir);
            if step_count > 0 {
                result.push_str(&step_count.to_string());
                result.push(',');
                step_count = 0;
            }
            result.push_str("L,");
        } else if map.get(&right) == Some(&35) {
            cur_dir = Direction::get_next_clockwise(cur_dir);
            if step_count > 0 {
                result.push_str(&step_count.to_string());
                result.push(',');
                step_count = 0;
            }
            result.push_str("R,");
        } else {
            result.push_str(&step_count.to_string());
            break;
        }
    }
    result
}

fn populate_map(output: &Vec<i64>) -> HashMap<Point, i64> {
    let width = output.iter().position(|&d| d == 10).unwrap() + 1;
    output
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let point = Point::new((i % width) as i16, (i / width) as i16);
            (point, *c)
        })
        .collect()
}

fn display_map(output: &Vec<i64>) {
    let display = output
        .iter()
        .map(|&c| (c as u8) as char)
        .collect::<String>();
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
                .len()
                == 4
        })
        .map(|(k, _n)| *k)
        .collect::<Vec<_>>()
}

fn read_input() -> io::Result<Vec<i64>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let input = buffer
        .trim()
        .split(",")
        .filter_map(|s| s.parse().ok())
        .collect();
    Ok(input)
}
