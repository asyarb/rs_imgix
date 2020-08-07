pub enum ImgixFit {
    Clamp,
    Clip,
    Crop,
    FaceArea,
    Fill,
    FillMax,
    Max,
    Min,
    Scale,
}

impl ToString for ImgixFit {
    fn to_string(&self) -> String {
        match self {
            ImgixFit::Clamp => "clamp".into(),
            ImgixFit::Clip => "clip".into(),
            ImgixFit::Crop => "crop".into(),
            ImgixFit::FaceArea => "facearea".into(),
            ImgixFit::Fill => "fill".into(),
            ImgixFit::FillMax => "fillmax".into(),
            ImgixFit::Max => "max".into(),
            ImgixFit::Min => "min".into(),
            ImgixFit::Scale => "scale".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        assert_eq!("facearea", ImgixFit::FaceArea.to_string());
    }
}
