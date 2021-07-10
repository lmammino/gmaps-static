use gmaps_static::*;

fn main() {
    // Rebuilds URL from docs: https://developers.google.com/maps/documentation/maps-static/styling
    let credentials = Credentials::try_from_env("API_KEY", "SECRET_KEY")
        .unwrap_or_else(|_| String::from("YOUR_API_KEY").into());

    let map = Builder::new(credentials, (512, 512).into())
        .zoom(ZOOM_15)
        .center("Brooklyn".into())
        .add_style(
            style::Style::new()
                .feature(style::Feature::RoadLocal)
                .element(style::Element::GeometryAll)
                .add_rule(style::Rule::Color((0x00, 0xff, 0x00).into())),
        )
        .add_style(
            style::Style::new()
                .feature(style::Feature::LandscapeAll)
                .element(style::Element::GeometryFill)
                .add_rule(style::Rule::Color((0x00, 0x00, 0x00).into())),
        )
        .add_style(style::Style::for_element(
            style::Element::LabelsAll,
            style::Rule::InvertLightness(true),
        ))
        .add_style(
            style::Style::new()
                .feature(style::Feature::RoadArterial)
                .element(style::Element::LabelsAll)
                .add_rule(style::Rule::InvertLightness(false)),
        );

    println!("{}", map.make_url());
}
