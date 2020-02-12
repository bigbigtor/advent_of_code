use crate::direction::Direction;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

impl Point {
    pub fn new(x: i16, y: i16) -> Point {
        Point {
            x: x,
            y: y,
        }
    }

    pub fn step(&self, dir: Direction) -> Point {
        let (x, y) = match dir {
            Direction::North => ( 0,  1),
            Direction::South => ( 0, -1),
            Direction::West  => (-1,  0),
            Direction::East  => ( 1,  0),
        };
        self.translate((x, y))
    }

    pub fn translate(&self, (x, y): (i16, i16)) -> Point {
        Point::new(self.x + x, self.y + y)
    }

    pub fn get_manhattan_distance(a: &Point, b: &Point) -> i16 {
        Point::get_absolute_distance(a.x, b.x)
            + Point::get_absolute_distance(a.y, b.y)
    }

    fn get_absolute_distance(a: i16, b: i16) -> i16 {
        let res = a - b;
        if res < 0 { -res } else { res }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manhattan_1() {
        let p1 = Point::new(0,0);
        let p2 = Point::new(3,3);
        let output = 6;
        assert_eq!(Point::get_manhattan_distance(&p1, &p2), output);
    }

    #[test]
    fn test_manhattan_2() {
        let p1 = Point::new(0,0);
        let p2 = Point::new(-3,-3);
        let output = 6;
        assert_eq!(Point::get_manhattan_distance(&p1, &p2), output);
    }
}
