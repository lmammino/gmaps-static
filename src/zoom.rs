use std::fmt;

#[derive(Clone)]
pub struct Zoom(u8);

pub static WORLD: &Zoom = &Zoom(1);
pub static CONTINENT: &Zoom = &Zoom(5);
pub static CITY: &Zoom = &Zoom(10);
pub static STREETS: &Zoom = &Zoom(15);
pub static BUILDINGS: &Zoom = &Zoom(20);

impl Zoom {
    pub fn new(zoom: u8) -> Self {
        let clamp_zoom = if zoom > 21 { 21 } else { zoom };
        Zoom(clamp_zoom)
    }
}

impl fmt::Display for Zoom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

impl From<u8> for Zoom {
    fn from(zoom: u8) -> Self {
        Zoom::new(zoom)
    }
}

impl From<i32> for Zoom {
    fn from(zoom: i32) -> Self {
        Zoom::new(zoom as u8)
    }
}
