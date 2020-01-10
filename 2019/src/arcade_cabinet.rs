use crate::computer::{Computer, Status};
use crate::tile::Tile;
use crate::joystick::Joystick;
use crate::screen::Screen;
use crate::point::Point;
use std::{thread, time, cmp};

pub struct ArcadeCabinet {
    computer: Computer,
    screen: Screen,
    score: u64,
    joystick: Joystick,
    ball_pos: Point,
    paddle_pos: Point,
}

impl ArcadeCabinet {
    pub fn new() -> ArcadeCabinet {
        ArcadeCabinet {
            computer: Computer::new(),
            screen: Screen::new(0, 0),
            score: 0,
            joystick: Joystick::Neutral,
            ball_pos: Point::new(0, 0),
            paddle_pos: Point::new(0, 0),
        }
    }

    pub fn load_game(&mut self, game: &Vec<i64>) {
        self.computer.load(game);
    }

    pub fn initialize_game(&mut self) {
        while self.computer.run() == Status::ReturningOutput {
            continue;
        }
        let tiles = self.computer
                        .output
                        .chunks(3)
                        .collect::<Vec<_>>();
        let max_x = tiles.iter()
                          .map(|v| v[0])
                          .max()
                          .unwrap() as usize;
        let max_y = tiles.iter()
                          .map(|v| v[1])
                          .max()
                          .unwrap() as usize;
        let screen_size = (max_x + 1) * (max_y + 1);
        self.screen = Screen::new(screen_size, max_x + 1);
        self.handle_output();
    }

    pub fn start_game(&mut self) {
        loop {
            //let ms = time::Duration::from_millis(20);
            //thread::sleep(ms);
            match self.computer.run() {
                Status::AwaitingInput => {
                    self.joystick = match self.ball_pos.x.cmp(&self.paddle_pos.x) {
                         cmp::Ordering::Less => Joystick::TiltedLeft,
                         cmp::Ordering::Equal => Joystick::Neutral,
                         cmp::Ordering::Greater => Joystick::TiltedRight,
                    };
                    let input = self.joystick.read_input();
                    self.computer.input.push(input);
                },
                Status::ReturningOutput => self.handle_output(),
                Status::Halt => break,
            };
            println!("{}", self.score);
            println!("{}", self.screen);
        }
    }

    fn handle_output(&mut self) {
        while self.computer.output.len() > 2 {
            let (x, y, tile) = (
                self.computer.output.remove(0),
                self.computer.output.remove(0),
                self.computer.output.remove(0)
            );
            if x == -1 && y == 0 {
                self.score = tile as u64;
            } else {
                let tile = Tile::from(tile);
                self.screen.set_tile(x as usize, y as usize, &tile);
                match tile {
                    Tile::Ball => self.ball_pos = Point::new(x as i16, y as i16),
                    Tile::HorizontalPaddle => self.paddle_pos = Point::new(x as i16, y as i16),
                    _ => (),
                }
            }
        }
    }
}
