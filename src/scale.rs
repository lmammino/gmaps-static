use crate::QueryStringable;
use std::fmt;

pub const SCALE1: Scale = Scale::S1;
pub const SCALE2: Scale = Scale::S2;

#[derive(Clone)]
pub enum Scale {
    S1,
    S2,
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Scale::*;
        write!(
            f,
            "{}",
            match self {
                S1 => "1",
                S2 => "2",
            }
        )
    }
}

impl QueryStringable for Scale {
    fn as_query_params(&self) -> Vec<(String, String)> {
        vec![("scale".to_string(), self.to_string())]
    }
}
