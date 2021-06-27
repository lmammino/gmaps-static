use crate::QueryStringable;
use std::fmt;

pub const ROADMAP: MapType = MapType::RoadMap;
pub const SATELLITE: MapType = MapType::Satellite;
pub const TERRAIN: MapType = MapType::Terrain;
pub const HYBRID: MapType = MapType::Hybrid;

#[derive(Clone, Copy)]
pub enum MapType {
    RoadMap,
    Satellite,
    Terrain,
    Hybrid,
}

impl fmt::Display for MapType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use MapType::*;
        write!(
            f,
            "{}",
            match self {
                RoadMap => String::from("roadmap"),
                Satellite => String::from("satellite"),
                Terrain => String::from("terrain"),
                Hybrid => String::from("hybrid"),
            }
        )
    }
}

impl QueryStringable for MapType {
    fn as_query_params(&self) -> Vec<(String, String)> {
        vec![("maptype".to_string(), self.to_string())]
    }
}
