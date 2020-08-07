use qstring::QString;

// #[derive(Clone)]

// #[derive(Clone)]
// pub enum ImgixColorSpace {
//     SRGB,
//     AdobeRGB1998,
//     TinySRGB,
//     Strip,
// }

// #[derive(Builder, Clone)]

// #[derive(Builder, Clone)]

// #[derive(Builder, Clone)]
// pub struct ImgixClientHints {
//     pub width: bool,
//     pub dpr: bool,
//     // TODOeriaize as saveData?
//     pub save_data: bool,
// }

mod auto;
mod crop;
mod fit;
mod rect;

pub struct ImgixUrl {
    // crop: ImgixCrop,
// ch: ImgixClientHints,
// cs: ImgixClientHints,
}

impl ImgixUrl {
    pub fn build(url: &str) -> ImgixUrlBuilder {
        ImgixUrlBuilder {
            params: Vec::new(),
            url: url.into(),
        }
    }
}

pub struct ImgixUrlBuilder<'a> {
    params: Vec<(&'a str, String)>,
    url: String,
}

impl<'a> ImgixUrlBuilder<'a> {
    pub fn finish(&self) -> String {
        let qs = QString::new(self.params.to_owned());

        format!("{}/?{}", self.url, qs)
    }

    /// Quality
    pub fn q(&mut self, val: i32) -> &mut Self {
        self.params.push(("q", val.to_string()));
        self
    }
    /// Width
    pub fn w(&mut self, val: i32) -> &mut Self {
        self.params.push(("w", val.to_string()));
        self
    }
    /// Height
    pub fn h(&mut self, val: i32) -> &mut Self {
        self.params.push(("h", val.to_string()));
        self
    }
    pub fn dpr(&mut self, val: i32) -> &mut Self {
        self.params.push(("dpr", val.to_string()));
        self
    }
    pub fn bg(&mut self, val: &str) -> &mut Self {
        self.params.push(("bg", val.into()));
        self
    }
    pub fn blur(&mut self, val: i32) -> &mut Self {
        self.params.push(("blur", val.to_string()));
        self
    }
    pub fn faceindex(&mut self, val: i32) -> &mut Self {
        self.params.push(("faceindex", val.to_string()));
        self
    }
    pub fn facepad(&mut self, val: i32) -> &mut Self {
        self.params.push(("facepad", val.to_string()));
        self
    }

    /// Aspect Ratio
    pub fn ar(&mut self, w: i32, h: i32) -> &mut Self {
        self.params.push(("ar", format!("{}:{}", w, h)));
        self
    }
    /// Auto
    pub fn auto(&mut self, auto: auto::ImgixAuto) -> &mut Self {
        self.params.push(("auto", auto.to_string()));
        self
    }
    /// Rect
    pub fn rect(&mut self, rect: rect::ImgixRect) -> &mut Self {
        self.params.push(("rect", rect.to_string()));
        self
    }
    /// Fit
    pub fn fit(&mut self, fit: fit::ImgixFit) -> &mut Self {
        self.params.push(("fit", fit.to_string()));
        self
    }
    pub fn crop(&mut self, crop: crop::ImgixCrop) -> &mut Self {
        self.params.push(("crop", crop.to_string()));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let url = ImgixUrl::build("https://foo.com")
            .blur(40)
            .q(40)
            .w(300)
            .ar(9, 1)
            .finish();

        assert_eq!(url, "https://foo.com/?blur=40&q=40&w=300&ar=9:1")
    }
}
