#[derive(Clone)]
pub struct ImgixAuto {
    compress: bool,
    enhance: bool,
    format: bool,
    redeye: bool,
}

pub struct ImgixAutoBuilder {
    inner: ImgixAuto,
}

impl ImgixAutoBuilder {
    pub fn finish(&self) -> ImgixAuto {
        self.inner.clone()
    }

    pub fn compress(&mut self, val: bool) -> &mut Self {
        self.inner.compress = val;
        self
    }
    pub fn enhance(&mut self, val: bool) -> &mut Self {
        self.inner.enhance = val;
        self
    }
    pub fn format(&mut self, val: bool) -> &mut Self {
        self.inner.format = val;
        self
    }
    pub fn redeye(&mut self, val: bool) -> &mut Self {
        self.inner.redeye = val;
        self
    }
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

impl Default for ImgixAuto {
    fn default() -> Self {
        Self {
            compress: false,
            enhance: false,
            format: false,
            redeye: false,
        }
    }
}

impl ImgixAuto {
    pub fn build() -> ImgixAutoBuilder {
        ImgixAutoBuilder {
            inner: Self::default(),
        }
    }
}
