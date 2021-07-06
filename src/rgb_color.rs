use std::fmt;

pub const RGB_BLACK: RgbColor = RgbColor::Black;
pub const RGB_BROWN: RgbColor = RgbColor::Brown;
pub const RGB_GREEN: RgbColor = RgbColor::Green;
pub const RGB_PURPLE: RgbColor = RgbColor::Purple;
pub const RGB_YELLOW: RgbColor = RgbColor::Yellow;
pub const RGB_BLUE: RgbColor = RgbColor::Blue;
pub const RGB_GRAY: RgbColor = RgbColor::Gray;
pub const RGB_ORANGE: RgbColor = RgbColor::Orange;
pub const RGB_RED: RgbColor = RgbColor::Red;
pub const RGB_WHITE: RgbColor = RgbColor::White;

#[derive(Clone)]
pub enum RgbColor {
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
    Custom(u8, u8, u8),
}

impl RgbColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        RgbColor::Custom(r, g, b)
    }
}

impl fmt::Display for RgbColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use RgbColor::*;

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
            Custom(r, g, b) => write!(f, "0x{:02x}{:02x}{:02x}", r, g, b,),
        }
    }
}

impl From<(u8, u8, u8)> for RgbColor {
    fn from(rgb: (u8, u8, u8)) -> Self {
        let (r, g, b) = rgb;
        RgbColor::new(r, g, b)
    }
}

impl From<(i32, i32, i32)> for RgbColor {
    fn from(rgb: (i32, i32, i32)) -> Self {
        let (r, g, b) = rgb;
        RgbColor::new(r as u8, g as u8, b as u8)
    }
}
