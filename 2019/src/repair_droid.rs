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

    pub fn get_target_position(&self) -> Option<Point> {
        self.map.iter()
                .find(|(_k, &v)| v == Diagnostic::FoundTarget)
                .map(|(k, _v)| *k)
    }

    pub fn map_area(&mut self) {
        self.step(Direction::North, Point::new(0, 0));
    }

    fn step(&mut self, dir: Direction, origin: Point) {
        let diag = self.get_diagnostic(dir);
        let current = origin.step(dir);
        self.map.insert(current, diag);
        if diag == Diagnostic::FoundTarget ||
           diag == Diagnostic::Moved {
               for &d in Direction::iter() {
                   if !self.map.contains_key(&current.step(d)) {
                       self.step(d, current);
                   }
               }
        }
        println!("{}", self.map);
    }

    fn get_diagnostic(&mut self, dir: Direction) -> Diagnostic {
        self.brain.run();
        self.brain.input.push(dir as i64);
        self.brain.run();
        Diagnostic::from(self.brain.output.remove(0))
    }
}
