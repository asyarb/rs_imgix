use qstring::QString;

// #[derive(Clone)]
// pub enum ImgixFit {
//     Clamp,
//     Clip,
//     Crop,
//     FaceArea,
//     Fill,
//     FillMax,
//     Max,
//     Min,
//     Scale,
// }

// #[derive(Clone)]
// pub enum ImgixColorSpace {
//     SRGB,
//     AdobeRGB1998,
//     TinySRGB,
//     Strip,
// }

// #[derive(Builder, Clone)]

// #[derive(Builder, Clone)]
// pub struct ImgixCrop {
//     pub top: bool,
//     pub bottom: bool,
//     pub left: bool,
//     pub right: bool,
//     pub faces: bool,
//     pub focalpoint: bool,
//     pub edges: bool,
//     pub entropy: bool,
// }

// #[derive(Builder, Clone)]
// pub struct ImgixClientHints {
//     pub width: bool,
//     pub dpr: bool,
//     // TODOeriaize as saveData?
//     pub save_data: bool,
// }

mod auto;
mod rect;

pub struct ImgixUrl {
    // rect: ImgixRect,
// fit: ImgixFit,
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
        self.params.push(("auto", auto.into()));
        self
    }
    pub fn rect(&mut self, rect: rect::ImgixRect) -> &mut Self {
        self.params.push(("rect", rect.into()));
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
