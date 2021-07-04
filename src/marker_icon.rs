use crate::IconAnchor;
use std::fmt;

#[derive(Clone)]
pub struct MarkerIcon<S: AsRef<str> + Clone> {
    anchor: Option<IconAnchor>,
    url: S,
}

impl<S: AsRef<str> + Clone> MarkerIcon<S> {
    pub fn new(url: S) -> Self {
        MarkerIcon { anchor: None, url }
    }

    pub fn position(&self, anchor: IconAnchor) -> Self {
        MarkerIcon {
            anchor: Some(anchor),
            ..self.clone()
        }
    }
}

impl<S: AsRef<str> + Clone> fmt::Display for MarkerIcon<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parts: Vec<String> = vec![];

        if let Some(anchor) = &self.anchor {
            parts.push(format!("anchor:{}", anchor.to_string()));
        }

        parts.push(format!("icon:{}", self.url.as_ref()));

        write!(f, "{}", parts.join("|"))
    }
}
