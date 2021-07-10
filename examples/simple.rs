use gmaps_static::*;

fn main() {
    let credentials = Credentials::try_from_env("API_KEY", "SECRET_KEY")
        .unwrap_or_else(|_| String::from("YOUR_API_KEY").into());

    let map = Builder::new(credentials, (400, 300).into())
        .scale(SCALE2)
        .center("Colosseo".into())
        .zoom(STREETS)
        .format(GIF)
        .maptype(HYBRID)
        .region("it".into())
        .language("it".into());

    println!("{}", map.make_url());
}
