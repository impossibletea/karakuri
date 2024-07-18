use super::{Behavior, Figure, Name, Transform};

pub struct ComponentsPayload {
    pub name: Name,
    pub transform: Transform,
    pub behavior: Option<Box<dyn Behavior>>,
    pub figure: Option<Figure>,
}

impl ComponentsPayload {
    pub fn new(
        name: Name,
        transform: Transform,
        behavior: Option<Box<dyn Behavior>>,
        figure: Option<Figure>,
    ) -> ComponentsPayload {
        ComponentsPayload {
            name,
            transform,
            behavior,
            figure,
        }
    }

    pub fn from_name(name: Name) -> ComponentsPayload {
        ComponentsPayload {
            name,
            transform: Transform::default(),
            behavior: None,
            figure: None,
        }
    }
}
