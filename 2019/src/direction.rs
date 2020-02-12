use std::slice::Iter;

#[derive(Copy, Clone)]
pub enum Direction {
    North = 1,
    South = 2,
    West  = 3,
    East  = 4,
}

impl Direction {
    pub fn iter() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [Direction::North, Direction::South, Direction::West, Direction::East];
        DIRECTIONS.iter()
    }

    pub fn get_next_clockwise(dir: Direction) -> Direction {
        match dir {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn get_next_anticlockwise(dir: Direction) -> Direction {
        match dir {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
}
