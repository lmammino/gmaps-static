use gmaps_static::*;

fn main() {
    // Rebuilds URL from docs: https://developers.google.com/maps/documentation/maps-static/styling

    // https://maps.googleapis.com/maps/api/staticmap?
    // center=Brooklyn
    // &size=512x512
    // &zoom=15
    // &style=feature%3Aroad.local%7Celement%3Ageometry%7Ccolor%3A0x00ff00
    // &style=feature%3Alandscape%7Celement%3Ageometry.fill%7Ccolor%3A0x000000
    // &style=element%3Alabels%7Cinvert_lightness%3Atrue
    // &style=feature%3Aroad.arterial%7Celement%3Alabels%7Cinvert_lightness%3Afalse
    // &key=YOUR_API_KEY

    let credentials =
        Credentials::try_from_default_env().unwrap_or_else(|_| String::from("YOUR_API_KEY").into());

    let map = Map::new(credentials, (512, 512).into())
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

    println!("{}", map.url());
}
