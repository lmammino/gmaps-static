use std::fmt;

#[derive(Clone)]
pub struct Size {
    width: i32,
    height: i32,
}

impl Size {
    pub fn new(width: i32, height: i32) -> Self {
        let w = if width <= 0 { 1 } else { width };
        let h = if height <= 0 { 1 } else { height };
        Size {
            width: w,
            height: h,
        }
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

impl From<(i32, i32)> for Size {
    fn from(s: (i32, i32)) -> Self {
        Size::new(s.0, s.1)
    }
}
