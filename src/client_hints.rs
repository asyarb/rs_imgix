#[derive(Clone)]
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

impl Default for ImgixClientHints {
    fn default() -> Self {
        Self {
            width: false,
            dpr: false,
            save_data: false,
        }
    }
}

impl ImgixClientHints {
    pub fn build() -> ImgixClientHintsBuilder {
        ImgixClientHintsBuilder {
            inner: Self::default(),
        }
    }
}

pub struct ImgixClientHintsBuilder {
    inner: ImgixClientHints,
}

impl ImgixClientHintsBuilder {
    pub fn finish(&self) -> ImgixClientHints {
        self.inner.clone()
    }

    pub fn width(&mut self) -> &mut Self {
        self.inner.width = true;
        self
    }
    pub fn dpr(&mut self) -> &mut Self {
        self.inner.dpr = true;
        self
    }
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
