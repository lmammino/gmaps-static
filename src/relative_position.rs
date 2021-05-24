use std::fmt;

pub static TOP: RelativePosition = RelativePosition::Top;
pub static BOTTOM: RelativePosition = RelativePosition::Bottom;
pub static LEFT: RelativePosition = RelativePosition::Left;
pub static RIGTH: RelativePosition = RelativePosition::Right;
pub static CENTER: RelativePosition = RelativePosition::Center;
pub static TOP_LEFT: RelativePosition = RelativePosition::TopLeft;
pub static TOP_RIGTH: RelativePosition = RelativePosition::TopRight;
pub static BOTTOM_LEFT: RelativePosition = RelativePosition::BottomLeft;
pub static BOTTOM_RIGHT: RelativePosition = RelativePosition::BottomRight;

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
