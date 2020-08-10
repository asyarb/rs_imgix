/// Builder to represent the `ch` URL parameter. Begin constructing the
/// parameter by calling `build()`.
///
/// # Example
/// ```
/// use rs_imgix::{ImgixUrl, ImgixClientHints};
///
/// let url = ImgixUrl::build("https://foo.com")
///     .ch(ImgixClientHins::build().dpr().width())
///     .finish();
///
/// assert_eq!(url, "https://foo.com/?ch=dpr,width");
/// ```
#[derive(Clone, Debug, Default)]
pub struct ImgixClientHints {
    width: bool,
    dpr: bool,
    save_data: bool,
}

impl ToString for ImgixClientHints {
    fn to_string(&self) -> String {
        let mut opts = Vec::new();

        if self.width {
            opts.push("width");
        }
        if self.dpr {
            opts.push("dpr");
        }
        if self.save_data {
            opts.push("save-data");
        }

        opts.join(",")
    }
}

impl ImgixClientHints {
    /// Starts building the `ch` parameter. Returns an `ImgixClientHintsBuilder`
    /// to specify options to pass to `ch`.
    pub fn build() -> ImgixClientHintsBuilder {
        ImgixClientHintsBuilder {
            inner: Self::default(),
        }
    }
}

/// Builder for specifying `ch` parameter options.
#[derive(Debug)]
pub struct ImgixClientHintsBuilder {
    inner: ImgixClientHints,
}

impl ImgixClientHintsBuilder {
    /// Completes the construction of the `ch` parameter and returns the
    /// `ImgixClientHints` type.
    pub fn finish(&self) -> ImgixClientHints {
        self.inner.clone()
    }

    /// Overrides the imgix `w` parameter.
    pub fn width(&mut self) -> &mut Self {
        self.inner.width = true;
        self
    }

    /// Overrides the `dpr` parameter.
    pub fn dpr(&mut self) -> &mut Self {
        self.inner.dpr = true;
        self
    }

    /// Reduces image quality to `q=45` and may change the output format of the
    /// image.
    pub fn save_data(&mut self) -> &mut Self {
        self.inner.save_data = true;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let str_params = ImgixClientHints::build()
            .width()
            .dpr()
            .save_data()
            .finish()
            .to_string();

        assert_eq!(str_params, "width,dpr,save-data");
    }
}
