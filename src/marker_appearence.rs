use crate::{MarkerIcon, MarkerStyle};
use std::fmt;

#[derive(Clone)]
pub enum MarkerAppearence<S: AsRef<str> + Clone> {
    Icon(MarkerIcon<S>),
    Styled(MarkerStyle),
}

impl<S: AsRef<str> + Clone> From<MarkerIcon<S>> for MarkerAppearence<S> {
    fn from(marker_icon: MarkerIcon<S>) -> Self {
        MarkerAppearence::Icon(marker_icon)
    }
}

impl<S: AsRef<str> + Clone> From<MarkerStyle> for MarkerAppearence<S> {
    fn from(marker_style: MarkerStyle) -> Self {
        MarkerAppearence::Styled(marker_style)
    }
}

impl<S: AsRef<str> + Clone> fmt::Display for MarkerAppearence<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use MarkerAppearence::*;
        write!(
            f,
            "{}",
            match &self {
                Icon(i) => i.to_string(),
                Styled(s) => s.to_string(),
            }
        )
    }
}
