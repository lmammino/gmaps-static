use crate::QueryStringable;

#[derive(Clone)]
pub struct Region(&'static str);

impl QueryStringable for Region {
    fn as_query_params(&self) -> Vec<(String, String)> {
        vec![("region".to_string(), self.0.to_string())]
    }
}

impl From<&'static str> for Region {
    fn from(region: &'static str) -> Self {
        Region(region)
    }
}
