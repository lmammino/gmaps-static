use std::fmt::{Display, Formatter, Result};

pub const VISIBILITY_ON: RuleVisibility = RuleVisibility::On;
pub const VISIBILITY_OFF: RuleVisibility = RuleVisibility::Off;
pub const VISIBILITY_SIMPLIFIED: RuleVisibility = RuleVisibility::Simplified;

#[derive(Clone)]
pub enum RuleVisibility {
    On,
    Off,
    Simplified,
}

impl Display for RuleVisibility {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use RuleVisibility::*;
        f.write_str(match self {
            On => "on",
            Off => "off",
            Simplified => "simplified",
        })
    }
}
