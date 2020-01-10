use std::fmt;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Tile {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display = match self {
            Tile::Empty => " ",
            Tile::Wall => "|",
            Tile::Block => "#",
            Tile::HorizontalPaddle => "_",
            Tile::Ball => "o",
        };
        write!(f, "{}", display)
    }
}

impl From<i64> for Tile {
    fn from(val: i64) -> Tile {
        match val {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::HorizontalPaddle,
            4 => Tile::Ball,
            o => panic!("Unexpected value for tile: {}", o),
        }
    }
}


