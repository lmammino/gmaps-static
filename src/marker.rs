use crate::{Location, MarkerStyle};
use std::fmt;

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

impl fmt::Display for Marker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parts: Vec<String> = vec![];

        if let Some(style) = &self.style {
            parts.push(style.to_string());
        }

        for location in &self.locations {
            parts.push(location.to_string());
        }

        write!(f, "{}", parts.join("|"))
    }
}

#[cfg(test)]
mod tests {
    use crate::BLUE;

    use super::*;

    #[test]
    fn it_builds_a_complete_style() {
        let style = Marker::new()
            .style(MarkerStyle::new().color(BLUE).label('S'.into()))
            .add_location("11211".into())
            .add_location("11206".into())
            .add_location("11222".into());
        assert_eq!("color:blue|label:S|11211|11206|11222", style.to_string());
    }
}
