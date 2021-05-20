use std::fmt;
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Clone)]
pub struct MarkerLabel(char);

impl MarkerLabel {
    pub fn new(label: char) -> Result<Self, String> {
        let label = label.to_ascii_uppercase();
        if RangeInclusive::new('A', 'Z').contains(&label)
            || RangeInclusive::new('0', '9').contains(&label)
        {
            return Ok(MarkerLabel(label));
        }

        Err(format!(
            "Invalid Label '{}'. Only a char matching [A-Z0-9] is accepted",
            label
        ))
    }
}

impl FromStr for MarkerLabel {
    type Err = String;

    fn from_str(label: &str) -> Result<Self, Self::Err> {
        if label.len() != 1 {
            return Err(format!(
                "Invalid label '{}'. Only a string made of a char matching [A-Z0-9] is accepted",
                label
            ));
        }

        MarkerLabel::new(label.chars().next().unwrap())
    }
}

impl From<char> for MarkerLabel {
    fn from(c: char) -> Self {
        MarkerLabel::new(c).unwrap()
    }
}

impl fmt::Display for MarkerLabel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
