use crate::{MarkerLabel, MarkerSize, RgbColor};
use std::fmt;

#[derive(Clone)]
pub struct MarkerStyle {
    size: Option<MarkerSize>,
    color: Option<RgbColor>,
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

    pub fn size(mut self, size: MarkerSize) -> Self {
        self.size = Some(size);
        self
    }

    pub fn color(mut self, color: RgbColor) -> Self {
        self.color = Some(color);
        self
    }

    pub fn label(mut self, label: MarkerLabel) -> Self {
        self.label = Some(label);
        self
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
    use crate::{MID, RGB_BLUE};

    use super::*;

    #[test]
    fn it_builds_a_complete_style() {
        let style = MarkerStyle::new()
            .color(RGB_BLUE)
            .label('C'.into())
            .size(MID);
        assert_eq!("size:mid|color:blue|label:C", style.to_string());
    }
}
