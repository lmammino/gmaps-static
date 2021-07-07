mod center;
mod credentials;
mod format;
mod icon_anchor;
mod language;
mod location;
mod maptype;
mod marker;
mod marker_appearence;
mod marker_color;
mod marker_icon;
mod marker_label;
mod marker_scale;
mod marker_size;
mod marker_style;
mod path;
mod querystringable;
mod region;
mod relative_position;
mod rgb_color;
mod rgba_color;
mod scale;
mod signature;
mod size;
mod visible;
mod zoom;

pub use center::*;
pub use credentials::*;
pub use format::*;
pub use icon_anchor::*;
pub use language::*;
pub use location::*;
pub use maptype::*;
pub use marker::*;
pub use marker_appearence::*;
pub use marker_color::*;
pub use marker_icon::*;
pub use marker_label::*;
pub use marker_scale::*;
pub use marker_size::*;
pub use marker_style::*;
pub use path::*;
pub use querystringable::*;
pub use region::*;
pub use relative_position::*;
pub use rgb_color::*;
pub use rgba_color::*;
pub use scale::*;
pub use signature::*;
pub use size::*;
pub use visible::*;
pub use zoom::*;

use url::Url;

#[macro_use]
extern crate lazy_static;

pub trait AllStr: std::convert::AsRef<str> + std::clone::Clone {}

#[derive(Clone)]
pub struct UrlBuilder<S: AsRef<str> + Clone> {
    credentials: Credentials<S>,
    size: Size,
    center: Option<Center>,
    zoom: Option<Zoom>,
    scale: Option<Scale>,
    format: Option<Format>,
    maptype: Option<MapType>,
    language: Option<Language>,
    region: Option<Region>,
    markers: Vec<Marker<S>>,
    paths: Vec<Path>,
    visible: Vec<Visible>,
}

const BASE_URL: &str = "https://maps.googleapis.com/maps/api/staticmap";

impl<S: AsRef<str> + Clone> UrlBuilder<S> {
    pub fn new(credentials: Credentials<S>, size: Size) -> Self {
        UrlBuilder {
            credentials,
            size,
            center: None,
            zoom: None,
            scale: None,
            format: None,
            maptype: None,
            language: None,
            region: None,
            markers: vec![],
            paths: vec![],
            visible: vec![],
        }
    }

    pub fn center(mut self, center: Center) -> Self {
        self.center = Some(center);
        self
    }

    pub fn zoom(mut self, zoom: Zoom) -> Self {
        self.zoom = Some(zoom);
        self
    }

    pub fn scale(mut self, scale: Scale) -> Self {
        self.scale = Some(scale);
        self
    }

    pub fn format(mut self, format: Format) -> Self {
        self.format = Some(format);
        self
    }

    pub fn maptype(mut self, maptype: MapType) -> Self {
        self.maptype = Some(maptype);
        self
    }

    pub fn language(mut self, language: Language) -> Self {
        self.language = Some(language);
        self
    }

    pub fn region(mut self, region: Region) -> Self {
        self.region = Some(region);
        self
    }

    pub fn markers(mut self, markers: Vec<Marker<S>>) -> Self {
        self.markers = markers;
        self
        }
    }

    pub fn add_marker(mut self, marker: Marker<S>) -> Self {
        self.markers.push(marker);
        self
    }

    pub fn paths(mut self, paths: Vec<Path>) -> Self {
        self.paths = paths;
        self
    }

    pub fn add_path(mut self, path: Path) -> Self {
        self.paths.push(path);
        self
    }

    pub fn visible(mut self, visible_locations: Vec<Visible>) -> Self {
        self.visible = visible_locations;
        self
    }

    pub fn add_visible(mut self, visible_location: Visible) -> Self {
        self.visible.push(visible_location);
        self
    }

    pub fn make_url(&self) -> String {
        // TODO: make this method fallible and return an error if there's no (center+zoom) and no marker
        let mut url = Url::parse(BASE_URL).unwrap();
        {
            let mut pairs = url.query_pairs_mut();
            let parts: Vec<&dyn QueryStringable> = vec![
                &self.center,
                &self.markers,
                &self.size,
                &self.scale,
                &self.format,
                &self.maptype,
                &self.zoom,
                &self.language,
                &self.region,
                &self.paths,
                &self.visible,
            ];

            parts
                .iter()
                .flat_map(|p| p.as_query_params())
                .for_each(|(key, value)| {
                    pairs.append_pair(key.as_str(), value.as_str());
                });

            pairs.append_pair("key", self.credentials.api_key.as_ref());
        }

        // If credentials has a secret_key, calculate and appends the signature
        // This must be the last query string parameters to be added (all the others need to
        // calculate the signature)
        if let Some(secret) = &self.credentials.secret_key {
            let signature = sign(&url, secret.as_ref());
            url.query_pairs_mut()
                .append_pair("signature", signature.as_str());
        }

        url.to_string()
    }
}

