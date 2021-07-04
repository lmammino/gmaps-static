use std::fmt;

pub const TOP: RelativePosition = RelativePosition::Top;
pub const BOTTOM: RelativePosition = RelativePosition::Bottom;
pub const LEFT: RelativePosition = RelativePosition::Left;
pub const RIGTH: RelativePosition = RelativePosition::Right;
pub const CENTER: RelativePosition = RelativePosition::Center;
pub const TOP_LEFT: RelativePosition = RelativePosition::TopLeft;
pub const TOP_RIGTH: RelativePosition = RelativePosition::TopRight;
pub const BOTTOM_LEFT: RelativePosition = RelativePosition::BottomLeft;
pub const BOTTOM_RIGHT: RelativePosition = RelativePosition::BottomRight;

#[derive(Clone)]
pub enum RelativePosition {
    Top,
    Bottom,
    Left,
    Right,
    Center,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl fmt::Display for RelativePosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use RelativePosition::*;
        write!(
            f,
            "{}",
            match self {
                Top => "top",
                Bottom => "bottom",
                Left => "left",
                Right => "right",
                Center => "center",
                TopLeft => "topleft",
                TopRight => "topright",
                BottomLeft => "bottomleft",
                BottomRight => "bottomright",
            }
        )
    }
}
