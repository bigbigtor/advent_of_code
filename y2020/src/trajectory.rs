use num::Integer;

#[derive(PartialEq)]
pub enum Content {
    Tree,
    Empty,
}

impl From<char> for Content {
    fn from(c: char) -> Self {
        match c {
            '#' => Self::Tree,
            '.' => Self::Empty,
            o => panic!(format!("Cannot parse Content from char [{}]", o))
        }
    }
}

pub type Slope = (usize, usize);

pub struct Map {
    grid: Vec<Content>,
    width: usize,                                                                                                                                           
}

impl Map {
    pub fn new(grid: Vec<Content>, width: usize) -> Map {
        Map { grid, width }
    }
                                                                                                                        
    pub fn get_trees_in_trajectory_for(&self, slope: Slope) -> usize {
        (1..(self.grid.len().div_ceil(&self.width) / slope.1)).into_iter()
        .map(|i| &self.grid[(i * slope.1 * self.width) + (i * slope.0 % self.width)])
        .filter(|&content| content == &Content::Tree)
        .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_trees_in_trajectory_for_works() {
        let raw_input = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
        let slope = (3, 1);
        let map = parse_map(raw_input);
        assert_eq!(map.get_trees_in_trajectory_for(slope), 7);
    }

    #[test]
    fn day3_part1() {
        let raw_input = ".......#..#....#...#...#......#
..##..#...##.###.#..#.....#.#..
#..#.#....#......#..#.........#
.#..##...#........#....#..#..#.
#.#.#....###...#........#.....#
.#...#.#.##.#.##...#.#.........
####......#.......###.##.#.....
..#...........#...#.#.#........
.#.......#....###.####..#......
...##........#....##.......##..
.###......##.#......##....#.#.#
........#.#......##...#......#.
#....##.#..#...#.......#.......
.#..##........##.........#....#
.#..#..#...#....#.#......#.#...
..#.#......##.#.......#....##..
......##......#.#..##.#..#...#.
.....##.......#.#....#.#.......
........#.....#.....#..###.#...
#........#..#.....#...#.#.#..#.
.#..#.....#...#........#.....#.
.#.#.....#.....#...#...........
.....#.#..#..#...#..#..#..##..#
##.#...#....#..#.##..#.....#.#.
#.......####......#..#.#....#..
......#.#...####.........#.#..#
.#.........#..#.#...#..........
...#####.#....#.#..#......#.#.#
##....#.###....##...##..#.....#
...........####.##.#....##.##..
#.#.#..........#.#..##.#.######
##...#..#...........###..#....#
.#.#.#...##..........##.#...#..
...#.#........#..##...#....#...
......#..#...#..##....#.......#
.#..#.......#..#......##....##.
.......#.......#........#..##..
...#...#...#.##......#.##.#....
.........#.........#.#.#.##....
..#...................#....#..#
.........#..#.....#.#...#....#.
#.#.#...#........#..###.#......
#.#.#.####......##...#...#....#
#...........##..#.#.#....#..#..
........#..#.#...........##.#.#
.#.........#...........#..#....
#............##.#..#....##...##
.#....##..#.#....#.......#..#..
..#.#...#.#......####.......#..
...#.#.......###......#.....#..
#......#.......#.#...#.#..##...
...#.....#...##.#.....#.#......
#.#.#............#..#......#..#
....#...#...##.##.##...##.#....
..##........#..#........#...##.
.......#..#...#.........#.....#
...........#.#......#...#......
...##..##..##..###..#..#..#..#.
#..##.......##..#....#....#.#..
#.#.##.#..##.....#....#.#......
....#..##......#.#..#....#....#
.#.#.........##...#......##.##.
##...........#..#.....#.###....
.#.###........#...#....##..#...
......##.....#.................
.#.##..#.#.......#......#.#.#..
.#...#....#.##..........##.##..
#...##......####.#....#....#...
.#...#.##.#.#.....#...#........
.#................#.##.#.###...
...#.#..#.#.....##.....##....#.
..##.#..#..##.....#....#...#.##
........###.##..#..###.....#..#
..##.....#.......#.#...##......
#.#..###...##.###.##.#..#...#..
#..#..#.#...#....#...##.....#.#
#..................#........#..
#.....#.......#.##....##....#..
...#.............#.....#...#...
...#...#.##..##.....#........#.
.......#........##....###..##..
.#....#....#.#..#......#....#.#
..........#..#.#.....##...#.##.
.#...##.#...........#.#.......#
..#.##.....#.###.#.............
..#....###..........#.#.#......
#.....#.####..#.#......#..#.#.#
...#........#..#...............
.###.#.##.....#.#...........#..
..#....#..#....#..##....#......
......#..#.....#.#.##.......#.#
###..#...#.#..#....#..##.###..#
.#....##.###........##...##.#.#
........##..##.#....##..#....#.
...#..#....#.#....#...#...##...
#.....#......#.##........#....#
....#....###.##...#.#.##....#..
......#.##..#.#..........#...#.
...........#...#....##...#....#
......#.#.........#....#.#.#...
.###..........#.###.##....#...#
...##.......#......#....#....#.
#..#...#.#..####...#......#..#.
....##..#.#.........#..........
.##.###.##....##.####....#...#.
..##.......#........#...#..#...
....#####..........###....#....
.#.#..#.#.#....#..#............
........#.....#....#.......##..
...........##....##..##.....##.
..###........#.#.#..#....##...#
.....#...........##......#..#..
...##........#.##.#......##..#.
##..#....#............##..#..#.
.#.....#...##.##..............#
#..##........#...#...#......##.
......##.....#.......####.##..#
...#.#....#...#..#.............
..#...#..##.###..#..#.......##.
##....###.......#...#..#.......
#..#.....###.....#.#.........#.
#.#....#.............#...#.....
..#.#.##..........#.....##.#...
.....##......#..#..#.....#..#..
##.#..#..#.##......###....#..#.
...#............##...#..##.....
.#..#....#.........#......#.##.
.##.##...#..............#..#.##
...#....#...###...#...#....#..#
..#...#..####..#....#.#...##..#
..............##.##.......##...
..##.#..##...........#.#.#...#.
..................##.####.###..
.#...........#.......#......#..
.#.#.#...#....#.........##...##
....#..........#.#....#.#.....#
..........#.#..........#.#.....
...........#.....#.#......#....
........#..#.#.#.#.............
...###...##...##..####.##......
.#..#......###.....#...#.....#.
.........##............#.#.....
#.#..#.#.#....###.#.#..#..#..##
..........#...#.##.#..#..#....#
#..#.......##....#..##........#
##.#...#....##.............#...
....#........#......##..#..#.##
.................#.#.#.#.#.....
...........#.#.....#.......#...
#.......#.......#............#.
....#...........#.#.##.....#..#
#...#.....#....#..##...#.......
..#.....#.....#.##.##....#.....
.#.#..#...#..#..##.....##..#...
.#.#....#.........####.........
#...#..####.....#...#..##......
..#...##.#.....#...#.....##....
.#...#.....#.#.#......#.......#
..#.....##.#..#.#...##.........
##.#...#..#....#....#.##.##...#
.#..#....#..##.#.......#..#....
...##.#......#...###.......#...
...#..#.........##.####........
#.#..#..##...........#..#......
.#...#.#......#.#..........#...
...###...#.......#.....#.#...##
..#....#.#.##..........##...#..
.....###.........#.....#..##..#
.......##.....#.#.....#.#..##..
.#.#.###..##.......##...#......
......#.....#................##
.#......##..##.#.#...#...#...##
.#...#......#.......#.#........
.#..........###...#..#...#.....
.........##.....#.#..#..#.#...#
#...#...#.........#..#..#....#.
###.......#.#.....#....##......
.#..#......#..#...........#..#.
..##....##..##...#......#......
.#........#....#...#....#.....#
.#.......#...#...#..##.#.#..#..
#...#........#.##.....#.....#..
#..##.....#..........#...#...##
............#...............#..
.#.##...#.....#.#..#..#..#.....
.#.#.#...#........#....#...##..
##......#.....#.###.#...#.#..#.
.........##..#..#.#...#...#...#
#...#.#....#..#..#.....#.......
.......#.###...#.............#.
..#.....#.#.#..###.#....#.....#
....#...#.#....#.#..........#..
..#......#.###.#.#..#.....#...#
#............#..##...##......#.
#...........#..#....#.###..###.
.#.##.#.#.......#.............#
..............#................
..#.#.....#.....#...#......#...
.#.#.#..#..#.#...........##....
.....##.#......#..#.##....#....
.......##..#.#.#..#............
..#.....#.....#.###..#.....#.#.
......##.....#..##.#...#.....#.
...#...#....#..#..#........#...
..#.##..#....#.........#.#..#..
#....#.....###.....#......#....
##.....#..#..##.........#.##.##
.#.#....#.#..........#.........
.##.#...#..#.......#.##...#....
...#...#.....#....#...#.#..#...
.....#....#.....#.....#.#......
...........#.#.......#.......#.
.........##.###.##........#....
#..##.....#...#.#..............
.#...#....##........#.#..#....#
..#...#........#...#..#.##.#..#
........#...#.....##.#.#....#.#
#..#.......###.#....#.#.#......
.......#...##....#...#..##..#..
.....##........#.#.#..#....##..
.#....#..#.#...........#......#
...##....#.##.....##.......#...
.##..#..#....#.#....#..#....##.
..#....#.....###.......#..##..#
....#.......#....##..#....#..##
....#......##..#....#.#...#.#..
.##.#......##..................
##.#....#........#..#..#...##.#
.......#..#.#...##.....#.#.....
..##.#...........#.#.#..#.#.#..
.....#....#......#..#.......#..
#.#...#.####..##.......#..##...
...#....#.....#.##.#..#.##..#..
.#.......#......##........##.#.
.......#.#...#..#...#..##.#....
.#....#........#.#.....##..#..#
#..#.....#..#.............#...#
#...#....#..#...###..#...#.#...
.#..#.....#..........#..##.####
#.#.#.#.##.#.#.....##.#........
...#....##....#...#..##.......#
..##.##.#.#........#..........#
..###........###..#..........#.
...#......#..##.#........#..#..
#.#.#..#........#..#..........#
...#........#..##.#...#.###....
##......#.####.#....#......#...
.#..#......#................#..
#.#........#.#.....##.....##...
#...............#..#.......#.#.
.##..#...........##..#..#.#....
#......#.#.......#.#.#.##..#.##
.....##.#..###.............##..
....##.........#..#...#........
.....#.....#.#.#..#.#..........
#.........#....##.#.##.....#..#
.#.........#......#.#.##.#.#...
##.........#.....#..#.#..#.##.#
....#......##...#.....#..#..###
..#..............#...#..####...
#....#...##.#.......#...#..#...
#.......###.#.#.......#.......#
...##....#.#...........#...###.
...........#..#.#.....#..##..#.
..#.........#..###..#.....#...#
..#.#.....#.#.#...#.#.#......#.
........#.....#.#......##....##
##.#.#...#.#........#.....#...#
........#....#...............#.
##.###......####...#####..#....
...##...#..#....#........#...#.
...###.#..................##.#.
##.#.......###.......#...#.#...
....#..#.#...#...#....#.#.#..##
....#...........#..#...........
#..#.#..#...#...#..#...........
...#...#.#....#..#....#........
#....#.......#.##........#..#..
.....#...#..#................#.
#......#.......#..........##..#
.#....#.#......#.#...#....##..#
...#.##...#......#.#...##...##.
..#...#..##...#...#....#.......
.....#....#.#.#..........#.#...
...#...#..#....#..#.#..........
......#.#..........##.......#..
.#...##.#.#...#..##..#...#.....
..#..#.........#........#.#.#..
#.#..##..#.....##......#.....#.
#..#.....#.#....#...#.#....#.#.
......#........##.#..#...#.....
...#.##.#.#......#.#..##...#..#
....#..###..#..#.....###....##.
.....#...#.#.....#..........#.#
.#...##..##.....#..#...#.#.#...
.##.#......##...##..#...#.....#
.#.##....#...#.##.#.#...#.#...#
....#.#...#....###.#.....#.....
#.....####................#..#.
....#.....#...#.#.......##.#...
.#...##.#...#..#...........#.#.
..#####..#.#...#...##........#.
...#...##........#...#.#....###
........#.#.#..#.....#.......#.
...#...#..##............##.....
#.#..###....###.#...#.#...##.##
..#.##...#......#..#.........##
.##..#..#.....#..#.........#.#.
.#..#.#....#.##...#..#.##....##
..#...#.#...##.#.#...#...#....#
#..........#.......##..##....#.
#...###.#......#....#.........#
#.....#...##.......##....##....
.##.#..#.##......#.##....#..#..
............#.#....##.#..#....#
.#.........##.##...#....#.....#
##....##..#..#....##...#.....##
...#.....#...........#.....##..
......#...#.........#.......#..
............#...##.#.....#.#.#.
.#........##..........#.....#.#
.###.........#.....#.##...#....
.##..#...##...#..#..#.##.......";
        let slope = (3, 1);
        let map = parse_map(raw_input);
        assert_eq!(map.get_trees_in_trajectory_for(slope), 234);
    }

    fn parse_map(raw_input: &str) -> Map {
        let width = raw_input.find("\n").unwrap();
        let grid = raw_input.lines()
        .flat_map(|line| line.chars().map(|ch| Content::from(ch)))
        .collect();
        Map::new(grid, width)
    }
}