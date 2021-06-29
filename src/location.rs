use std::fmt;

#[derive(Clone)]
pub enum Location {
    Address(String),
    LatLng(f64, f64),
}

impl Location {
    pub fn from_address(address: String) -> Self {
        Location::Address(address)
    }

    pub fn from_lat_lng(lat: f64, lng: f64) -> Self {
        let clamp_lat = if lat < -90.0 {
            -90.0
        } else if lat > 90.0 {
            90.0
        } else {
            lat
        };

        let clamp_lng = if lng < -180.0 {
            -180.0
        } else if lng > 180.0 {
            180.0
        } else {
            lng
        };

        Location::LatLng(clamp_lat, clamp_lng)
    }
}

impl From<(f32, f32)> for Location {
    fn from(lat_lng: (f32, f32)) -> Self {
        let (lat, lng) = lat_lng;
        Location::from_lat_lng(lat as f64, lng as f64)
    }
}

impl From<(f64, f64)> for Location {
    fn from(lat_lng: (f64, f64)) -> Self {
        let (lat, lng) = lat_lng;
        Location::from_lat_lng(lat, lng)
    }
}

impl From<String> for Location {
    fn from(address: String) -> Self {
        Location::from_address(address)
    }
}

impl<'a> From<&'a str> for Location {
    fn from(address: &'a str) -> Self {
        Location::from_address(String::from(address))
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Location::*;
        write!(
            f,
            "{}",
            match self {
                Address(s) => s.clone(),
                LatLng(lat, lng) => format!("{:.6},{:.6}", lat, lng),
            }
        )
    }
}
