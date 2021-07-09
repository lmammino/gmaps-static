use gmaps_static::*;

fn main() {
    let map = Builder::new("YOUR_API_KEY".into(), (400, 300).into())
        .scale(SCALE2)
        .center("Colosseo".into())
        .zoom(STREETS)
        .format(GIF)
        .maptype(HYBRID)
        .region("it".into())
        .language("it".into());

    println!("{}", map.make_url());
}
