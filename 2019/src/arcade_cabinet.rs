use crate::computer::{Computer, Status};
use crate::tile::Tile;

pub struct ArcadeCabinet {
    computer: Computer,
    screen: Vec<Tile>,
    width: usize,
}

impl ArcadeCabinet {
    pub fn new() -> ArcadeCabinet {
        ArcadeCabinet {
            computer: Computer::new(),
            screen: Vec::new(),
            width: 0,
        }
    }

    pub fn load_game(&mut self, game: &Vec<i64>) {
        self.computer.load(game);
    }

    pub fn start_game(&mut self) {
        while self.computer.run() != Status::Halt {
            continue;
        }
        let tiles = self.computer
                        .output
                        .chunks(3)
                        .collect::<Vec<_>>();
        self.width = tiles.iter()
                          .map(|v| v[0])
                          .max()
                          .unwrap() as usize;
        let height = tiles.iter()
                          .map(|v| v[1])
                          .max()
                          .unwrap() as usize;
        let screen_size = (self.width + 1) * (height + 1);
        self.screen = vec![Tile::Empty; screen_size];
        for v in tiles.iter() {
             let idx = v[0] as usize + (self.width * v[1] as usize);
             self.screen[idx] = Tile::from(v[2]);
        }
        let blocks = self.screen.iter()
                                .filter(|&t| *t == Tile::Block)
                                .collect::<Vec<_>>()
                                .len();
        println!("{}", blocks);
    }
}
