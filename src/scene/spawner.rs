use crate::{components::ComponentsPayload, Entity};

pub struct Spawner {
    pub(crate) entities_to_add: Vec<ComponentsPayload>,
    pub(crate) entities_to_remove: Vec<Entity>,
}

impl Default for Spawner {
    fn default() -> Spawner {
        Spawner::new()
    }
}

impl Spawner {
    pub fn new() -> Spawner {
        Spawner {
            entities_to_add: Vec::new(),
            entities_to_remove: Vec::new(),
        }
    }

    pub fn add_entity(&mut self, components: ComponentsPayload) {
        self.entities_to_add.push(components);
    }

    pub fn remove_entity(&mut self, entity: Entity) {
        self.entities_to_remove.push(entity);
    }
}
