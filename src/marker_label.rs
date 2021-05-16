use std::ops::RangeInclusive;

#[derive(Clone)]
pub struct MarkerLabel(char);

impl MarkerLabel {
    pub fn new(label: char) -> Result<Self, String> {
        if RangeInclusive::new('A', 'Z').contains(&label)
            || RangeInclusive::new('a', 'z').contains(&label)
            || RangeInclusive::new('0', '9').contains(&label)
        {
            return Ok(MarkerLabel(label.to_ascii_uppercase()));
        }

        Err(format!(
            "Invalid Label '{}'. Only a char matching [A-Z0-9] is accepted",
            label
        ))
    }
}
