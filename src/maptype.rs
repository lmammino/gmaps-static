#[derive(Clone, Copy)]
pub enum MapType {
    RoadMap,
    Satellite,
    Terrain,
    Hybrid,
}

impl ToString for MapType {
    fn to_string(&self) -> String {
        use MapType::*;
        match self {
            RoadMap => String::from("roadmap"),
            Satellite => String::from("satellite"),
            Terrain => String::from("terrain"),
            Hybrid => String::from("hybrid"),
        }
    }
}
