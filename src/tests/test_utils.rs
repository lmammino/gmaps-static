use querystring::querify;
use url::Url;

#[cfg(test)]
pub fn qs_from_url(url: String) -> Vec<(String, String)> {
    let u = Url::parse(url.as_str()).expect("The given url is valid");
    let raw_query = u.query().expect("There is a querystring in the given URL");
    qs(raw_query)
}

#[cfg(test)]
pub fn qs(raw_query: &str) -> Vec<(String, String)> {
    let mut query_args = querify(raw_query);
    query_args.sort_unstable();
    query_args
        .iter()
        .map(|(key, value)| (String::from(*key), String::from(*value)))
        .collect()
}
