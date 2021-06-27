use crate::QueryStringable;
use std::fmt;

#[derive(Clone)]
pub struct Zoom(u8);

pub const WORLD: Zoom = Zoom(1);
pub const ZOOM_1: Zoom = WORLD; // alias
pub const ZOOM_2: Zoom = Zoom(2);
pub const ZOOM_3: Zoom = Zoom(3);
pub const ZOOM_4: Zoom = Zoom(4);
pub const CONTINENT: Zoom = Zoom(5);
pub const ZOOM_5: Zoom = CONTINENT; // alias
pub const ZOOM_6: Zoom = Zoom(6);
pub const ZOOM_7: Zoom = Zoom(7);
pub const ZOOM_8: Zoom = Zoom(8);
pub const ZOOM_9: Zoom = Zoom(9);
pub const CITY: Zoom = Zoom(10);
pub const ZOOM_10: Zoom = CITY; // alias
pub const ZOOM_11: Zoom = Zoom(11);
pub const ZOOM_12: Zoom = Zoom(12);
pub const ZOOM_13: Zoom = Zoom(13);
pub const ZOOM_14: Zoom = Zoom(14);
pub const STREETS: Zoom = Zoom(15);
pub const ZOOM_15: Zoom = STREETS; // alias
pub const ZOOM_16: Zoom = Zoom(16);
pub const ZOOM_17: Zoom = Zoom(17);
pub const ZOOM_18: Zoom = Zoom(18);
pub const ZOOM_19: Zoom = Zoom(19);
pub const BUILDINGS: Zoom = Zoom(20);
pub const ZOOM_20: Zoom = BUILDINGS; // alias
pub const ZOOM_21: Zoom = Zoom(21);

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

impl QueryStringable for Zoom {
    fn as_query_params(&self) -> Vec<(String, String)> {
        vec![("zoom".to_string(), self.to_string())]
    }
}
