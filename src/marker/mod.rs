mod appearance;
mod icon;
mod icon_anchor;
mod label;
mod position;
mod scale;
mod size;
mod style;

pub use appearance::*;
pub use icon::*;
pub use icon_anchor::*;
pub use label::*;
pub use position::*;
pub use scale::*;
pub use size::*;
pub use style::*;

use crate::{Location, QueryStringable, RgbColor};
use std::fmt;

#[derive(Clone)]
pub struct Marker {
    appearence: Option<Appearence>,
    scale: Option<Scale>,
    locations: Vec<Location>,
}

impl Marker {
    pub fn new() -> Self {
        Marker {
            appearence: None,
            scale: None,
            locations: vec![],
        }
    }

    pub fn simple(color: RgbColor, label: char, location: Location) -> Self {
        let marker_style = Style::new().color(color).label(label.into());
        Marker::new()
            .appearence(marker_style.into())
            .add_location(location)
    }

    pub fn locations(mut self, locations: Vec<Location>) -> Marker {
        self.locations = locations;
        self
    }

    pub fn add_location(mut self, location: Location) -> Marker {
        self.locations.push(location);
        self
    }

    pub fn appearence(mut self, appearence: Appearence) -> Marker {
        self.appearence = Some(appearence);
        self
    }

    pub fn scale(mut self, scale: Scale) -> Marker {
        self.scale = Some(scale);
        self
    }
}

impl<'a> Default for Marker {
    fn default() -> Self {
        Marker::new()
    }
}

impl<'a> From<Location> for Marker {
    fn from(location: Location) -> Self {
        Marker {
            appearence: None,
            scale: None,
            locations: vec![location],
        }
    }
}

impl fmt::Display for Marker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parts: Vec<String> = vec![];

        if let Some(scale) = &self.scale {
            parts.push(scale.to_string());
        }

        if let Some(appearence) = &self.appearence {
            parts.push(appearence.to_string());
        }

        for location in &self.locations {
            parts.push(location.to_string());
        }

        write!(f, "{}", parts.join("|"))
    }
}

impl QueryStringable for Marker {
    fn as_query_params(&self) -> Vec<(String, String)> {
        vec![("markers".to_string(), self.to_string())]
    }
}

#[cfg(test)]
mod tests {
    use crate::{marker::Icon, marker::Style, RGB_BLUE};

    use super::*;

    #[test]
    fn it_builds_a_complete_style() {
        let marker_appearence: Appearence = Style::new().color(RGB_BLUE).label('S'.into()).into();
        let marker = Marker::new()
            .appearence(marker_appearence)
            .add_location("11211".into())
            .add_location("11206".into())
            .add_location("11222".into());
        assert_eq!("color:blue|label:S|11211|11206|11222", marker.to_string());
    }

    #[test]
    fn it_builds_a_complete_style_2() {
        let marker_appearence: Appearence = Icon::new("https://goo.gl/5y3S82")
            .anchor((32, 10).into())
            .into();
        let marker = Marker::new()
            .appearence(marker_appearence)
            .add_location("Canberra ACT".into());
        assert_eq!(
            "anchor:32,10|icon:https://goo.gl/5y3S82|Canberra ACT",
            marker.to_string()
        );
    }
}
