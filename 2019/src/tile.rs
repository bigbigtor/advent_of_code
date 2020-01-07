#[derive(Clone, Eq, PartialEq)]
pub enum Tile {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
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


