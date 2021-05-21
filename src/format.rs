use std::fmt;

pub const PNG: &Format = &Format::Png;
pub const PNG32: &Format = &Format::Png32;
pub const GIF: &Format = &Format::Gif;
pub const JPEG: &Format = &Format::Jpeg;
pub const JPEG_BASELINE: &Format = &Format::JpegBaseline;

#[derive(Clone, Copy)]
pub enum Format {
    Png,
    Png32,
    Gif,
    Jpeg,
    JpegBaseline,
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Format::Png => "png",
                Format::Png32 => "png32",
                Format::Gif => "gif",
                Format::Jpeg => "jpg",
                Format::JpegBaseline => "jpg-baseline",
            }
        )
    }
}
