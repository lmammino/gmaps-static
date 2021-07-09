use crate::QueryStringable;

#[derive(Clone)]
pub struct MapId<S: AsRef<str> + Clone>(S);

impl<S: AsRef<str> + Clone> QueryStringable for MapId<S> {
    fn as_query_params(&self) -> Vec<(String, String)> {
        vec![(String::from("map_id"), String::from(self.0.as_ref()))]
    }
}

impl<S: AsRef<str> + Clone> From<S> for MapId<S> {
    fn from(id: S) -> Self {
        MapId(id)
    }
}
