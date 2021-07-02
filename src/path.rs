use std::fmt::Display;

use crate::{Location, PathColor, QueryStringable};

#[derive(Clone)]
pub struct Path {
    weight: Option<u8>,
    color: Option<PathColor>,
    fill_color: Option<PathColor>,
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

    pub fn weight(&self, weight: u8) -> Self {
        Path {
            weight: Some(weight),
            ..self.clone()
        }
    }

    pub fn color(&self, color: PathColor) -> Self {
        Path {
            color: Some(color),
            ..self.clone()
        }
    }

    pub fn fill_color(&self, fill_color: PathColor) -> Self {
        Path {
            fill_color: Some(fill_color),
            ..self.clone()
        }
    }

    pub fn is_geodesic(&self) -> Self {
        Path {
            is_geodesic: true,
            ..self.clone()
        }
    }

    pub fn points(&self, points: Vec<Location>) -> Self {
        Path {
            points,
            ..self.clone()
        }
    }

    pub fn add_point(&self, point: Location) -> Self {
        let mut new_path = self.clone();
        new_path.points.push(point);
        new_path
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

        if let Some(weight) = &self.weight {
            parts.push(format!("weight:{}", weight));
        }

        if let Some(color) = &self.color {
            parts.push(format!("color:{}", color.to_string()));
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

    #[test]
    fn it_creates_a_path() {
        let path = Path::new()
            .color((0, 0, 255, 255).into())
            .weight(5_u8)
            .add_point((40.737102, -73.990318).into())
            .add_point((40.749825, -73.987963).into())
            .add_point((40.752946, -73.987384).into())
            .add_point((40.755823, -73.986397).into());

        let expected = "weight:5|color:0x0000ffff|40.737102,-73.990318|40.749825,-73.987963|40.752946,-73.987384|40.755823,-73.986397";

        assert_eq!(expected, path.to_string());
    }
}