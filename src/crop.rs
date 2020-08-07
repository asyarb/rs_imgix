#[derive(Clone)]
pub struct ImgixCrop {
    top: bool,
    bottom: bool,
    left: bool,
    right: bool,
    faces: bool,
    focalpoint: bool,
    edges: bool,
    entropy: bool,
}

impl ToString for ImgixCrop {
    fn to_string(&self) -> String {
        let mut opts = Vec::new();

        if self.top {
            opts.push("top");
        }
        if self.bottom {
            opts.push("bottom");
        }
        if self.left {
            opts.push("left");
        }
        if self.right {
            opts.push("right");
        }
        if self.faces {
            opts.push("faces");
        }
        if self.focalpoint {
            opts.push("focalpoint");
        }
        if self.edges {
            opts.push("edges");
        }
        if self.entropy {
            opts.push("entropy");
        }

        opts.join(",")
    }
}

impl Default for ImgixCrop {
    fn default() -> Self {
        Self {
            top: false,
            bottom: false,
            left: false,
            right: false,
            faces: false,
            focalpoint: false,
            edges: false,
            entropy: false,
        }
    }
}

impl ImgixCrop {
    pub fn build() -> ImgixCropBuilder {
        ImgixCropBuilder {
            inner: Self::default(),
        }
    }
}

pub struct ImgixCropBuilder {
    inner: ImgixCrop,
}

impl ImgixCropBuilder {
    pub fn finish(&self) -> ImgixCrop {
        self.inner.clone()
    }

    pub fn top(&mut self) -> &mut Self {
        self.inner.top = true;
        self
    }
    pub fn bottom(&mut self) -> &mut Self {
        self.inner.bottom = true;
        self
    }
    pub fn left(&mut self) -> &mut Self {
        self.inner.left = true;
        self
    }
    pub fn right(&mut self) -> &mut Self {
        self.inner.right = true;
        self
    }
    pub fn faces(&mut self) -> &mut Self {
        self.inner.faces = true;
        self
    }
    pub fn focalpoint(&mut self) -> &mut Self {
        self.inner.focalpoint = true;
        self
    }
    pub fn edges(&mut self) -> &mut Self {
        self.inner.edges = true;
        self
    }
    pub fn entropy(&mut self) -> &mut Self {
        self.inner.entropy = true;
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
