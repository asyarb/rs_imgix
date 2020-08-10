/// Represents a valid direction for the `x` option of `ImgixRect`.
#[derive(Debug)]
pub enum X {
    /// Equivalent to 0.
    Left,

    /// Equivalent to half of the difference of the image width minus the `rect`
    /// width.
    Center,

    /// Difference between the image width minus the `rect` width.
    Right,
}

impl ToString for X {
    fn to_string(&self) -> String {
        match self {
            X::Left => "left".into(),
            X::Right => "right".into(),
            X::Center => "center".into(),
        }
    }
}

/// Represents a valid direction for the `y` option of `ImgixRect`.
#[derive(Debug)]
pub enum Y {
    /// Equivalent to 0.
    Top,

    /// Equivalent to half the difference of the image height minus the `rect`
    /// height.
    Middle,

    /// Equivalent to the difference of the image height minus the `rect`
    /// height.
    Bottom,
}

impl ToString for Y {
    fn to_string(&self) -> String {
        match self {
            Y::Top => "top".into(),
            Y::Middle => "middle".into(),
            Y::Bottom => "bottom".into(),
        }
    }
}

/// Enum representing a valid directional value for `ImgixRect`. A direction can
/// be an integer or a valid `X` or `Y` value.
#[derive(Debug)]
pub enum Direction {
    /// Integer value.
    Number(i32),

    /// Valid X-Directional value.
    X(X),

    /// Valid Y-Direciton value.
    Y(Y),
}

impl ToString for Direction {
    fn to_string(&self) -> String {
        match self {
            Direction::Number(num) => num.to_string(),
            Direction::X(x) => x.to_string(),
            Direction::Y(y) => y.to_string(),
        }
    }
}

/// Struct to represent the `rect` URL parameter. Construct the
/// parameter by calling instantiating it plainly.
///
/// # Example
/// ```
/// use rs_imgix::{ImgixUrl, ImgixRect};
///
/// let rect = ImgixRect {
///     x: Direction::Number(300),
///     y: Direction::Y(Y::Bottom),
///     w: 100,
///     h: 50,
/// };
/// let url = ImgixUrl::build("https://foo.com")
///     .rect(rect)
///     .finish();
///
/// assert_eq!(url, "https://foo.com/?rect=300,bottom,100,50");
/// ```
#[derive(Debug)]
pub struct ImgixRect {
    /// Can take an integer or Left, Center, and Right options.
    pub x: Direction,

    /// Can take an integer or Top, Middle and Bottom options.
    pub y: Direction,

    /// The width of the bounding rect.
    pub w: i32,

    /// The height of the bounding rect.
    pub h: i32,
}

impl ToString for ImgixRect {
    fn to_string(&self) -> String {
        vec![
            self.x.to_string(),
            self.y.to_string(),
            self.w.to_string(),
            self.h.to_string(),
        ]
        .join(",")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let string_params: String = ImgixRect {
            x: Direction::Number(300),
            y: Direction::Y(Y::Bottom),
            w: 100,
            h: 50,
        }
        .to_string();

        assert_eq!(string_params, "300,bottom,100,50")
    }
}
