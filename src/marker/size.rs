use std::fmt;
use std::str::FromStr;

pub const TINY: Size = Size::Tiny;
pub const MID: Size = Size::Mid;
pub const SMALL: Size = Size::Small;

#[derive(Clone)]
pub enum Size {
    Tiny,
    Mid,
    Small,
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Size::*;
        write!(
            f,
            "{}",
            match &self {
                Tiny => "tiny",
                Mid => "mid",
                Small => "small",
            }
        )
    }
}

impl FromStr for Size {
    type Err = String;

    fn from_str(input: &str) -> Result<Size, Self::Err> {
        use Size::*;
        match input {
            "tiny" => Ok(Tiny),
            "mid" => Ok(Mid),
            "small" => Ok(Small),
            x => Err(format!("Invalid value for MarkerSize. Given '{}'", x)),
        }
    }
}
