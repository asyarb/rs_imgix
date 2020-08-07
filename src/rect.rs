pub enum RectDirection {
    Number(i32),
    Direction(String),
}

impl Into<String> for RectDirection {
    fn into(self) -> String {
        match self {
            RectDirection::Number(num) => num.to_string(),
            RectDirection::Direction(str) => str,
        }
    }
}

pub struct ImgixRect {
    pub x: RectDirection,
    pub y: RectDirection,
    pub w: i32,
    pub h: i32,
}

impl Into<String> for ImgixRect {
    fn into(self) -> String {
        vec![
            self.x.into(),
            self.y.into(),
            self.w.to_string(),
            self.h.to_string(),
        ]
        .join(",")
    }
}
