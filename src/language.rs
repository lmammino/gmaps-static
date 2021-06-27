use crate::QueryStringable;

#[derive(Clone)]
pub struct Language(&'static str);

impl QueryStringable for Language {
    fn as_query_params(&self) -> Vec<(String, String)> {
        vec![("language".to_string(), self.0.to_string())]
    }
}

impl From<&'static str> for Language {
    fn from(language: &'static str) -> Self {
        Language(language)
    }
}
