use std::fmt::Display;

use crate::{Location, QueryStringable, RgbaColor};

#[derive(Clone)]
pub struct Path {
    weight: Option<u8>,
    color: Option<RgbaColor>,
    fill_color: Option<RgbaColor>,
    is_geodesic: bool,
    points: Vec<Location>,
}

impl Path {
    pub fn new() -> Self {
        Path {
            weight: None,
            color: None,
            fill_color: None,
            is_geodesic: false,
            points: vec![],
        }
    }

    pub fn weight(mut self, weight: u8) -> Self {
        self.weight = Some(weight);
        self
    }

    pub fn color(mut self, color: RgbaColor) -> Self {
        self.color = Some(color);
        self
    }

    pub fn fill_color(mut self, fill_color: RgbaColor) -> Self {
        self.fill_color = Some(fill_color);
        self
    }

    pub fn geodesic(mut self) -> Self {
        self.is_geodesic = true;
        self
    }

    pub fn points(mut self, points: Vec<Location>) -> Self {
        self.points = points;
        self
    }

    pub fn add_point(mut self, point: Location) -> Self {
        self.points.push(point);
        self
    }
}

impl Default for Path {
    fn default() -> Self {
        Path::new()
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parts: Vec<String> = vec![];

        if let Some(color) = &self.color {
            parts.push(format!("color:{}", color.to_string()));
        }

        if let Some(weight) = &self.weight {
            parts.push(format!("weight:{}", weight));
        }

        if let Some(fill_color) = &self.fill_color {
            parts.push(format!("fillcolor:{}", fill_color.to_string()));
        }

        if self.is_geodesic {
            parts.push(String::from("geodesic:true"));
        }

        for point in &self.points {
            parts.push(point.to_string());
        }

        write!(f, "{}", parts.join("|"))
    }
}

impl QueryStringable for Path {
    fn as_query_params(&self) -> Vec<(String, String)> {
        vec![(String::from("path"), self.to_string())]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RGBA_TRANSPARENT;

    #[test]
    fn it_creates_a_sample_path() {
        let path = Path::new()
            .color((0, 0, 255, 255).into())
            .weight(5_u8)
            .add_point((40.737102, -73.990318).into())
            .add_point((40.749825, -73.987963).into())
            .add_point((40.752946, -73.987384).into())
            .add_point((40.755823, -73.986397).into());

        let expected = "color:0x0000ffff|weight:5|40.737102,-73.990318|40.749825,-73.987963|40.752946,-73.987384|40.755823,-73.986397";

        assert_eq!(expected, path.to_string());
    }

    #[test]
    fn it_creates_a_path_with_address_location_and_bg_color() {
        let path = Path::new()
            .color(RGBA_TRANSPARENT)
            .weight(5_u8)
            .fill_color(RgbaColor::new(255, 255, 0, 51))
            .geodesic()
            .add_point("8th Avenue & 34th St,New York,NY".into())
            .add_point("8th Avenue & 42nd St,New York,NY".into())
            .add_point("Park Ave & 42nd St,New York,NY,NY".into())
            .add_point("Park Ave & 34th St,New York,NY,NY".into());

        let expected =
            "color:0x00000000|weight:5|fillcolor:0xffff0033|geodesic:true|8th Avenue & 34th St,New York,NY|8th Avenue & 42nd St,New York,NY|Park Ave & 42nd St,New York,NY,NY|Park Ave & 34th St,New York,NY,NY";

        assert_eq!(expected, path.to_string());
    }

    #[test]
    fn it_uses_default_constructor() {
        let path = Path::default()
            .color((0, 0, 255, 255).into())
            .weight(2_u8)
            .add_point((40.737102, -73.990318).into());

        let expected = "color:0x0000ffff|weight:2|40.737102,-73.990318";

        assert_eq!(expected, path.to_string());
    }

    #[test]
    fn it_is_querystringable() {
        let path = Path::default()
            .color((0, 0, 255, 255).into())
            .weight(2_u8)
            .add_point((40.737102, -73.990318).into());

        let expected = (
            String::from("path"),
            String::from("color:0x0000ffff|weight:2|40.737102,-73.990318"),
        );

        assert_eq!(expected, path.as_query_params()[0]);
    }
}
