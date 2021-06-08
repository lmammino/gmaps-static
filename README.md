# GMapsStatic

Work in progress...


## Example

```rust
use GMapsStatic::*;

let map = UrlBuilder::new("YOUR_API_KEY".into(), (400, 300).into())
    .scale(SCALE2)
    .center("Colosseo".into())
    .zoom(STREETS)
    .format(GIF)
    .maptype(HYBRID)
    .region("it")
    .language("it");

println!("{}", map.make_url());
```

This will generate the following URL:

```plain
https://maps.googleapis.com/maps/api/staticmap?size=400x300&center=Colosseo&scale=2&format=gif&maptype=hybrid&language=it&region=it&key=YOUR_API_KEY
```

![A map of the area sorrounding the Coliseum generated with GMapsStatic](./images/coliseum.gif)
