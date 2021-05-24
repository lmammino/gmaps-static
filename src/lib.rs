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
pub use size::*;
pub use zoom::*;

use url::Url;

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

        // TODO: add markers

        url.query_pairs_mut()
            .append_pair("key", self.credentials.api_key.as_ref());

        // TODO: if credentials has a secret_key, calculate and add signature

        url.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_builds_a_simple_url() {
        let map = UrlBuilder::new("some_api_key".into(), (50, 50).into());

        assert_eq!(
            map.make_url(),
            "https://maps.googleapis.com/maps/api/staticmap?size=50x50&key=some_api_key"
        );
    }

    #[test]
    fn it_builds_a_more_complete_url() {
        let map = UrlBuilder::new("some_api_key".into(), (400, 300).into())
            .scale(SCALE2)
            .center("Colosseo".into())
            .zoom(STREETS)
            .format(GIF)
            .maptype(HYBRID)
            .region("it")
            .language("it");

        assert_eq!(
            map.make_url(),
            "https://maps.googleapis.com/maps/api/staticmap?size=400x300&center=Colosseo&scale=2&format=gif&maptype=hybrid&language=it&region=it&key=some_api_key"
        );
    }
}
