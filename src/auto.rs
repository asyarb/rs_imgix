#[derive(Clone)]
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

pub struct ImgixAutoBuilder {
    inner: ImgixAuto,
}

impl ImgixAutoBuilder {
    pub fn finish(&self) -> ImgixAuto {
        self.inner.clone()
    }

    pub fn compress(&mut self) -> &mut Self {
        self.inner.compress = true;
        self
    }
    pub fn enhance(&mut self) -> &mut Self {
        self.inner.enhance = true;
        self
    }
    pub fn format(&mut self) -> &mut Self {
        self.inner.format = true;
        self
    }
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
