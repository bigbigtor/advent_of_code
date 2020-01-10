pub enum Joystick {
    Neutral,
    TiltedLeft,
    TiltedRight,
}

impl Joystick {
    pub fn read_input(&self) -> i64 {
        match self {
            Joystick::Neutral => 0,
            Joystick::TiltedLeft => -1,
            Joystick::TiltedRight => 1,
        }
    }
}
