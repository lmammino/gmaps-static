use crate::{Color, MarkerLabel, MarkerSize};
use std::fmt;

#[derive(Clone)]
pub struct MarkerStyle {
    size: Option<&'static MarkerSize>,
    color: Option<&'static Color>,
    label: Option<MarkerLabel>,
}

impl MarkerStyle {
    pub fn new() -> Self {
        MarkerStyle {
            size: None,
            color: None,
            label: None,
        }
    }

    pub fn size(&self, size: &'static MarkerSize) -> Self {
        MarkerStyle {
            size: Some(size),
            ..(*self).clone()
        }
    }

    pub fn color(&self, color: &'static Color) -> Self {
        MarkerStyle {
            color: Some(color),
            ..(*self).clone()
        }
    }

    pub fn label(&self, label: MarkerLabel) -> Self {
        MarkerStyle {
            label: Some(label),
            ..(*self).clone()
        }
    }
}

impl Default for MarkerStyle {
    fn default() -> Self {
        MarkerStyle::new()
    }
}

impl fmt::Display for MarkerStyle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parts: Vec<String> = vec![];

        if let Some(size) = &self.size {
            parts.push(format!("size:{}", size))
        }

        if let Some(color) = &self.color {
            parts.push(format!("color:{}", color));
        }

        if let Some(label) = &self.label {
            parts.push(format!("label:{}", label))
        }

        write!(f, "{}", parts.join("|"))
    }
}

#[cfg(test)]
mod tests {
    use crate::{BLUE, MID};

    use super::*;

    #[test]
    fn it_builds_a_complete_style() {
        let style = MarkerStyle::new().color(BLUE).label('C'.into()).size(MID);
        assert_eq!("size:mid|color:blue|label:C", style.to_string());
    }
}
