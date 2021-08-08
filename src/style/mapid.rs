use crate::QueryStringable;

#[derive(Clone)]
pub struct MapId(String);

impl QueryStringable for MapId {
    fn as_query_params(&self) -> Vec<(String, String)> {
        vec![(String::from("map_id"), self.0.clone())]
    }
}

impl<S: AsRef<str>> From<S> for MapId {
    fn from(id: S) -> Self {
        MapId(String::from(id.as_ref()))
    }
}
