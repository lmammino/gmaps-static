use crate::RgbColor;
use std::fmt;

pub const RGBA_BLACK: RgbaColor = RgbaColor::Black;
pub const RGBA_BROWN: RgbaColor = RgbaColor::Brown;
pub const RGBA_GREEN: RgbaColor = RgbaColor::Green;
pub const RGBA_PURPLE: RgbaColor = RgbaColor::Purple;
pub const RGBA_YELLOW: RgbaColor = RgbaColor::Yellow;
pub const RGBA_BLUE: RgbaColor = RgbaColor::Blue;
pub const RGBA_GRAY: RgbaColor = RgbaColor::Gray;
pub const RGBA_ORANGE: RgbaColor = RgbaColor::Orange;
pub const RGBA_RED: RgbaColor = RgbaColor::Red;
pub const RGBA_WHITE: RgbaColor = RgbaColor::White;
pub const RGBA_TRANSPARENT: RgbaColor = RgbaColor::Custom(0, 0, 0, 0);

#[derive(Clone)]
pub enum RgbaColor {
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
    Custom(u8, u8, u8, u8),
}

impl RgbaColor {
    pub fn new(r: u8, g: u8, b: u8, alpha: u8) -> Self {
        RgbaColor::Custom(r, g, b, alpha)
    }
}

impl fmt::Display for RgbaColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use RgbaColor::*;

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
            Custom(r, g, b, alpha) => write!(f, "0x{:02x}{:02x}{:02x}{:02x}", r, g, b, alpha),
        }
    }
}

impl From<(u8, u8, u8)> for RgbaColor {
    fn from(rgb: (u8, u8, u8)) -> Self {
        let (r, g, b) = rgb;
        RgbaColor::new(r, g, b, 255)
    }
}

impl From<(i32, i32, i32)> for RgbaColor {
    fn from(rgb: (i32, i32, i32)) -> Self {
        let (r, g, b) = rgb;
        RgbaColor::new(r as u8, g as u8, b as u8, 255)
    }
}

impl From<(u8, u8, u8, u8)> for RgbaColor {
    fn from(rgba: (u8, u8, u8, u8)) -> Self {
        let (r, g, b, alpha) = rgba;
        RgbaColor::new(r, g, b, alpha)
    }
}

impl From<(i32, i32, i32, i32)> for RgbaColor {
    fn from(rgba: (i32, i32, i32, i32)) -> Self {
        let (r, g, b, alpha) = rgba;
        RgbaColor::new(r as u8, g as u8, b as u8, alpha as u8)
    }
}

impl From<RgbColor> for RgbaColor {
    fn from(color: RgbColor) -> Self {
        match color {
            RgbColor::Black => RgbaColor::Black,
            RgbColor::Brown => RgbaColor::Brown,
            RgbColor::Green => RgbaColor::Green,
            RgbColor::Purple => RgbaColor::Purple,
            RgbColor::Yellow => RgbaColor::Yellow,
            RgbColor::Blue => RgbaColor::Blue,
            RgbColor::Gray => RgbaColor::Gray,
            RgbColor::Orange => RgbaColor::Orange,
            RgbColor::Red => RgbaColor::Red,
            RgbColor::White => RgbaColor::White,
            RgbColor::Custom(r, g, b) => RgbaColor::Custom(r, g, b, 255_u8),
        }
    }
}
