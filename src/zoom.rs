#[derive(Clone)]
pub struct Zoom(u8);

impl Zoom {
    pub fn new(zoom: u8) -> Self {
        let clamp_zoom = if zoom > 21 { 21 } else { zoom };
        Zoom(clamp_zoom)
    }

    pub fn World() -> Self {
        Zoom(1)
    }

    pub fn Continent() -> Self {
        Zoom(5)
    }

    pub fn City() -> Self {
        Zoom(10)
    }

    pub fn Streets() -> Self {
        Zoom(15)
    }

    pub fn Buildings() -> Self {
        Zoom(20)
    }
}

impl ToString for Zoom {
    fn to_string(&self) -> String {
        self.0.to_string()
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
