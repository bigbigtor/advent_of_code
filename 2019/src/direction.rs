#[derive(Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
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
