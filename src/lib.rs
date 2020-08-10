#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]

/*!
# Overview

`rs_imgix` is a small crate for constructing valid Imgix URL `String`s. It
utilizes the common Rust builder pattern to dynamically assign parameters that
are added to the URL.

In a nutshell, basic usage looks like this:

```rust
use rs_imgix::ImgixUrl;

fn main() {
    let url = ImgixUrl::build("https://example.com")
        .blur(40)
        .q(40)
        .w(300)
        .finish();

    assert_eq!(url, "https://example.com/?blur=40&q=40&w=300");
}
```

Note that the `ImgixUrl` builder currently does not prevent construction of
URLs that produce invalid behavior with Imgix's API. For example, it will not
prevent the usage of the `ar` (Aspect Ratio) parameter if `fit=crop` is not
specified.

For more information on Imgix's URL API, refer to their [documentation](https://docs.imgix.com/apis/url).
*/

use qstring::QString;

mod auto;
mod client_hints;
mod color_space;
mod crop;
mod fit;
mod rect;

pub use crate::auto::ImgixAuto;
pub use crate::client_hints::ImgixClientHints;
pub use crate::color_space::ImgixColorSpace;
pub use crate::crop::ImgixCrop;
pub use crate::fit::ImgixFit;
pub use crate::rect::{Direction, ImgixRect, X, Y};

/// A builder for Imgix URLs. Begin constructing a new URL by calling `build()`.
///
/// # Example
/// ```
/// use rs_imgix::ImgixUrl;
///
/// let url = ImgixUrl::build("https://foo.com").blur(20).finish();
/// assert_eq!(url, "https://foo.com/?blur=20");
/// ```
#[derive(Debug)]
pub struct ImgixUrl;

impl ImgixUrl {
    /// Starts building a Imgix URL. Returns an `ImgixUrlBuilder` to add
    /// additional query parameters to the URL.
    pub fn build(url: &str) -> ImgixUrlBuilder<'_> {
        ImgixUrlBuilder {
            params: Vec::new(),
            url: url.into(),
        }
    }
}

/// Builder for specifying URL parameters to add to the constructed URL.
#[derive(Debug)]
pub struct ImgixUrlBuilder<'a> {
    params: Vec<(&'a str, String)>,
    url: String,
}

impl<'a> ImgixUrlBuilder<'a> {
    /// Completes the construction of the URL and returns the final URL with
    /// query string parametrs.
    pub fn finish(&self) -> String {
        let qs = QString::new(self.params.to_owned());

        format!("{}/?{}", self.url, qs)
    }

    /// Controls the output quality of lossy file formats.
    /// Valid values are in the range of 0 - 100.
    ///
    /// See [Imgix docs](https://docs.imgix.com/apis/url/format/q) for more info.
    pub fn q(&mut self, val: i32) -> &mut Self {
        self.params.push(("q", val.to_string()));
        self
    }

    /// The width of the output image, interpreted as pixels. The resulting
    /// image will be `val` pixesl wide.
    ///
    /// See [Imgix docs](https://docs.imgix.com/apis/url/size/w) for more info.
    pub fn w(&mut self, val: i32) -> &mut Self {
        self.params.push(("w", val.to_string()));
        self
    }

    /// The height of the output image, interpreted as pixels. The resulting
    /// image will be `val` pixesl tall.
    ///
    /// See [Imgix docs](https://docs.imgix.com/apis/url/size/h) for more info.
    pub fn h(&mut self, val: i32) -> &mut Self {
        self.params.push(("h", val.to_string()));
        self
    }

    /// Controls the output density of your image, so you can serve images at
    /// the correct density for every
    ///
    /// See [Imgix docs](https://docs.imgix.com/apis/url/pixel-density/dpr) for
    /// more info.
    pub fn dpr(&mut self, val: i32) -> &mut Self {
        self.params.push(("dpr", val.to_string()));
        self
    }

    /// The bg parameter allows you to fill in any transparent areas in your
    /// image with a color of your choice.
    ///
    /// See [Imgix docs](https://docs.imgix.com/apis/url/fill/bg) for
    /// more info.
    pub fn bg(&mut self, val: &str) -> &mut Self {
        self.params.push(("bg", val.into()));
        self
    }

