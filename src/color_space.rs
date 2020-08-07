pub enum ImgixColorSpace {
    SRGB,
    AdobeRGB1998,
    TinySRGB,
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
