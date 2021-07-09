use std::fmt;

pub const TOP: Position = Position::Top;
pub const BOTTOM: Position = Position::Bottom;
pub const LEFT: Position = Position::Left;
pub const RIGTH: Position = Position::Right;
pub const CENTER: Position = Position::Center;
pub const TOP_LEFT: Position = Position::TopLeft;
pub const TOP_RIGHT: Position = Position::TopRight;
pub const BOTTOM_LEFT: Position = Position::BottomLeft;
pub const BOTTOM_RIGHT: Position = Position::BottomRight;

#[derive(Clone)]
pub enum Position {
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

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Position::*;
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
