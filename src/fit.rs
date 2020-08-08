#[derive(Debug)]
/// The `fit` parameter controls how the output image is fit to its target
/// dimensions after resizing, and how any background areas will be filled.
///
/// See [Imgix docs](https://docs.imgix.com/apis/url/size/fit) for more info.
pub enum ImgixFit {
    /// Resizes the image to fit within the width and height dimensions without
    /// cropping or distorting the image, and the remaining space is filled with
    /// extended pixels from the edge of the image.
    Clamp,

    /// **Default**. Resizes the image to fit within the width and height
    /// boundaries without cropping or distorting the image.
    Clip,

    /// Resizes the image to fill the width and height dimensions and crops any
    /// excess image data.
    Crop,

    /// Finds the area containing all faces, or a specific face in an image, and
    /// scales it to specified width and height dimensions. Can be used in
    /// conjunction with faceindex to identify a specific face, as well as
    /// facepad to include additional padded area around the face to zoom out
    /// from the immediate area around the faces.
    FaceArea,

    /// Resizes the image to fit within the requested width and height
    /// dimensions while preserving the original aspect ratio and without
    /// discarding any original image data.
    Fill,

    /// Resizes the image to fit within the requested width and height
    /// dimensions while preserving the original aspect ratio and without
    /// discarding any original image data.
    FillMax,

    /// Resizes the image to fit within the width and height dimensions without
    /// cropping or distorting the image, but will not increase the size of the
    /// image if it is smaller than the output size.
    Max,

    /// Resizes and crops the image to match the aspect ratio of the requested
    /// width and height. Will not exceed the original width and height of the
    /// image.
    Min,

    /// Scales the image to fit the constraining dimensions exactly.
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
