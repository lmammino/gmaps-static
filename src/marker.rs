use crate::{Location, MarkerAppearence, MarkerScale};
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

#[cfg(test)]
mod tests {
    use crate::BLUE;

    use super::*;
    use crate::MarkerStyle;

    #[test]
    fn it_builds_a_complete_style() {
        let marker_appearence: MarkerAppearence<String> =
            MarkerStyle::new().color(BLUE).label('S'.into()).into();
        let style = Marker::new()
            .appearence(marker_appearence)
            .add_location("11211".into())
            .add_location("11206".into())
            .add_location("11222".into());
        assert_eq!("color:blue|label:S|11211|11206|11222", style.to_string());
    }
}
