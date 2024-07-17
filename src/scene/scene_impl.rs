use crate::{
    components::{ComponentsPayload, Figure, Name, Transform},
    Entity,
};

// TODO: this should be configurable by user.
const MAX_ENTITIES: usize = 5000;

pub struct Scene {
    next_entity: Entity,
    free_entities: Vec<Entity>,

    entities: [Option<Entity>; MAX_ENTITIES],
    name_components: [Option<Name>; MAX_ENTITIES],
    transform_components: [Option<Transform>; MAX_ENTITIES],
    figure_components: [Option<Figure>; MAX_ENTITIES],

    entities_to_add: Vec<ComponentsPayload>,
    entities_to_remove: Vec<Entity>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            next_entity: 0,
            free_entities: Vec::new(),

            entities: [(); MAX_ENTITIES].map(|_| None),
            name_components: [(); MAX_ENTITIES].map(|_| None),
            transform_components: [(); MAX_ENTITIES].map(|_| None),
            figure_components: [(); MAX_ENTITIES].map(|_| None),

            entities_to_add: Vec::new(),
            entities_to_remove: Vec::new(),
        }
    }
}
