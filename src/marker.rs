use crate::{Location, MarkerStyle};

#[derive(Clone)]
pub struct Marker {
    style: Option<MarkerStyle>,
    locations: Vec<Location>,
}

impl Marker {
    pub fn new() -> Self {
        Marker {
            style: None,
            locations: vec![],
        }
    }

    pub fn add_location(&self, location: Location) -> Marker {
        let mut new_marker = self.clone();
        new_marker.locations.push(location);
        new_marker
    }

    pub fn style(&self, style: MarkerStyle) -> Marker {
        Marker {
            style: Some(style),
            ..(*self).clone()
        }
    }
}

impl Default for Marker {
    fn default() -> Self {
        Marker::new()
    }
}

impl From<Location> for Marker {
    fn from(location: Location) -> Marker {
        Marker {
            style: None,
            locations: vec![location],
        }
    }
}
