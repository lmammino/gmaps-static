use std::fmt;

pub const BLACK: MarkerColor = MarkerColor::Black;
pub const BROWN: MarkerColor = MarkerColor::Brown;
pub const GREEN: MarkerColor = MarkerColor::Green;
pub const PURPLE: MarkerColor = MarkerColor::Purple;
pub const YELLOW: MarkerColor = MarkerColor::Yellow;
pub const BLUE: MarkerColor = MarkerColor::Blue;
pub const GRAY: MarkerColor = MarkerColor::Gray;
pub const ORANGE: MarkerColor = MarkerColor::Orange;
pub const RED: MarkerColor = MarkerColor::Red;
pub const WHITE: MarkerColor = MarkerColor::White;

#[derive(Clone)]
pub enum MarkerColor {
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
    Rgb(u8, u8, u8),
}

impl MarkerColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        MarkerColor::Rgb(r, g, b)
    }
}

impl fmt::Display for MarkerColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use MarkerColor::*;

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
            Rgb(r, g, b) => write!(f, "0x{:02x}{:02x}{:02x}", r, g, b,),
        }
    }
}

impl From<(u8, u8, u8)> for MarkerColor {
    fn from(rgb: (u8, u8, u8)) -> Self {
        let (r, g, b) = rgb;
        MarkerColor::new(r, g, b)
    }
}

impl From<(i32, i32, i32)> for MarkerColor {
    fn from(rgb: (i32, i32, i32)) -> Self {
        let (r, g, b) = rgb;
        MarkerColor::new(r as u8, g as u8, b as u8)
    }
}
