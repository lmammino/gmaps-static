# gmaps_static

[![Rust](https://github.com/lmammino/gmaps-static/actions/workflows/Rust.yml/badge.svg)](https://github.com/lmammino/gmaps-static/actions/workflows/Rust.yml)
[![codecov](https://codecov.io/gh/lmammino/gmaps-static/branch/main/graph/badge.svg?token=4CNbvgaDc1)](https://codecov.io/gh/lmammino/gmaps-static)
[![crates.io badge](https://img.shields.io/crates/v/gmaps-static.svg)](https://crates.io/crates/gmaps-static)
[![Documentation](https://docs.rs/gmaps-static/badge.svg)](https://docs.rs/gmaps-static)


Work in progress...

## Example

```rust
use gmaps_static::*;

let map = Map::new("YOUR_API_KEY".into(), (400, 300).into())
    .scale(SCALE2)
    .center("Colosseo".into())
    .zoom(STREETS)
    .format(GIF)
    .maptype(HYBRID)
    .region("it".into())
    .language("it".into());

println!("{}", map.url());
```

This will generate the following URL:

```plain
https://maps.googleapis.com/maps/api/staticmap?size=400x300&center=Colosseo&scale=2&format=gif&maptype=hybrid&language=it&region=it&key=YOUR_API_KEY
```

![A map of the area sorrounding the Coliseum generated with GMapsStatic](./images/coliseum.gif)


## Features

 - [x] center
 - [x] zoom
 - [x] size
 - [x] scale
 - [x] format
 - [x] maptype
 - [x] language
 - [x] region
 - [x] markers
 - [x] paths
   - [ ] [encoded polyline paths](https://developers.google.com/maps/documentation/utilities/polylinealgorithm)
 - [x] viewports (`visible` parameter)
 - [x] styled maps
   - [x] support for `map_id` parameter
   - [x] support for URL-based styles (`style` parameter)
 - [X] [Clientid authentication](https://developers.google.com/maps/premium/apikey/maps-static-apikey#generating_valid_signatures)


## TODO list pre-1.0.0

 - [x] Remove ambiguous type aliases in favour of wrapper types
 - [x] Remove immutability and implement `clone()` in builder struct
 - [ ] Better error management
 - [ ] Proper crate documentation
 - [x] Examples
 - [ ] More test (support for all examples in the official docs)


## Contributing

Everyone is very welcome to contribute to this project.
You can contribute just by submitting bugs or suggesting improvements by
[opening an issue on GitHub](https://github.com/lmammino/gmaps-static/issues).


## License

Licensed under [MIT License](LICENSE). ?? Luciano Mammino.