#[cfg(test)]
mod tests {
    mod test_utils;

    use super::*;
    use test_utils::*;

    #[test]
    fn it_builds_a_simple_url() {
        let map = UrlBuilder::new("YOUR_API_KEY".into(), (50, 50).into());

        let generated_url = qs_from_url(map.make_url());
        let expected_url = qs_from_url(
            "https://maps.googleapis.com/maps/api/staticmap?size=50x50&key=YOUR_API_KEY"
                .to_string(),
        );
        assert_eq!(generated_url, expected_url);
    }

    #[test]
    fn it_builds_a_url_with_a_signature_if_secret_is_used() {
        let credentials =
            Credentials::with_secret_key("YOUR_API_KEY", "X8XXXxxxxxXwrIEQfguOVNGv2jY=");
        let map = UrlBuilder::new(credentials, (50, 50).into());

        let generated_url = qs_from_url(map.make_url());
        let expected_url = qs_from_url(
            "https://maps.googleapis.com/maps/api/staticmap?size=50x50&key=YOUR_API_KEY&signature=Ig1D2O-jLfIGKJaO7SWeWVvLwR4%3D"
                .to_string(),
        );
        assert_eq!(generated_url, expected_url);
    }

    #[test]
    fn it_builds_a_more_complete_url() {
        let map = UrlBuilder::new("YOUR_API_KEY".into(), (400, 300).into())
            .scale(SCALE2)
            .center("Colosseo".into())
            .zoom(STREETS)
            .format(GIF)
            .maptype(HYBRID)
            .region("it".into())
            .language("it".into());

        let generated_url = qs_from_url(map.make_url());
        let expected_url = qs_from_url(
            "https://maps.googleapis.com/maps/api/staticmap?\
            size=400x300\
            &center=Colosseo&\
            scale=2&\
            zoom=15&\
            format=gif&\
            maptype=hybrid&\
            language=it&\
            region=it&\
            key=YOUR_API_KEY"
                .to_string(),
        );
        assert_eq!(generated_url, expected_url);
    }

    #[test]
    fn it_builds_a_more_complete_url_2() {
        let marker1 = Marker::simple(RGB_BLUE, 'S', (40.702147, -74.015794).into());
        let marker2 = Marker::simple(RGB_GREEN, 'G', (40.711614, -74.012318).into());
        let marker3 = Marker::simple(RGB_RED, 'C', (40.718217, -73.998284).into());

        let path = Path::default()
            .color((0, 0, 255, 255).into())
            .weight(2_u8)
            .add_point((40.737102, -73.990318).into());

        let map = UrlBuilder::new("YOUR_API_KEY".into(), (600, 300).into())
            .center("Brooklyn Bridge,New York,NY".into())
            .zoom(ZOOM_13)
            .maptype(ROADMAP)
            .add_marker(marker1)
            .add_marker(marker2)
            .add_marker(marker3)
            .add_path(path)
            .add_visible("Dumbo Brooklyn, NY 11201".into());

        let generated_url = qs_from_url(map.make_url());
        let expected_url = qs_from_url(
            "https://maps.googleapis.com/maps/api/staticmap?\
        center=Brooklyn+Bridge%2CNew+York%2CNY\
        &zoom=13\
        &size=600x300\
        &maptype=roadmap\
        &markers=color%3Ablue%7Clabel%3AS%7C40.702147%2C-74.015794\
        &markers=color%3Agreen%7Clabel%3AG%7C40.711614%2C-74.012318\
        &markers=color%3Ared%7Clabel%3AC%7C40.718217%2C-73.998284\
        &path=color%3A0x0000ffff%7Cweight%3A2%7C40.737102%2C-73.990318\
        &visible=Dumbo+Brooklyn%2C+NY+11201\
        &key=YOUR_API_KEY"
                .to_string(),
        );

        assert_eq!(generated_url, expected_url);
    }
}
