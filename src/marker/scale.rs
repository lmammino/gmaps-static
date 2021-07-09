use std::fmt;

pub const MARKER_SCALE1: Scale = Scale::Scale1;
pub const MARKER_SCALE2: Scale = Scale::Scale2;
pub const MARKER_SCALE4: Scale = Scale::Scale4;

#[derive(Clone)]
pub enum Scale {
    Scale1,
    Scale2,
    Scale4,
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Scale::*;
        write!(
            f,
            "scale:{}",
            match self {
                Scale1 => "1",
                Scale2 => "2",
                Scale4 => "4",
            }
        )
    }
}
