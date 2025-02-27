pub mod api_models;

#[derive(Clone, Debug, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(
        x: i32,
        y: i32,
    ) -> Self {
        Self { x, y }
    }
}
