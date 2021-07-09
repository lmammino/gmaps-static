use std::fmt;

use super::IconAnchor;

#[derive(Clone)]
pub struct Icon<S: AsRef<str> + Clone> {
    anchor: Option<IconAnchor>,
    url: S,
}

impl<S: AsRef<str> + Clone> Icon<S> {
    pub fn new(url: S) -> Self {
        Icon { anchor: None, url }
    }

    pub fn position(mut self, anchor: IconAnchor) -> Self {
        self.anchor = Some(anchor);
        self
    }
}

impl<S: AsRef<str> + Clone> fmt::Display for Icon<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parts: Vec<String> = vec![];

        if let Some(anchor) = &self.anchor {
            parts.push(format!("anchor:{}", anchor.to_string()));
        }

        parts.push(format!("icon:{}", self.url.as_ref()));

        write!(f, "{}", parts.join("|"))
    }
}
