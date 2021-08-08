use gmaps_static::*;

fn main() {
    // Example from https://developers.google.com/maps/documentation/maps-static/start#Markers

    // http://maps.googleapis.com/maps/api/staticmap?
    // size=600x400
    // &style=visibility:on
    // &style=feature:water%7Celement:geometry%7Cvisibility:on
    // &style=feature:landscape%7Celement:geometry%7Cvisibility:on
    // &markers=anchor:32,10%7Cicon:https://goo.gl/5y3S82%7CCanberra+ACT
    // &markers=anchor:topleft%7Cicon:http://tinyurl.com/jrhlvu6%7CMelbourne+VIC
    // &markers=anchor:topright%7Cicon:https://goo.gl/1oTJ9Y%7CSydney+NSW
    // &key=YOUR_API_KEY

    let credentials =
        Credentials::try_from_default_env().unwrap_or_else(|_| String::from("YOUR_API_KEY").into());

    let map = Map::new(credentials, (600, 400).into())
        .add_style(style::Style::new().add_rule(style::VISIBILITY_ON.into()))
        .add_style(
            style::Style::new()
                .feature(style::Feature::WaterAll)
                .element(style::Element::GeometryAll)
                .add_rule(style::VISIBILITY_ON.into()),
        )
        .add_style(
            style::Style::new()
                .feature(style::Feature::LandscapeAll)
                .element(style::Element::GeometryAll)
                .add_rule(style::VISIBILITY_ON.into()),
        )
        .add_marker(
            marker::Marker::new()
                .add_location("Canberra ACT".into())
                .appearence(
                    marker::Icon::new("https://goo.gl/5y3S82")
                        .anchor((32, 10).into())
                        .into(),
                ),
        )
        .add_marker(
            marker::Marker::new()
                .add_location("Melbourne VIC".into())
                .appearence(
                    marker::Icon::new("http://tinyurl.com/jrhlvu6")
                        .anchor(marker::TOP_LEFT.into())
                        .into(),
                ),
        )
        .add_marker(
            marker::Marker::new()
                .add_location("Sydney NSW".into())
                .appearence(
                    marker::Icon::new("https://goo.gl/1oTJ9Y")
                        .anchor(marker::TOP_RIGHT.into())
                        .into(),
                ),
        );

    println!("{}", map.url());
}
