use crate::{Location, QueryStringable};

#[derive(Clone)]
pub struct Center(Location);

impl QueryStringable for Center {
    fn as_query_params(&self) -> Vec<(String, String)> {
        vec![("center".to_string(), self.0.to_string())]
    }
}

impl From<(f32, f32)> for Center {
    fn from(lat_lng: (f32, f32)) -> Self {
        Center(lat_lng.into())
    }
}

impl From<(f64, f64)> for Center {
    fn from(lat_lng: (f64, f64)) -> Self {
        Center(lat_lng.into())
    }
}

impl From<String> for Center {
    fn from(address: String) -> Self {
        Center(address.into())
    }
}

impl<'a> From<&'a str> for Center {
    fn from(address: &'a str) -> Self {
        Center(address.into())
    }
}
