use crate::direction::Direction;
use crate::diagnostic::Diagnostic;
use crate::point::Point;
use crate::computer::Computer;
use crate::map::Map;

pub struct RepairDroid {
    brain: Computer,
    map: Map,
}

impl RepairDroid {
    pub fn new() -> RepairDroid {
        RepairDroid {
            brain: Computer::new(),
            map: Map::new(),
        }
    }

    pub fn load(&mut self, program: &Vec<i64>) {
        self.brain.load(&program);
    }

    pub fn run(&mut self) -> Option<Point> {
        let current = Point::new(0, 0);
        self.map.insert(current, Diagnostic::Moved);
        Direction::iter()
                  .filter_map(|&dir| self.step(dir, current))
                  .next()
    }

    fn step(&mut self, dir: Direction, origin: Point) -> Option<Point> {
        self.brain.run();
        self.brain.input.push(dir as i64);
        self.brain.run();
        let diag = Diagnostic::from(self.brain.output.remove(0));
        let current = origin.step(dir);
        self.map.insert(current, diag);
        println!("{}", self.map);
        match diag {
            Diagnostic::HitAWall => None,
            Diagnostic::FoundTarget => Some(current),
            Diagnostic::Moved => Direction::iter()
                                           .filter_map(|&d|
                                               match self.map.contains_key(&current.step(d)) {
                                                   true => None,
                                                   false => self.step(d, current),
                                               }
                                           ).next(),
        }
    }
}
