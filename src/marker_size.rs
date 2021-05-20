use std::fmt;
use std::str::FromStr;

#[derive(Clone)]
pub enum MarkerSize {
    Tiny,
    Mid,
    Small,
}

impl fmt::Display for MarkerSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use MarkerSize::*;
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

impl FromStr for MarkerSize {
    type Err = String;

    fn from_str(input: &str) -> Result<MarkerSize, Self::Err> {
        use MarkerSize::*;
        match input {
            "tiny" => Ok(Tiny),
            "mid" => Ok(Mid),
            "small" => Ok(Small),
            x => Err(format!("Invalid value for MarkerSize. Given '{}'", x)),
        }
    }
}
