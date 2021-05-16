# GMapsStatic

Work in progress...


## Example

```rust
use GMapsStatic::*;

let map = UrlBuilder::new("some_api_key".into(), (400, 300).into())
    .scale(2)
    .center("Colosseo".into())
    .zoom(Zoom::Streets())
    .format(Format::Gif)
    .maptype(MapType::Hybrid)
    .region("it")
    .language("it");

println!("{}", map.make_url());
```

This will generate the following URL:

```plain
https://maps.googleapis.com/maps/api/staticmap?size=400x300&center=Colosseo&scale=2&format=gif&maptype=hybrid&language=it&region=it&key=some_api_key
```

![A map of the area sorrounding the Coliseum generated with GMapsStatic](./images/coliseum.gif)
