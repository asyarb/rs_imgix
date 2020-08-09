# rs-imgix

`rs-imgix` is a small crate for constructing valid Imgix URL `String`s. It
utilizes the builder pattern to dynamically assign parameters that
are added to the URL.

## Installation

In your `Cargo.toml`:

```toml
[dependencies]
rs_imgix = { git = "https://github.com/asyarb/rs_imgix" }
```

## Overview

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

For more information on Imgix's URL API, please refer to their
[documentation](https://docs.imgix.com/apis/url).

## Disclaimer

This is largely a learning project to gain familiarity with Rust and creating a
library crate. This crate is not intended for use in a production environment.
This library has no official affliation with Imgix.

## License

MIT.
