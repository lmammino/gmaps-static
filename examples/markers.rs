use gmaps_static::*;

fn main() {
    // Example from https://developers.google.com/maps/documentation/maps-static/start#Markers

    // https://maps.googleapis.com/maps/api/staticmap?
    // center=63.259591,-144.667969
    // &zoom=6
    // &size=400x400
    // &markers=color:blue%7Clabel:S%7C62.107733,-145.541936
    // &markers=size:tiny%7Ccolor:green%7CDelta+Junction,AK
    // &markers=size:mid%7Ccolor:0xFFFF00%7Clabel:C%7CTok,AK"
    // &key=YOUR_API_KEY
    let credentials =
        Credentials::try_from_default_env().unwrap_or_else(|_| String::from("YOUR_API_KEY").into());

    let map = Map::new(credentials, (400, 400).into())
        .center((63.259591, -144.667969).into())
        .zoom(ZOOM_6)
        .add_marker(marker::Marker::simple(
            RGB_BLUE,
            'S',
            (62.107733, -145.541936).into(),
        ))
        .add_marker(
            marker::Marker::new()
                .appearence(marker::Appearence::Styled(
                    marker::Style::new().size(marker::TINY).color(RGB_GREEN),
                ))
                .add_location("Delta Junction,AK".into()),
        )
        .add_marker(
            marker::Marker::new()
                .appearence(marker::Appearence::Styled(
                    marker::Style::new()
                        .size(marker::MID)
                        .label('C'.into())
                        .color((0xFF, 0xFF, 0x00).into()),
                ))
                .add_location("Tok,AK".into()),
        );

    println!("{}", map.url());
}
