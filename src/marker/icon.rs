use std::fmt;

use super::IconAnchor;

#[derive(Clone)]
pub struct Icon {
    anchor: Option<IconAnchor>,
    url: String,
}

impl Icon {
    pub fn new<S: AsRef<str>>(url: S) -> Self {
        Icon {
            anchor: None,
            url: String::from(url.as_ref()),
        }
    }

    pub fn with_anchor<S: AsRef<str>>(url: S, anchor: IconAnchor) -> Self {
        Icon {
            anchor: Some(anchor),
            url: String::from(url.as_ref()),
        }
    }

    pub fn anchor(mut self, anchor: IconAnchor) -> Self {
        self.anchor = Some(anchor);
        self
    }
}

impl fmt::Display for Icon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parts: Vec<String> = vec![];

        if let Some(anchor) = &self.anchor {
            parts.push(format!("anchor:{}", anchor));
        }

        parts.push(format!("icon:{}", self.url));

        write!(f, "{}", parts.join("|"))
    }
}
