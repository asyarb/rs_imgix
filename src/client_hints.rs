/// Builder to represent the `ch` URL parameter. Begin constructing the
/// parameter by calling `build()`.
///
/// # Example
/// ```
/// use rs_imgix::{ImgixUrl, ImgixClientHints};
///
/// let url = ImgixUrl::build("https://foo.com")
///     .ch(ImgixClientHints::build().dpr().width().finish())
///     .finish();
///
/// assert_eq!(url, "https://foo.com/?ch=dpr,width");
/// ```
#[derive(Clone, Debug, Default)]
pub struct ImgixClientHints<'a> {
    opts: Vec<&'a str>,
}

impl<'a> ToString for ImgixClientHints<'a> {
    fn to_string(&self) -> String {
        self.opts.join(",")
    }
}

impl<'a> ImgixClientHints<'a> {
    /// Starts building the `ch` parameter. Returns an `ImgixClientHintsBuilder`
    /// to specify options to pass to `ch`.
    pub fn build() -> Self {
        Self::default()
    }

    /// Completes the construction of the `ch` parameter and returns the
    /// `ImgixClientHints` type.
    pub fn finish(&self) -> Self {
        self.clone()
    }

    /// Overrides the imgix `w` parameter.
    pub fn width(&mut self) -> &mut Self {
        self.opts.push("width");
        self
    }

    /// Overrides the `dpr` parameter.
    pub fn dpr(&mut self) -> &mut Self {
        self.opts.push("dpr");
        self
    }

    /// Reduces image quality to `q=45` and may change the output format of the
    /// image.
    pub fn save_data(&mut self) -> &mut Self {
        self.opts.push("save-data");
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
