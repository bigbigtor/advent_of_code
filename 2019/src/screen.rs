use std::fmt;
use crate::tile::Tile;

pub struct Screen {
    tiles: Vec<Tile>,
    width: usize,
}

impl Screen {
    pub fn new(size: usize, width: usize) -> Screen {
        Screen {
            tiles: vec![Tile::Empty; size],
            width: width,
        }
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: &Tile) {
        let idx = x + (y * self.width);
        self.tiles[idx] = *tile;
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.tiles.chunks(self.width) {
            for tile in row.iter() {
                write!(f, "{}", tile)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
