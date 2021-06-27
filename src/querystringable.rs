pub trait QueryStringable {
    fn as_query_params(&self) -> Vec<(String, String)>;
}

impl<T: QueryStringable> QueryStringable for Option<T> {
    fn as_query_params(&self) -> Vec<(String, String)> {
        match self {
            Some(v) => v.as_query_params(),
            None => vec![],
        }
    }
}

impl<T: QueryStringable> QueryStringable for Vec<T> {
    fn as_query_params(&self) -> Vec<(String, String)> {
        self.iter()
            .flat_map(|item| item.as_query_params())
            .collect()
    }
}
