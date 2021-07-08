use std::fmt::{Display, Formatter, Result};

/// Elements are subdivisions of a feature.
///
/// A road, for example, consists of the graphical line (the geometry) on the map,
/// and also the text denoting its name (a label)
#[derive(Clone)]
pub enum StyleElement {
    /// `all` (default) selects all elements of the specified feature.
    All,
    /// `geometry` selects all geometric elements of the specified feature.
    GeometryAll,
    /// `geometry.fill` selects only the fill of the feature's geometry.
    GeometryFill,
    /// `geometry.stroke` selects only the stroke of the feature's geometry.
    GeometryStroke,
    /// `labels` selects the textual labels associated with the specified feature.
    LabelsAll,
    /// `labels.icon` selects only the icon displayed within the feature's label.
    LabelsIcon,
    /// `labels.text` selects only the text of the label.
    LabelsTextAll,
    /// `labels.text.fill` selects only the fill of the label. The fill of a label is
    /// typically rendered as a colored outline that surrounds the label text.
    LabelsTextFill,
    /// `labels.text.stroke` selects only the stroke of the label's text.
    LabelsTextStroke,
}

impl Display for StyleElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use StyleElement::*;
        f.write_str(match self {
            All => "all",
            GeometryAll => "geometry",
            GeometryFill => "geometry.fill",
            GeometryStroke => "geometry.stroke",
            LabelsAll => "labels",
            LabelsIcon => "labels.icon",
            LabelsTextAll => "labels.text",
            LabelsTextFill => "labels.text.fill",
            LabelsTextStroke => "labels.text.stroke",
        })
    }
}
