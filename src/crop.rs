/// Crop mode controls how the image is aligned when `fit=crop` is set. The
/// `w` and `h` parameters should also be set, so that the crop behavior is
/// defined within specific image dimensions.
///
/// # Example
/// ```
/// use rs_imgix::{ImgixUrl, ImgixCrop, ImgixFit};
///
/// let url = ImgixUrl::build("https://foo.com")
///     .fit(ImgixFit::Crop)
///     .w(400)
///     .h(300)
///     .crop(ImgixCrop::build().top().entropy().finish())
///     .finish();
///
/// assert_eq!(url, "https://foo.com/?fit=crop&w=400&h=300&crop=top,entropy")
/// ```
#[derive(Clone, Debug, Default)]
pub struct ImgixCrop<'a> {
    opts: Vec<&'a str>,
}

impl<'a> ToString for ImgixCrop<'a> {
    fn to_string(&self) -> String {
        self.opts.join(",")
    }
}

impl<'a> ImgixCrop<'a> {
    /// Starts building the `crop` parameter. Returns an `ImgixCropBuilder` to
    /// specify options to pass to `crop`.
    pub fn build() -> Self {
        Self::default()
    }

    /// Completes the construction of the `crop` parameter and returns the final
    /// `ImgixCrop` type.
    pub fn finish(&self) -> Self {
        self.clone()
    }

    /// Crop from the top of he image, down.
    pub fn top(&mut self) -> &mut Self {
        self.opts.push("top");
        self
    }

    /// Crop from the bottom of the image, up.
    pub fn bottom(&mut self) -> &mut Self {
        self.opts.push("bottom");
        self
    }

    /// Crop from the left of the image, right.
    pub fn left(&mut self) -> &mut Self {
        self.opts.push("left");
        self
    }

    /// Crop from the right of the image, left.
    pub fn right(&mut self) -> &mut Self {
        self.opts.push("right");
        self
    }

    /// If faces are detected in the image, attempts to center the crop to them.
    /// Otherwise, will default to centered if no other values are provided.
    pub fn faces(&mut self) -> &mut Self {
        self.opts.push("faces");
        self
    }

    /// Designates that the Imgix URL can accept focal point parameters `fp-x`,
    /// `fp-y` and `fp-z` values.
    ///
    /// See [Imgix docs](https://docs.imgix.com/apis/url/focalpoint-crop) for
    /// more info on focal point cropping.
    pub fn focalpoint(&mut self) -> &mut Self {
        self.opts.push("focalpoint");
        self
    }

    /// Automatically finds and crops to an area of interest by performing edge
    /// detection, looking for objects in the image.
    pub fn edges(&mut self) -> &mut Self {
        self.opts.push("edges");
        self
    }

    /// Automatically finds and crops to an area of interest by looking for busy
    /// sections of the image.
    pub fn entropy(&mut self) -> &mut Self {
        self.opts.push("entropy");
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let str_params = ImgixCrop::build()
            .top()
            .bottom()
            .edges()
            .finish()
            .to_string();

        assert_eq!(str_params, "top,bottom,edges");
    }
}
