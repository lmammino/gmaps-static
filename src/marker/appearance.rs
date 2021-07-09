use super::{Icon, Style};
use std::fmt;

#[derive(Clone)]
pub enum Appearence<S: AsRef<str> + Clone> {
    Icon(Icon<S>),
    Styled(Style),
}

impl<S: AsRef<str> + Clone> From<Icon<S>> for Appearence<S> {
    fn from(marker_icon: Icon<S>) -> Self {
        Appearence::Icon(marker_icon)
    }
}

impl<S: AsRef<str> + Clone> From<Style> for Appearence<S> {
    fn from(marker_style: Style) -> Self {
        Appearence::Styled(marker_style)
    }
}

impl<S: AsRef<str> + Clone> fmt::Display for Appearence<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Appearence::*;
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
