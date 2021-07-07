use crate::{RgbColor, StyleRuleVisibility};

#[derive(Clone)]
pub struct StyleRule {
    hue: Option<RgbColor>,
    lightness: Option<f32>,
    saturation: Option<f32>,
    gamma: Option<f32>,
    invert_lightness: bool,
    visibility: Option<StyleRuleVisibility>,
    color: Option<RgbColor>,
    weight: Option<u8>,
}

impl StyleRule {
    pub fn new() -> Self {
        StyleRule {
            hue: None,
            lightness: None,
            saturation: None,
            gamma: None,
            invert_lightness: false,
            visibility: None,
            color: None,
            weight: None,
        }
    }

    /// Set the hue (an RGB hex string of format #RRGGBB) that indicates the basic color.
    ///
    /// **Note**: This option sets the hue while keeping the saturation and lightness specified in
    /// the default Google style (or in other style options you define on the map). The resulting
    /// color is relative to the style of the base map. If Google makes any changes to the base map
    /// style, the changes affect your map's features styled with hue. It's better to use the
    /// absolute color styler if you can.
    pub fn hue(mut self, hue: RgbColor) -> Self {
        self.hue = Some(hue);
        self
    }
}

impl Default for StyleRule {
    fn default() -> Self {
        StyleRule::new()
    }
}
