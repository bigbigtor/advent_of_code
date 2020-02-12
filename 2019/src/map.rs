use crate::point::Point;
use crate::diagnostic::Diagnostic;
use std::collections::HashMap;
use std::fmt;
use std::ops::Deref;
use std::ops::DerefMut;

pub struct Map(HashMap<Point, Diagnostic>);

impl Map {
    pub fn new() -> Map {
        Map {
            0: HashMap::new(),
        }
    }
}

impl Deref for Map {
    type Target = HashMap<Point, Diagnostic>;
    fn deref(&self) -> &HashMap<Point, Diagnostic> {
        &self.0
    }
}

impl DerefMut for Map {
    fn deref_mut(&mut self) -> &mut HashMap<Point, Diagnostic> {
        &mut self.0
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x_min = self.keys()
                        .map(|p| p.x)
                        .min()
                        .unwrap();
        let x_max = self.keys()
                        .map(|p| p.x)
                        .max()
                        .unwrap();
        let y_min = self.keys()
                        .map(|p| p.y)
                        .min()
                        .unwrap();
        let y_max = self.keys()
                        .map(|p| p.y)
                        .max()
                        .unwrap();
        for y in (y_min..=y_max).rev() {
            for x in x_min..=x_max {
                let p = Point::new(x, y);
                if let Some(diag) = self.get(&p) {
                    write!(f, "{}", diag)?;
                } else {
                    write!(f, "{}", " ")?;
                }
            }
            write!(f, "{}", "\n")?;
        }
        Ok(())
    }
}
