mod color;
mod credentials;
mod format;
mod icon_anchor;
mod location;
mod maptype;
mod marker;
mod marker_appearence;
mod marker_icon;
mod marker_label;
mod marker_scale;
mod marker_size;
mod marker_style;
mod relative_position;
mod scale;
mod signature;
mod size;
mod zoom;

pub use color::*;
pub use credentials::*;
pub use format::*;
pub use icon_anchor::*;
pub use location::*;
pub use maptype::*;
pub use marker::*;
pub use marker_appearence::*;
pub use marker_icon::*;
pub use marker_label::*;
pub use marker_scale::*;
pub use marker_size::*;
pub use marker_style::*;
pub use relative_position::*;
pub use scale::*;
use signature::*;
pub use size::*;
pub use zoom::*;

use url::Url;

#[macro_use]
extern crate lazy_static;

#[derive(Clone)]
pub struct UrlBuilder<S: AsRef<str> + Clone> {
    credentials: Credentials<S>,
    size: Size,
    center: Option<Location>,
    zoom: Option<&'static Zoom>,
    scale: Option<&'static Scale>,
    format: Option<&'static Format>,
    maptype: Option<&'static MapType>,
    language: Option<S>,
    region: Option<S>,
    markers: Vec<Marker<S>>,
}

static BASE_URL: &str = "https://maps.googleapis.com/maps/api/staticmap";

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
        }
    }

    pub fn center(&self, center: Location) -> Self {
        UrlBuilder {
            center: Some(center),
            ..(*self).clone()
        }
    }

    pub fn zoom(&self, zoom: &'static Zoom) -> Self {
        UrlBuilder {
            zoom: Some(zoom),
            ..(*self).clone()
        }
    }

    pub fn scale(&self, scale: &'static Scale) -> Self {
        UrlBuilder {
            scale: Some(scale),
            ..(*self).clone()
        }
    }

    pub fn format(&self, format: &'static Format) -> Self {
        UrlBuilder {
            format: Some(format),
            ..(*self).clone()
        }
    }

    pub fn maptype(&self, maptype: &'static MapType) -> Self {
        UrlBuilder {
            maptype: Some(maptype),
            ..(*self).clone()
        }
    }

    pub fn language(&self, language: S) -> Self {
        UrlBuilder {
            language: Some(language),
            ..(*self).clone()
        }
    }

    pub fn region(&self, region: S) -> Self {
        UrlBuilder {
            region: Some(region),
            ..(*self).clone()
        }
    }

    pub fn markers(&self, markers: Vec<Marker<S>>) -> Self {
        UrlBuilder {
            markers,
            ..(*self).clone()
        }
    }

    pub fn add_marker(&self, marker: Marker<S>) -> Self {
        let mut new_markers = self.markers.clone();
        new_markers.push(marker);

        UrlBuilder {
            markers: new_markers,
            ..(*self).clone()
        }
    }

    pub fn make_url(&self) -> String {
        // TODO: make this method fallible and return an error if there's no (center+zoom) and no marker

        let mut url = Url::parse(BASE_URL).unwrap();
        url.query_pairs_mut()
            .append_pair("size", self.size.to_string().as_str());

        if let Some(center) = self.center.as_ref() {
            url.query_pairs_mut()
                .append_pair("center", center.to_string().as_str());
        }

        if let Some(scale) = &self.scale {
            url.query_pairs_mut()
                .append_pair("scale", scale.to_string().as_str());
        }

        if let Some(format) = self.format {
            url.query_pairs_mut()
                .append_pair("format", format.to_string().as_str());
        }

        if let Some(maptype) = self.maptype {
            url.query_pairs_mut()
                .append_pair("maptype", maptype.to_string().as_str());
        }

        if let Some(language) = self.language.as_ref() {
            url.query_pairs_mut()
                .append_pair("language", language.as_ref());
        }

        if let Some(region) = self.region.as_ref() {
            url.query_pairs_mut().append_pair("region", region.as_ref());
        }

        if let Some(zoom) = self.zoom {
            url.query_pairs_mut()
                .append_pair("zoom", zoom.to_string().as_ref());
        }

        for marker in &self.markers {
            url.query_pairs_mut()
                .append_pair("markers", marker.to_string().as_ref());
        }

        url.query_pairs_mut()
            .append_pair("key", self.credentials.api_key.as_ref());

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
            .region("it")
            .language("it");

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
        let marker1 = Marker::simple(&Color::Blue, 'S', (40.702147, -74.015794).into());
        let marker2 = Marker::simple(&Color::Green, 'G', (40.711614, -74.012318).into());
        let marker3 = Marker::simple(&Color::Red, 'C', (40.718217, -73.998284).into());

        let map = UrlBuilder::new("YOUR_API_KEY".into(), (600, 300).into())
            .center("Brooklyn Bridge,New York,NY".into())
            .zoom(&ZOOM_13)
            .maptype(&MapType::RoadMap)
            .add_marker(marker1)
            .add_marker(marker2)
            .add_marker(marker3);

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
        &key=YOUR_API_KEY"
                .to_string(),
        );

        assert_eq!(generated_url, expected_url);
    }
}
