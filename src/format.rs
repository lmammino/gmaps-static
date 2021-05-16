#[derive(Clone, Copy)]
pub enum Format {
    Png,
    Png32,
    Gif,
    Jpeg,
    JpegBaseline,
}

impl ToString for Format {
    fn to_string(&self) -> String {
        use Format::*;
        match self {
            Png => String::from("png"),
            Png32 => String::from("png32"),
            Gif => String::from("gif"),
            Jpeg => String::from("jpg"),
            JpegBaseline => String::from("jpg-baseline"),
        }
    }
}
