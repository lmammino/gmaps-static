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

impl ToString for Color {
    fn to_string(&self) -> String {
        use Color::*;
        match &self {
            Black => String::from("black"),
            Brown => String::from("brown"),
            Green => String::from("green"),
            Purple => String::from("purple"),
            Yellow => String::from("yellow"),
            Blue => String::from("blue"),
            Gray => String::from("gray"),
            Orange => String::from("orange"),
            Red => String::from("red"),
            White => String::from("white"),
            Rgb(r, g, b) => {
                format!("0x{:02x}{:02x}{:02x}", r, g, b)
            }
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
