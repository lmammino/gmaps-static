pub const STYLE_VISIBILITY_ON: StyleRuleVisibility = StyleRuleVisibility::On;
pub const STYLE_VISIBILITY_OFF: StyleRuleVisibility = StyleRuleVisibility::Off;
pub const STYLE_VISIBILITY_SIMPLIFIED: StyleRuleVisibility = StyleRuleVisibility::Simplified;

#[derive(Clone)]
pub enum StyleRuleVisibility {
    On,
    Off,
    Simplified,
}
