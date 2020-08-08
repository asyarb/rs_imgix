#[derive(Debug)]
pub enum X {
    Left,
    Center,
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

#[derive(Debug)]
pub enum Y {
    Top,
    Middle,
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

#[derive(Debug)]
pub enum Direction {
    Number(i32),
    X(X),
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

pub struct ImgixRect {
    pub x: Direction,
    pub y: Direction,
    pub w: i32,
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
