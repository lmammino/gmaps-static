use std::fmt;

pub static ROADMAP: &MapType = &MapType::RoadMap;
pub static SATELLITE: &MapType = &MapType::Satellite;
pub static TERRAIN: &MapType = &MapType::Terrain;
pub static HYBRID: &MapType = &MapType::Hybrid;

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
