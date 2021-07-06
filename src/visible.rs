use crate::{Location, QueryStringable};
use std::fmt;

#[derive(Clone)]
pub struct Visible(Location);

impl From<(f32, f32)> for Visible {
    fn from(lat_lng: (f32, f32)) -> Self {
        let (lat, lng) = lat_lng;
        Visible((lat as f64, lng as f64).into())
    }
}

impl From<(f64, f64)> for Visible {
    fn from(lat_lng: (f64, f64)) -> Self {
        Visible(lat_lng.into())
    }
}

impl From<String> for Visible {
    fn from(address: String) -> Self {
        Visible(address.into())
    }
}

impl<'a> From<&'a str> for Visible {
    fn from(address: &'a str) -> Self {
        Visible(address.into())
    }
}

impl fmt::Display for Visible {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl QueryStringable for Visible {
    fn as_query_params(&self) -> Vec<(String, String)> {
        vec![(String::from("visible"), self.to_string())]
    }
}
