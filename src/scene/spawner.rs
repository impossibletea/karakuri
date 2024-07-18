use crate::{components::ComponentsPayload, Entity};

/// Spawner is used by entity behaviors to add or remove entities in the scene
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

#[cfg(test)]
mod tests {
    use crate::components::Name;

    use super::*;

    #[test]
    fn test_add_entity() {
        let mut spawner = Spawner::new();

        spawner.add_entity(ComponentsPayload::from_name(Name::new(String::from("Sonic"))));
        spawner.add_entity(ComponentsPayload::from_name(Name::new(String::from("Tails"))));

        assert_eq!(spawner.entities_to_add.len(), 2);
    }

    fn test_remove_entity() {
        let mut spawner = Spawner::new();

        spawner.remove_entity(5);
        spawner.remove_entity(7);

        assert_eq!(spawner.entities_to_remove.len(), 2);
    }
}
