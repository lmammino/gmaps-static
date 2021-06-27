use crate::{Location, QueryStringable};

pub type Center = Location;

impl QueryStringable for Center {
    fn as_query_params(&self) -> Vec<(String, String)> {
        vec![("center".to_string(), self.to_string())]
    }
}
