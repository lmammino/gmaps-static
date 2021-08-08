use gmaps_static::*;

fn main() {
    // https://maps.googleapis.com/maps/api/staticmap?
    // center=Colosseo
    // &size=400x300
    // &scale=2
    // &format=gif
    // &maptype=hybrid
    // &zoom=15
    // &language=it
    // &region=it
    // &key=YOUR_API_KEY

    let credentials =
        Credentials::try_from_default_env().unwrap_or_else(|_| String::from("YOUR_API_KEY").into());

    let map = Map::new(credentials, (400, 300).into())
        .scale(SCALE2)
        .center("Colosseo".into())
        .zoom(STREETS)
        .format(GIF)
        .maptype(HYBRID)
        .region("it".into())
        .language("it".into());

    println!("{}", map.url());
}
