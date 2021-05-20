use std::fmt;

#[derive(Clone)]
pub enum Color {
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

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color::Rgb(r, g, b)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Color::*;

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

impl From<(u8, u8, u8)> for Color {
    fn from(rgb: (u8, u8, u8)) -> Self {
        let (r, g, b) = rgb;
        Color::new(r, g, b)
    }
}

impl From<(i32, i32, i32)> for Color {
    fn from(rgb: (i32, i32, i32)) -> Self {
        let (r, g, b) = rgb;
        Color::new(r as u8, g as u8, b as u8)
    }
}
