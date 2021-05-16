use std::str::FromStr;

#[derive(Clone)]
pub enum MarkerSize {
    Tiny,
    Mid,
    Small,
}

impl ToString for MarkerSize {
    fn to_string(&self) -> String {
        use MarkerSize::*;
        match &self {
            Tiny => String::from("tiny"),
            Mid => String::from("mid"),
            Small => String::from("small"),
        }
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
