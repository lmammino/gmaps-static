use crate::{RgbColor, StyleRuleVisibility};
/// Style rule transformations to create custom map styles.
///
/// Style rules are applied in the order that you specify.
/// Do not combine multiple operations into a single style operation.
/// Instead, define each operation as a separate entry in the style array.
///
/// **Note**: Order is important, as some operations are not commutative.
/// Features and/or elements that are modified through style operations (usually) already
/// have existing styles. The operations act on those existing styles, if present.
#[derive(Clone)]
pub enum StyleRule {
    /// Sets the hue that indicates the basic color.
    ///
    /// **Note**: This option sets the hue while keeping the saturation and lightness specified in
    /// the default Google style (or in other style options you define on the map). The resulting
    /// color is relative to the style of the base map. If Google makes any changes to the base map
    /// style, the changes affect your map's features styled with [`Self::Hue`]. It's better to use the
    /// absolute [`Self::Color`] styler if you can.
    Hue(RgbColor),

    /// Sets the lightness (a floating point value between `-100` and `100`) indicates the percentage
    /// change in brightness of the element. Negative values increase darkness (where `-100` specifies black)
    /// while positive values increase brightness (where `+100` specifies white).
    ///
    /// **Note**: This option sets the lightness while keeping the saturation and hue specified in the
    /// default Google style (or in other style options you define on the map). The resulting color is
    /// relative to the style of the base map. If Google makes any changes to the base map style, the
    /// changes affect your map's features styled with [`Self::Lightness`]. It's better to use the absolute
    /// [`Self::Color`] styler if you can.
    Lightness(f32),

    /// Sets the saturation (a floating point value between `-100` and `100`) indicates the percentage change
    /// in intensity of the basic color to apply to the element.
    ///
    /// **Note**: This option sets the saturation while keeping the hue and lightness specified in the
    /// default Google style (or in other style options you define on the map). The resulting color is
    /// relative to the style of the base map. If Google makes any changes to the base map style, the
    /// changes affect your map's features styled with [`Self::Saturation`]. It's better to use the absolute
    /// [`Self::Color`] styler if you can.
    Saturation(f32),

    /// Sets the gamma (a floating point value between `0.01` and `10.0`, where `1.0` applies no correction)
    /// indicates the amount of gamma correction to apply to the element. Gamma corrections modify the
    /// lightness of colors in a non-linear fashion, while not affecting white or black values. Gamma
    /// correction is typically used to modify the contrast of multiple elements. For example, you can
    /// modify the gamma to increase or decrease the contrast between the edges and interiors of elements.
    ///
    /// **Note**: This option adjusts the lightness relative to the default Google style, using a gamma
    /// curve. If Google makes any changes to the base map style, the changes affect your map's features
    /// styled with [`Self::Gamma`]. It's better to use the absolute [`Self::Color`] styler if you can.
    Gamma(f32),

    /// Inverts the existing lightness. This is useful, for example, for quickly switching to a darker
    /// map with white text.
    ///
    /// **Note**: This option simply inverts the default Google style. If Google makes any changes to
    /// the base map style, the changes affect your map's features styled with [`Self::InvertLightness`].
    /// It's better to use the absolute [`Self::Color`] styler if you can.
    InvertLightness,

    /// Sets the visibility which indicates whether and how the element appears on the map.
    ///
    /// A [`StyleRuleVisibility::Simplified`] visibility removes some style
    /// features from the affected features; roads, for example, are simplified into thinner lines without
    /// outlines, while parks lose their label text but retain the label icon.
    Visibility(StyleRuleVisibility),

    /// Sets the color of the feature.
    Color(RgbColor),

    /// Sets the weight (an integer value, greater than or equal to zero) sets the weight of the feature,
    /// in pixels. Setting the weight to a high value may result in clipping near tile borders.
    Weight(u8),
}
