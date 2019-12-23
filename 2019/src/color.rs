use std::fmt;

#[derive(Copy, Clone, Debug)]
pub enum Color {
    Black,
    White,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display = match self {
            Color::Black => '.',
            Color::White => '#',
        };
        write!(f, "{}", display)
    }
}
