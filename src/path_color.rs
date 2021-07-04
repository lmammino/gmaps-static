use std::fmt;

pub const PATH_BLACK: PathColor = PathColor::Black;
pub const PATH_BROWN: PathColor = PathColor::Brown;
pub const PATH_GREEN: PathColor = PathColor::Green;
pub const PATH_PURPLE: PathColor = PathColor::Purple;
pub const PATH_YELLOW: PathColor = PathColor::Yellow;
pub const PATH_BLUE: PathColor = PathColor::Blue;
pub const PATH_GRAY: PathColor = PathColor::Gray;
pub const PATH_ORANGE: PathColor = PathColor::Orange;
pub const PATH_RED: PathColor = PathColor::Red;
pub const PATH_WHITE: PathColor = PathColor::White;
pub const PATH_TRANSPARENT: PathColor = PathColor::Rgb(0, 0, 0, 0);

#[derive(Clone)]
pub enum PathColor {
    Black,
    Brown,
    Green,
    Purple,
    Yellow,
    Blue,
    Gray,
    Orange,
    Red,
    White,
    Rgb(u8, u8, u8, u8),
}

impl PathColor {
    pub fn new(r: u8, g: u8, b: u8, alpha: u8) -> Self {
        PathColor::Rgb(r, g, b, alpha)
    }
}

impl fmt::Display for PathColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use PathColor::*;

        match &self {
            Black => write!(f, "black"),
            Brown => write!(f, "brown"),
            Green => write!(f, "green"),
            Purple => write!(f, "purple"),
            Yellow => write!(f, "yellow"),
            Blue => write!(f, "blue"),
            Gray => write!(f, "gray"),
            Orange => write!(f, "orange"),
            Red => write!(f, "red"),
            White => write!(f, "white"),
            Rgb(r, g, b, alpha) => write!(f, "0x{:02x}{:02x}{:02x}{:02x}", r, g, b, alpha),
        }
    }
}

impl From<(u8, u8, u8)> for PathColor {
    fn from(rgb: (u8, u8, u8)) -> Self {
        let (r, g, b) = rgb;
        PathColor::new(r, g, b, 255)
    }
}

impl From<(i32, i32, i32)> for PathColor {
    fn from(rgb: (i32, i32, i32)) -> Self {
        let (r, g, b) = rgb;
        PathColor::new(r as u8, g as u8, b as u8, 255)
    }
}

impl From<(u8, u8, u8, u8)> for PathColor {
    fn from(rgba: (u8, u8, u8, u8)) -> Self {
        let (r, g, b, alpha) = rgba;
        PathColor::new(r, g, b, alpha)
    }
}

impl From<(i32, i32, i32, i32)> for PathColor {
    fn from(rgba: (i32, i32, i32, i32)) -> Self {
        let (r, g, b, alpha) = rgba;
        PathColor::new(r as u8, g as u8, b as u8, alpha as u8)
    }
}
