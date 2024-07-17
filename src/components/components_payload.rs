use super::{Figure, Name, Transform};

pub struct ComponentsPayload {
    pub name: Name,
    pub transform: Transform,
    pub figure: Option<Figure>,
}

impl ComponentsPayload {
    pub fn new(name: Name, transform: Transform, figure: Option<Figure>) -> ComponentsPayload {
        ComponentsPayload {
            name,
            transform,
            figure,
        }
    }

    pub fn from_name(name: Name) -> ComponentsPayload {
        ComponentsPayload {
            name,
            transform: Transform::default(),
            figure: None,
        }
    }
}
