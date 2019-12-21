use std::collections::HashMap;
use crate::point::Point;
use crate::direction::Direction;
use crate::computer::{Computer, Status};

pub struct Robot {
    brain: Computer,
    current_pos: Point,
    direction: Direction,
    grid: HashMap<Point, Color>,
}

impl Robot {
    pub fn new() -> Robot {
        Robot {
            brain: Computer::new(),
            current_pos: Point::new(0, 0),
            direction: Direction::North,
            grid: HashMap::new(),
        }
    }

    pub fn run(&mut self, input: &Vec<i64>) -> usize {
        self.brain.load(&input);
        loop {
            let status = self.brain.run();
            if status == Status::AwaitingInput {
                let input = match self.scan_panel() {
                    Color::Black => 0,
                    Color::White => 1,
                };
                self.brain.input.push(input);
            } else if status == Status::ReturningOutput {
                let target_color = match self.brain.output.remove(0) {
                    0 => Color::Black,
                    1 => Color::White,
                    o => panic!("invalid color: {}", o),
                };
                self.paint_panel(&target_color);
                if self.brain.run() == Status::ReturningOutput {
                    let turn_dir = self.brain.output.remove(0);
                    self.turn(turn_dir);
                    self.advance(1);
                } else {
                    panic!("expected direction {:?}", self.brain.dump_memory());
                }
            } else if status == Status::Halt {
                break;
            }
        }
        self.grid.len()
    }

    fn advance(&mut self, num_steps: i16) {
        let (x, y) = match self.direction {
            Direction::North => (0, num_steps),
            Direction::East => (num_steps, 0),
            Direction::South => (0, -num_steps),
            Direction::West => (-num_steps, 0),
        };
        self.current_pos = self.current_pos.translate((x, y));
    }

    fn turn(&mut self, turn_dir: i64) {
        self.direction = match turn_dir {
            0 => Direction::get_next_anticlockwise(self.direction),
            1 => Direction::get_next_clockwise(self.direction),
            o => panic!("invalid turn direction {}", o),
        };
    }

    fn paint_panel(&mut self, color: &Color) {
        let p = Point::new(self.current_pos.x, self.current_pos.y);
        if let Some(c) = self.grid.get_mut(&p) {
            *c = *color;
        } else {
            self.grid.insert(p, *color);
        }
    }

    fn scan_panel(&mut self) -> Color {
        let p = Point::new(self.current_pos.x, self.current_pos.y);
        self.grid.entry(p).or_insert(Color::Black).clone()
    }
}

#[derive(Copy, Clone)]
enum Color {
    Black,
    White,
}
