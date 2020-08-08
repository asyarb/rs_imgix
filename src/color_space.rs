/// The `cs` parameter specifies the color space of the output image.
///
/// See [Imgix docs](https://docs.imgix.com/apis/url/format/cs) for more
/// info.
#[derive(Debug)]
pub enum ImgixColorSpace {
    /// Sets the image to the sRGB color space, the Internet standard.
    SRGB,

    /// Sets the image to the Adobe RGB (1998) color space which provides
    /// accurate color reproduction from screen to print.
    AdobeRGB1998,

    /// Saves on file size by reducing the color space metadata. May cause a
    /// slight shift in color values.
    TinySRGB,

    /// Completely removes color space metadata for maximum size reduction.
    Strip,
}

impl ToString for ImgixColorSpace {
    fn to_string(&self) -> String {
        match self {
            ImgixColorSpace::SRGB => "srgb".into(),
            ImgixColorSpace::AdobeRGB1998 => "adobergb1998".into(),
            ImgixColorSpace::TinySRGB => "tinysrgb".into(),
            ImgixColorSpace::Strip => "strip".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        assert_eq!(ImgixColorSpace::SRGB.to_string(), "srgb");
    }
}
