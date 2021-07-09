use std::fmt;

use super::Position;

#[derive(Clone)]
pub enum IconAnchor {
    Relative(Position),
    Absolute(i32, i32),
}

impl From<Position> for IconAnchor {
    fn from(position: Position) -> Self {
        IconAnchor::Relative(position)
    }
}

impl From<(i32, i32)> for IconAnchor {
    fn from((x, y): (i32, i32)) -> Self {
        IconAnchor::Absolute(x, y)
    }
}

impl fmt::Display for IconAnchor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use IconAnchor::*;
        match self {
            Relative(position) => write!(f, "{}", position.to_string()),
            Absolute(x, y) => write!(f, "{},{}", x, y),
        }
    }
}