    /// Applies a Gaussian style blur to your image, smoothing out image noise.
    /// Valid values are in the range from 0 – 2000.
    ///
    /// See [Imgix docs](https://docs.imgix.com/apis/url/stylize/blur) for
    /// more info.
    pub fn blur(&mut self, val: i32) -> &mut Self {
        self.params.push(("blur", val.to_string()));
        self
    }

    /// The faceindex parameter selects a face on which to center an image when
    /// `fit=facearea`.
    ///
    /// Must be a positive integer from less than `N` where `N` is the total
    /// number of detected faces.
    ///
    /// See [Imgix docs](https://docs.imgix.com/apis/url/face-detection/faceindex) for more
    /// info.
    pub fn faceindex(&mut self, val: i32) -> &mut Self {
        self.params.push(("faceindex", val.to_string()));
        self
    }

    /// The facepad parameter defines how much padding to allow for each face
    /// when `fit=facearea`.
    ///
    /// See [Imgix docs](https://docs.imgix.com/apis/url/face-detection/facepad) for more info.
    pub fn facepad(&mut self, val: i32) -> &mut Self {
        self.params.push(("facepad", val.to_string()));
        self
    }

    /// Resizes and crops the original image to match a specified aspect ratio.
    /// This parameter will work only when `fit=crop` is set.
    ///
    /// See [Imgix docs](https://docs.imgix.com/apis/url/size/ar) for more info.
    pub fn ar(&mut self, w: i32, h: i32) -> &mut Self {
        self.params.push(("ar", format!("{}:{}", w, h)));
        self
    }

    /// The `auto` parameter helps you automate a baseline level of optimization
    /// across your entire image catalog.
    ///
    /// See [Imgix docs](https://docs.imgix.com/apis/url/auto/auto) for more
    /// info.
    pub fn auto(&mut self, auto: auto::ImgixAuto<'_>) -> &mut Self {
        self.params.push(("auto", auto.to_string()));
        self
    }

    /// The `rect` parameter selects a sub-region of the source image to use for
    /// processing before applying other resize operations.
    ///
    /// See [Imgix docs](https://docs.imgix.com/apis/url/size/rect) for more
    /// info.
    pub fn rect(&mut self, rect: rect::ImgixRect) -> &mut Self {
        self.params.push(("rect", rect.to_string()));
        self
    }

    /// The `fit` parameter controls how the output image is fit to its target
    /// dimensions after resizing, and how any background areas will be filled.
    ///
    /// See [Imgix docs](https://docs.imgix.com/apis/url/size/fit) for more
    /// info.
    pub fn fit(&mut self, fit: fit::ImgixFit) -> &mut Self {
        self.params.push(("fit", fit.to_string()));
        self
    }

    /// Crop mode controls how the image is aligned when `fit=crop` is set. The
    /// `w` and `h` parameters should also be set, so that the crop behavior is
    /// defined within specific image dimensions.
    ///
    /// See [Imgix docs](https://docs.imgix.com/apis/url/size/crop) for more
    /// info.
    pub fn crop(&mut self, crop: crop::ImgixCrop<'_>) -> &mut Self {
        self.params.push(("crop", crop.to_string()));
        self
    }

    /// The `cs` parameter specifies the color space of the output image.
    ///
    /// See [Imgix docs](https://docs.imgix.com/apis/url/format/cs) for more
    /// info.
    pub fn cs(&mut self, cs: color_space::ImgixColorSpace) -> &mut Self {
        self.params.push(("cs", cs.to_string()));
        self
    }

    /// The ch parameter opts in specific images to use [Client
    /// Hints](https://developers.google.com/web/updates/tags/clienthints),
    /// which provide automatic resource selection using browser headers.
    ///
    /// See [Imgix docs](https://docs.imgix.com/apis/url/format/ch) for more
    /// info.
    pub fn ch(&mut self, ch: client_hints::ImgixClientHints<'_>) -> &mut Self {
        self.params.push(("ch", ch.to_string()));
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
            .fit(ImgixFit::Crop)
            .ar(9, 1)
            .finish();

        assert_eq!(url, "https://foo.com/?blur=40&q=40&w=300&fit=crop&ar=9:1");
    }
}
