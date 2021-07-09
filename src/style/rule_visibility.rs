use std::fmt::{Display, Formatter, Result};

pub const STYLE_VISIBILITY_ON: StyleRuleVisibility = StyleRuleVisibility::On;
pub const STYLE_VISIBILITY_OFF: StyleRuleVisibility = StyleRuleVisibility::Off;
pub const STYLE_VISIBILITY_SIMPLIFIED: StyleRuleVisibility = StyleRuleVisibility::Simplified;

#[derive(Clone)]
pub enum StyleRuleVisibility {
    On,
    Off,
    Simplified,
}

impl Display for StyleRuleVisibility {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use StyleRuleVisibility::*;
        f.write_str(match self {
            On => "on",
            Off => "off",
            Simplified => "simplified",
        })
    }
}
