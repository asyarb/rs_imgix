/// Builder to represent the `auto` URL parameter. Begin constructing the
/// parameter by calling `build()`.
///
/// # Example
/// ```
/// use rs_imgix::{ImgixUrl, ImgixAuto};
///
/// let url = ImgixUrl::build("https://foo.com")
///     .auto(ImgixAuto::build().compress().redeye().finish())
///     .finish();
///
/// assert_eq!(url, "https://foo.com/?auto=compress,redeye");
/// ```
#[derive(Clone, Debug, Default)]
pub struct ImgixAuto<'a> {
    opts: Vec<&'a str>,
}

impl<'a> ToString for ImgixAuto<'a> {
    fn to_string(&self) -> String {
        self.opts.join(",")
    }
}

impl<'a> ImgixAuto<'a> {
    /// Starts building the `auto` parameter. Returns an `ImgixAutoBuilder` to
    /// specify options to pass to `auto`.
    pub fn build() -> Self {
        Self::default()
    }

    /// Completes the construction of the `auto` parameter and returns the
    /// `ImgixAutoType` type.
    pub fn finish(&self) -> Self {
        self.clone()
    }

    /// When set, Imgix will apply best-effort techniques to reduce the size of
    /// the image.
    ///
    /// See [Imgix docs](https://docs.imgix.com/apis/url/auto/auto#compress) for
    /// more info.
    pub fn compress(&mut self) -> &mut Self {
        self.opts.push("compress");
        self
    }

    /// When set, the image is adjusted using the distrubution of higlights,
    /// midtones, and shadow across RGB channels.
    ///
    /// See [Imgix docs](https://docs.imgix.com/apis/url/auto/auto#enhance) for
    /// more info.
    pub fn enhance(&mut self) -> &mut Self {
        self.opts.push("enhance");
        self
    }

    /// When set, Imgix determines whether the image can be served in a better format by a process called automatic content negotiation.
    ///
    /// See [Imgix docs](https://docs.imgix.com/apis/url/auto/auto#format) for
    /// more info.
    pub fn format(&mut self) -> &mut Self {
        self.opts.push("format");
        self
    }

    /// When set, red-eye removal is applied to any detected faces.
    ///
    /// See [Imgix docs](https://docs.imgix.com/apis/url/auto/auto#redeye) for
    /// more info.
    pub fn redeye(&mut self) -> &mut Self {
        self.opts.push("redeye");
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let str_params = ImgixAuto::build()
            .compress()
            .enhance()
            .format()
            .finish()
            .to_string();

        assert_eq!(str_params, "compress,enhance,format");
    }
}
