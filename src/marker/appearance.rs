use super::{Icon, Style};
use std::fmt;

#[derive(Clone)]
pub enum Appearence {
    Icon(Icon),
    Styled(Style),
}

impl From<Icon> for Appearence {
    fn from(marker_icon: Icon) -> Self {
        Appearence::Icon(marker_icon)
    }
}

impl From<Style> for Appearence {
    fn from(marker_style: Style) -> Self {
        Appearence::Styled(marker_style)
    }
}

impl fmt::Display for Appearence {
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
