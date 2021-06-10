use crate::{Color, Location, MarkerAppearence, MarkerScale, MarkerStyle, QueryStringable};
use std::fmt;

#[derive(Clone)]
pub struct Marker<S: AsRef<str> + Clone> {
    appearence: Option<MarkerAppearence<S>>,
    scale: Option<MarkerScale>,
    locations: Vec<Location>,
}

impl<S: AsRef<str> + Clone> Marker<S> {
    pub fn new() -> Self {
        Marker {
            appearence: None,
            scale: None,
            locations: vec![],
        }
    }

    pub fn simple(color: &'static Color, label: char, location: Location) -> Self {
        let marker_style = MarkerStyle::new().color(color).label(label.into());
        Marker::new()
            .appearence(marker_style.into())
            .add_location(location)
    }

    pub fn add_location(&self, location: Location) -> Marker<S> {
        let mut new_marker = self.clone();
        new_marker.locations.push(location);
        new_marker
    }

    pub fn appearence(&self, appearence: MarkerAppearence<S>) -> Marker<S> {
        Marker {
            appearence: Some(appearence),
            ..(*self).clone()
        }
    }

    pub fn scale(&self, scale: MarkerScale) -> Marker<S> {
        Marker {
            scale: Some(scale),
            ..(*self).clone()
        }
    }
}

impl<S: AsRef<str> + Clone> Default for Marker<S> {
    fn default() -> Self {
        Marker::new()
    }
}

impl<S: AsRef<str> + Clone> From<Location> for Marker<S> {
    fn from(location: Location) -> Self {
        Marker {
            appearence: None,
            scale: None,
            locations: vec![location],
        }
    }
}

impl<S: AsRef<str> + Clone> fmt::Display for Marker<S> {
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

impl<S: AsRef<str> + Clone> QueryStringable for Marker<S> {
    fn as_query_params(&self) -> Vec<(String, String)> {
        vec![("markers".to_string(), self.to_string())]
    }
}

#[cfg(test)]
mod tests {
    use crate::{MarkerIcon, BLUE};

    use super::*;
    use crate::MarkerStyle;

    #[test]
    fn it_builds_a_complete_style() {
        let marker_appearence: MarkerAppearence<String> =
            MarkerStyle::new().color(BLUE).label('S'.into()).into();
        let marker = Marker::new()
            .appearence(marker_appearence)
            .add_location("11211".into())
            .add_location("11206".into())
            .add_location("11222".into());
        assert_eq!("color:blue|label:S|11211|11206|11222", marker.to_string());
    }

    #[test]
    fn it_builds_a_complete_style_2() {
        let marker_appearence: MarkerAppearence<&str> = MarkerIcon::new("https://goo.gl/5y3S82")
            .position((32, 10).into())
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
