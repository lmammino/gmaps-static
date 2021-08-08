use crate::QueryStringable;
use std::fmt::{Display, Formatter, Result};

mod element;
mod feature;
mod mapid;
mod rule;
mod rule_visibility;

pub use element::*;
pub use feature::*;
pub use mapid::*;
pub use rule::*;
pub use rule_visibility::*;

#[derive(Clone)]
pub struct Style {
    feature: Option<Feature>,
    element: Option<Element>,
    rules: Vec<Rule>,
}

impl Style {
    pub fn new() -> Self {
        Style {
            feature: None,
            element: None,
            rules: vec![],
        }
    }

    pub fn for_element(element: Element, rule: Rule) -> Self {
        Style {
            feature: None,
            element: Some(element),
            rules: vec![rule],
        }
    }

    pub fn for_feature(feature: Feature, rule: Rule) -> Self {
        Style {
            feature: Some(feature),
            element: None,
            rules: vec![rule],
        }
    }

    pub fn feature(mut self, feature: Feature) -> Self {
        self.feature = Some(feature);
        self
    }

    pub fn element(mut self, element: Element) -> Self {
        self.element = Some(element);
        self
    }

    pub fn rules(mut self, rules: Vec<Rule>) -> Self {
        self.rules = rules;
        self
    }

    pub fn add_rule(mut self, rule: Rule) -> Self {
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
    fn it_builds_some_styles() {
        let style = Style::new()
            .feature(Feature::RoadLocal)
            .element(Element::GeometryAll)
            .add_rule(Rule::Color((0, 255, 0).into()));
        let expected = "feature:road.local|element:geometry|color:0x00ff00";
        assert_eq!(expected, style.to_string());

        let style = Style::new()
            .feature(Feature::LandscapeAll)
            .element(Element::GeometryFill)
            .add_rule(Rule::Color((0, 0, 0).into()));
        let expected = "feature:landscape|element:geometry.fill|color:0x000000";
        assert_eq!(expected, style.to_string());

        let style = Style::for_element(Element::LabelsAll, Rule::InvertLightness(true));
        let expected = "element:labels|invert_lightness:true";
        assert_eq!(expected, style.to_string());

        let style = Style::new()
            .feature(Feature::RoadArterial)
            .element(Element::LabelsAll)
            .add_rule(Rule::InvertLightness(false));
        let expected = "feature:road.arterial|element:labels|invert_lightness:false";
        assert_eq!(expected, style.to_string());

        let style = Style::new()
            .feature(Feature::RoadHighwayAll)
            .element(Element::GeometryAll)
            .rules(vec![
                VISIBILITY_SIMPLIFIED.into(),
                Rule::Color((0xc2, 0x80, 0xe9).into()),
            ]);
        let expected = "feature:road.highway|element:geometry|visibility:simplified|color:0xc280e9";
        assert_eq!(expected, style.to_string());

        let style = Style::new().feature(Feature::TransitLine).rules(vec![
            VISIBILITY_SIMPLIFIED.into(),
            Rule::Color((0xba, 0xba, 0xba).into()),
        ]);
        let expected = "feature:transit.line|visibility:simplified|color:0xbababa";
        assert_eq!(expected, style.to_string());

        let style = Style::new()
            .feature(Feature::RoadHighwayAll)
            .element(Element::LabelsTextStroke)
            .rules(vec![
                VISIBILITY_ON.into(),
                Rule::Color((0xb0, 0x6e, 0xba).into()),
            ]);
        let expected =
            "feature:road.highway|element:labels.text.stroke|visibility:on|color:0xb06eba";
        assert_eq!(expected, style.to_string());

        let style = Style::new()
            .feature(Feature::RoadHighwayAll)
            .element(Element::LabelsTextFill)
            .rules(vec![
                VISIBILITY_ON.into(),
                Rule::Color((0xff, 0xff, 0xff).into()),
            ]);
        let expected = "feature:road.highway|element:labels.text.fill|visibility:on|color:0xffffff";
        assert_eq!(expected, style.to_string());

        let style = Style::for_feature(Feature::RoadAll, Rule::Color((255, 255, 255).into()));
        let expected = "feature:road|color:0xffffff";
        assert_eq!(expected, style.to_string());

        let style = Style::new()
            .feature(Feature::RoadLocal)
            .element(Element::LabelsAll)
            .add_rule(Rule::Color((0xff, 0xff, 0xff).into()));
        let expected = "feature:road.local|element:labels|color:0xffffff";
        assert_eq!(expected, style.to_string());

        let style = Style::new()
            .feature(Feature::RoadAll)
            .add_rule(Rule::Color((0xff, 0xff, 0xff).into()))
            .add_rule(VISIBILITY_SIMPLIFIED.into());
        let expected = "feature:road|color:0xffffff|visibility:simplified";
        assert_eq!(expected, style.to_string());
    }
}
