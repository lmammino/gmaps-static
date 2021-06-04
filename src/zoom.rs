use std::fmt;

#[derive(Clone)]
pub struct Zoom(u8);

pub static WORLD: &Zoom = &Zoom(1);
pub static ZOOM_1: &Zoom = WORLD; // alias
pub static ZOOM_2: &Zoom = &Zoom(2);
pub static ZOOM_3: &Zoom = &Zoom(3);
pub static ZOOM_4: &Zoom = &Zoom(4);
pub static CONTINENT: &Zoom = &Zoom(5);
pub static ZOOM_5: &Zoom = CONTINENT; // alias
pub static ZOOM_6: &Zoom = &Zoom(6);
pub static ZOOM_7: &Zoom = &Zoom(7);
pub static ZOOM_8: &Zoom = &Zoom(8);
pub static ZOOM_9: &Zoom = &Zoom(9);
pub static CITY: &Zoom = &Zoom(10);
pub static ZOOM_10: &Zoom = CITY; // alias
pub static ZOOM_11: &Zoom = &Zoom(11);
pub static ZOOM_12: &Zoom = &Zoom(12);
pub static ZOOM_13: &Zoom = &Zoom(13);
pub static ZOOM_14: &Zoom = &Zoom(14);
pub static STREETS: &Zoom = &Zoom(15);
pub static ZOOM_15: &Zoom = STREETS; // alias
pub static ZOOM_16: &Zoom = &Zoom(16);
pub static ZOOM_17: &Zoom = &Zoom(17);
pub static ZOOM_18: &Zoom = &Zoom(18);
pub static ZOOM_19: &Zoom = &Zoom(19);
pub static BUILDINGS: &Zoom = &Zoom(20);
pub static ZOOM_20: &Zoom = BUILDINGS; // alias
pub static ZOOM_21: &Zoom = &Zoom(21);

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
