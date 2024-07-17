use crate::{math::Vector2, utils::Color};

// TODO: this probably should be deleted when sprites are implemented
pub struct Figure {
    pub color: Color,
    pub size: Vector2,
}

impl Figure {
    pub fn new(color: Color, size: Vector2) -> Figure {
        Figure { color, size }
    }
}
