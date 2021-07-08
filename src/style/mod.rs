use crate::QueryStringable;
use std::fmt::{Display, Formatter, Result};

mod element;
mod feature;
mod rule;
mod rule_visibility;

pub use element::*;
pub use feature::*;
pub use rule::*;
pub use rule_visibility::*;

#[derive(Clone)]
pub struct Style {
    feature: Option<StyleFeature>,
    element: Option<StyleElement>,
    rules: Vec<StyleRule>,
}

impl Style {
    pub fn new() -> Self {
        Style {
            feature: None,
            element: None,
            rules: vec![],
        }
    }

    pub fn feature(mut self, feature: StyleFeature) -> Self {
        self.feature = Some(feature);
        self
    }

    pub fn element(mut self, element: StyleElement) -> Self {
        self.element = Some(element);
        self
    }

    pub fn rules(mut self, rules: Vec<StyleRule>) -> Self {
        self.rules = rules;
        self
    }

    pub fn add_rule(mut self, rule: StyleRule) -> Self {
        self.rules.push(rule);
        self
    }
}

impl Default for Style {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Style {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut parts: Vec<String> = vec![];

        if let Some(feature) = &self.feature {
            parts.push(format!("feature:{}", feature));
        }

        if let Some(element) = &self.element {
            parts.push(format!("element:{}", element));
        }

        for rule in &self.rules {
            parts.push(rule.to_string());
        }

        write!(f, "{}", parts.join("|"))
    }
}

impl QueryStringable for Style {
    fn as_query_params(&self) -> Vec<(String, String)> {
        vec![("style".to_string(), self.to_string())]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_builds_some_simple_styles() {
        let style = Style::new()
            .feature(StyleFeature::RoadLocal)
            .element(StyleElement::GeometryAll)
            .add_rule(StyleRule::Color((0, 255, 0).into()));
        let expected = "feature:road.local|element:geometry|color:0x00ff00";
        assert_eq!(expected, style.to_string());

        let style = Style::new()
            .feature(StyleFeature::LandscapeAll)
            .element(StyleElement::GeometryFill)
            .add_rule(StyleRule::Color((0, 0, 0).into()));
        let expected = "feature:landscape|element:geometry.fill|color:0x000000";
        assert_eq!(expected, style.to_string());

        let style = Style::new()
            .element(StyleElement::LabelsAll)
            .add_rule(StyleRule::InvertLightness(true));
        let expected = "element:labels|invert_lightness:true";
        assert_eq!(expected, style.to_string());

        let style = Style::new()
            .feature(StyleFeature::RoadArterial)
            .element(StyleElement::LabelsAll)
            .add_rule(StyleRule::InvertLightness(false));
        let expected = "feature:road.arterial|element:labels|invert_lightness:false";
        assert_eq!(expected, style.to_string());

        let style = Style::new()
            .feature(StyleFeature::RoadHighwayAll)
            .element(StyleElement::GeometryAll)
            .rules(vec![
                STYLE_VISIBILITY_SIMPLIFIED.into(),
                StyleRule::Color((0xc2, 0x80, 0xe9).into()),
            ]);
        let expected = "feature:road.highway|element:geometry|visibility:simplified|color:0xc280e9";
        assert_eq!(expected, style.to_string());

        let style = Style::new().feature(StyleFeature::TransitLine).rules(vec![
            STYLE_VISIBILITY_SIMPLIFIED.into(),
            StyleRule::Color((0xba, 0xba, 0xba).into()),
        ]);
        let expected = "feature:transit.line|visibility:simplified|color:0xbababa";
        assert_eq!(expected, style.to_string());

        let style = Style::new()
            .feature(StyleFeature::RoadHighwayAll)
            .element(StyleElement::LabelsTextStroke)
            .rules(vec![
                STYLE_VISIBILITY_ON.into(),
                StyleRule::Color((0xb0, 0x6e, 0xba).into()),
            ]);
        let expected =
            "feature:road.highway|element:labels.text.stroke|visibility:on|color:0xb06eba";
        assert_eq!(expected, style.to_string());

        let style = Style::new()
            .feature(StyleFeature::RoadHighwayAll)
            .element(StyleElement::LabelsTextFill)
            .rules(vec![
                STYLE_VISIBILITY_ON.into(),
                StyleRule::Color((0xff, 0xff, 0xff).into()),
            ]);
        let expected = "feature:road.highway|element:labels.text.fill|visibility:on|color:0xffffff";
        assert_eq!(expected, style.to_string());
    }
}
