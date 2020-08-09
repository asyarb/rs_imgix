/// Builder to represent the `auto` URL parameter. Begin constructing a new URL
/// by calling `build()`.
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
pub struct ImgixAuto {
    compress: bool,
    enhance: bool,
    format: bool,
    redeye: bool,
}

impl ToString for ImgixAuto {
    fn to_string(&self) -> String {
        let mut opts = Vec::new();

        if self.compress {
            opts.push("compress");
        }
        if self.enhance {
            opts.push("enhance");
        }
        if self.format {
            opts.push("format")
        }
        if self.redeye {
            opts.push("redeye");
        }

        opts.join(",")
    }
}

impl ImgixAuto {
    /// Starts building the `auto` parameter. Returns an `ImgixAutoBuilder` to
    /// specify options to pass to `auto`.
    pub fn build() -> ImgixAutoBuilder {
        ImgixAutoBuilder {
            inner: Self::default(),
        }
    }
}

/// Builder for specifying `auto` parameter options.
#[derive(Debug)]
pub struct ImgixAutoBuilder {
    inner: ImgixAuto,
}

impl ImgixAutoBuilder {
    /// Completes the construction of the `auto` parameter and returns the final
    /// `ImgixAuto` type.
    pub fn finish(&self) -> ImgixAuto {
        self.inner.clone()
    }

    /// When set, Imgix will apply best-effort techniques to reduce the size of
    /// the image.
    ///
    /// See [Imgix docs](https://docs.imgix.com/apis/url/auto/auto#compress) for
    /// more info.
    pub fn compress(&mut self) -> &mut Self {
        self.inner.compress = true;
        self
    }

    /// When set, the image is adjusted using the distrubution of higlights,
    /// midtones, and shadow across RGB channels.
    ///
    /// See [Imgix docs](https://docs.imgix.com/apis/url/auto/auto#enhance) for
    /// more info.
    pub fn enhance(&mut self) -> &mut Self {
        self.inner.enhance = true;
        self
    }

    /// When set, Imgix determines whether the image can be served in a better format by a process called automatic content negotiation.
    ///
    /// See [Imgix docs](https://docs.imgix.com/apis/url/auto/auto#format) for
    /// more info.
    pub fn format(&mut self) -> &mut Self {
        self.inner.format = true;
        self
    }

    /// When set, red-eye removal is applied to any detected faces.
    ///
    /// See [Imgix docs](https://docs.imgix.com/apis/url/auto/auto#redeye) for
    /// more info.
    pub fn redeye(&mut self) -> &mut Self {
        self.inner.redeye = true;
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
