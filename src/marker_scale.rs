use std::fmt;

pub static MARKER_SCALE1: &MarkerScale = &MarkerScale::Scale1;
pub static MARKER_SCALE2: &MarkerScale = &MarkerScale::Scale2;
pub static MARKER_SCALE4: &MarkerScale = &MarkerScale::Scale4;

#[derive(Clone)]
pub enum MarkerScale {
    Scale1,
    Scale2,
    Scale4,
}

impl fmt::Display for MarkerScale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use MarkerScale::*;
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
