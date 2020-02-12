use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Diagnostic {
    HitAWall,
    Moved,
    FoundTarget,
}

impl From<i64> for Diagnostic {
    fn from(input: i64) -> Diagnostic {
        match input {
            0 => Diagnostic::HitAWall,
            1 => Diagnostic::Moved,
            2 => Diagnostic::FoundTarget,
            o => panic!("invalid value for diagnostic: {}", o),
        }
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display = match self {
            Diagnostic::HitAWall => '#',
            Diagnostic::Moved => '.',
            Diagnostic::FoundTarget => 'x',
        };
        write!(f, "{}", display)
    }
}
